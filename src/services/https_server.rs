use crate::{apis::services, base::functions, config::RsbpError, services::client_service};
use lazy_static::lazy_static;
use mio::{
    self,
    net::{TcpListener, TcpStream},
};
use regex::{Regex, RegexBuilder};
use rustls::{self, NoClientAuth, Session};
use rustls_pemfile;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs,
    io::{self, BufReader, Read, Write},
    net,
    sync::Arc,
};

// Token for our listening socket.
const LISTENER: mio::Token = mio::Token(0);

static mut STOP: bool = false;

// Which mode the server operates in.
#[derive(Clone)]
enum ServerMode {
    /// Write back received bytes
    Echo,

    /// Do one read, then write a bodged HTTP response and
    /// cleanly close the connection.
    Http,
}

/// Simple HTTP request.
struct HttpRequest {
    pub verb: String,
    pub path: String,
    // pub header: String,
    pub body: String,
    pub origin: String,
    // pub content_length: i32,
}

/// Simple HTTP response.
struct HttpResponse {
    pub status_code: String,
    pub headers: HashMap<String, String>,
    pub content: String,
    // pub headers_and_content: Vec<u8>,
    // pub content_length: i32,
}

impl HttpResponse {
    pub fn headers_and_content(&mut self) -> String {
        // Geht nur mit String content, z.B. json!
        let mut s = String::new();
        s.push_str(format!("HTTP/1.1 {}\r\n", self.status_code).as_str());
        let cl = self.content.as_bytes().len();
        self.headers.insert("Content-length".into(), cl.to_string());
        for p in self.headers.iter() {
            s.push_str(format!("{}: {}\r\n", p.0, p.1).as_str());
        }
        s.push_str("\r\n"); // End of header
        s.push_str(self.content.as_str());
        println!("headers_and_content {}", s);
        s
    }
}

/// This binds together a TCP listening socket, some outstanding connections, and a TLS server configuration.
struct TlsServer {
    server: TcpListener,
    connections: HashMap<mio::Token, OpenConnection>,
    next_id: usize,
    tls_config: Arc<rustls::ServerConfig>,
    mode: ServerMode,
}

impl TlsServer {
    fn new(server: TcpListener, mode: ServerMode, cfg: Arc<rustls::ServerConfig>) -> Self {
        TlsServer {
            server,
            connections: HashMap::new(),
            next_id: 2,
            tls_config: cfg,
            mode,
        }
    }

    fn accept(&mut self, registry: &mio::Registry) -> Result<(), io::Error> {
        loop {
            match self.server.accept() {
                Ok((socket, addr)) => {
                    println!("Accepting new connection from {:?}", addr);
                    let tls_conn = rustls::ServerSession::new(&Arc::clone(&self.tls_config));
                    let mode = self.mode.clone();
                    let token = mio::Token(self.next_id);
                    self.next_id += 1;
                    let mut connection = OpenConnection::new(socket, token, mode, tls_conn);
                    connection.register(registry);
                    self.connections.insert(token, connection);
                }
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                Err(err) => {
                    println!(
                        "encountered error while accepting connection; err={:?}",
                        err
                    );
                    return Err(err);
                }
            }
        }
    }

    fn conn_event(&mut self, registry: &mio::Registry, event: &mio::event::Event) {
        let token = event.token();
        if self.connections.contains_key(&token) {
            self.connections
                .get_mut(&token)
                .unwrap()
                .ready(registry, event);

            if self.connections[&token].is_closed() {
                self.connections.remove(&token);
            }
        }
    }
}

fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .unwrap()
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect()
}

fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let keyfile = fs::File::open(filename).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);

    loop {
        match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file") {
            Some(rustls_pemfile::Item::RSAKey(key)) => return rustls::PrivateKey(key),
            Some(rustls_pemfile::Item::PKCS8Key(key)) => return rustls::PrivateKey(key),
            None => break,
            _ => {}
        }
    }

    panic!(
        "no keys found in {:?} (encrypted keys not supported)",
        filename
    );
}

fn make_config(/*args: &Args*/) -> Arc<rustls::ServerConfig> {
    let client_auth = NoClientAuth::new();
    let suites = rustls::ALL_CIPHERSUITES.to_vec();
    let certs = load_certs("/opt/Haushalt/CSBP/cert/cert.pem");
    let privkey = load_private_key("/opt/Haushalt/CSBP/cert/cert.key");
    let mut config = rustls::ServerConfig::with_ciphersuites(client_auth, &suites);
    config.set_single_cert(certs, privkey).unwrap();
    Arc::new(config)
}

/// This is a connection which has been accepted by the server, and is currently being served.
/// It has a TCP-level stream, a TLS-level connection state, and some other state/metadata.
struct OpenConnection {
    socket: TcpStream,
    token: mio::Token,
    closing: bool,
    closed: bool,
    mode: ServerMode,
    tls_conn: rustls::ServerSession,
}

impl OpenConnection {
    fn new(
        socket: TcpStream,
        token: mio::Token,
        mode: ServerMode,
        tls_conn: rustls::ServerSession,
    ) -> OpenConnection {
        OpenConnection {
            socket,
            token,
            closing: false,
            closed: false,
            mode,
            tls_conn,
        }
    }

    /// We're a connection, and we have something to do.
    fn ready(&mut self, registry: &mio::Registry, ev: &mio::event::Event) {
        // If we're readable: read some TLS. Then see if that yielded new plaintext.
        // Then see if the backend is readable too.
        if ev.is_readable() {
            self.do_tls_read();
            self.try_plain_read();
        }
        if ev.is_writable() {
            self.do_tls_write_and_handle_error();
        }
        if self.closing {
            let _ = self.socket.shutdown(net::Shutdown::Both);
            self.closed = true;
            self.deregister(registry);
        } else {
            self.reregister(registry);
        }
    }

    fn do_tls_read(&mut self) {
        // Read some TLS data.
        match self.tls_conn.read_tls(&mut self.socket) {
            Err(err) => {
                if let io::ErrorKind::WouldBlock = err.kind() {
                    return;
                }
                println!("read error {:?}", err);
                self.closing = true;
                return;
            }
            Ok(0) => {
                println!("eof");
                self.closing = true;
                return;
            }
            Ok(_) => {}
        };

        // Process newly-received TLS messages.
        if let Err(err) = self.tls_conn.process_new_packets() {
            println!("cannot process packet: {:?}", err);
            // last gasp write to send any alerts
            self.do_tls_write_and_handle_error();
            self.closing = true;
        }
    }

    fn try_plain_read(&mut self) {
        // Read and process all available plaintext.
        if self.tls_conn.process_new_packets().is_ok() {
            let mut buf = Vec::<u8>::new();
            if let Ok(len) = self.tls_conn.read_to_end(&mut buf) {
                if len > 0 {
                    self.incoming_plaintext(&buf);
                }
            }
        }
    }

    /// Process some amount of received plaintext.
    fn incoming_plaintext(&mut self, buf: &[u8]) {
        match self.mode {
            ServerMode::Echo => {
                println!(
                    "plaintext buffer {} {:?}",
                    buf.len(),
                    std::str::from_utf8(&buf).unwrap()
                );
                self.tls_conn.write_all(buf).unwrap();
            }
            ServerMode::Http => {
                let req = get_http_request(buf);
                let mut response = handle_http_request(req);
                // let resp = b"HTTP/1.0 200 OK\r\nConnection: close\r\n\r\nHello world from rustls tlsserver\r\n";
                self.tls_conn
                    .write_all(response.headers_and_content().as_bytes())
                    .unwrap();
                let _ = self.tls_conn.flush();
                //self.tls_conn.send_close_notify();
            }
        }
    }

    fn tls_write(&mut self) -> io::Result<usize> {
        self.tls_conn.write_tls(&mut self.socket)
    }

    fn do_tls_write_and_handle_error(&mut self) {
        let rc = self.tls_write();
        if rc.is_err() {
            println!("write failed {:?}", rc);
            self.closing = true;
            return;
        }
    }

    fn register(&mut self, registry: &mio::Registry) {
        let event_set = self.event_set();
        registry
            .register(&mut self.socket, self.token, event_set)
            .unwrap();
    }

    fn reregister(&mut self, registry: &mio::Registry) {
        let event_set = self.event_set();
        registry
            .reregister(&mut self.socket, self.token, event_set)
            .unwrap();
    }

    fn deregister(&mut self, registry: &mio::Registry) {
        registry.deregister(&mut self.socket).unwrap();
    }

    /// What IO events we're currently waiting for,
    /// based on wants_read/wants_write.
    fn event_set(&self) -> mio::Interest {
        let rd = self.tls_conn.wants_read();
        let wr = self.tls_conn.wants_write();

        if rd && wr {
            mio::Interest::READABLE | mio::Interest::WRITABLE
        } else if wr {
            mio::Interest::WRITABLE
        } else {
            mio::Interest::READABLE
        }
    }

    fn is_closed(&self) -> bool {
        self.closed
    }
}

/// Get HTTP request from buffer.
fn get_http_request(buf: &[u8]) -> Result<HttpRequest, RsbpError> {
    lazy_static! {
        static ref RE_ORIGIN: Regex = RegexBuilder::new("\r\nOrigin: *([^\r]+)(\r\n)?")
            .case_insensitive(true)
            .build()
            .unwrap();
        // static ref RE_CL: Regex = RegexBuilder::new("\r\nContent-Length: *([0-9]+)(\r\n)?")
        //     .case_insensitive(true)
        //     .build()
        //     .unwrap();
    }
    let req0 = std::str::from_utf8(&buf);
    if let Err(err) = req0 {
        return Err(RsbpError::error_string(
            format!("get_http_request {}", err).as_str(),
        ));
    }
    let req = req0.unwrap().to_string();
    println!("handle http request {} {:?}", buf.len(), req);
    if functions::mach_nichts() == 1 {
        return Err(RsbpError::error_string(
            format!("get_http_request {}", "Fehler").as_str(),
        ));
    }
    let mut verb = String::new();
    let mut path = String::new();
    let mut header = String::new();
    let mut body = String::new();
    let mut origin = String::new();
    // let mut content_length = -1;
    if let Some(i) = req.find("\r\n\r\n") {
        header = req[..i].to_string();
        body = req[i + 4..].to_string();
        if let Some(c) = RE_ORIGIN.captures(header.as_str()) {
            origin = c[1].to_string();
        }
        // if let Some(c) = RE_CL.captures(header.as_str()) {
        //     content_length = functions::to_i32(&c[1]);
        // }
    }
    if header.is_empty() {
        return Err(RsbpError::error_string(
            format!("get_http_request {}", "Bad request").as_str(),
        ));
    }
    if let Some(i) = header.find("\r\n") {
        let line = req[..i].to_string();
        let arr = line.split(' ');
        let vec = arr.collect::<Vec<&str>>();
        if let Some(v) = vec.get(0) {
            verb = v.to_string();
        }
        if let Some(p) = vec.get(1) {
            path = p.to_string();
        }
    }
    if verb.is_empty() || path.is_empty() {
        return Err(RsbpError::error_string(
            format!("get_http_request {}", "Bad request").as_str(),
        ));
    }
    Ok(HttpRequest {
        verb,
        path,
        // header,
        body,
        origin,
        // content_length,
    })
}

/// Handle a HTTP request.
fn handle_http_request(request: Result<HttpRequest, RsbpError>) -> HttpResponse {
    let daten = services::get_daten();
    let mut status_code = String::from("200 OK");
    let mut rh = HashMap::<String, String>::new();
    let mut content = String::new();
    let mut contenttype = "text/html; charset=utf-8";
    let mut options = false;

    use chrono::TimeZone;
    let dtl = chrono::Local.from_local_datetime(&daten.get_now()).unwrap();
    let utc = dtl.naive_utc();
    let f = utc.format("%a, %d %b %Y %H:%M:%S GMT").to_string(); // Sun, 19 Sep 2021 06:18:38 GMT
    rh.insert("Date".into(), f);
    let mut verb = String::new();
    let mut path = String::new();
    let mut origin = String::new();
    let mut body = String::new();
    if let Ok(ref req) = request {
        verb = req.verb.to_string();
        path = req.path.to_string();
        origin = req.origin.to_string();
        body = req.body.to_string();
    }
    let mut error = match request {
        Err(ref err) => format!("{}", err),
        _ => "".into(),
    };
    if !error.is_empty() {
        functions::mach_nichts();
    } else if path == "/stop" {
        content = "Stop!".to_string();
        unsafe {
            STOP = true;
        }
    } else if path == "/favicon.ico" {
        functions::mach_nichts();
    } else if verb == "OPTIONS" {
        options = true; // CORS
        rh.insert(
            "Access-Control-Allow-Methods".into(),
            "POST, GET, OPTIONS".into(),
        );
        rh.insert(
            "Access-Control-Allow-Headers".into(),
            "X-PINGOTHER, Content-Type".into(),
        );
        rh.insert("Access-Control-Max-Age".into(), "86400".into());
    } else if verb == "POST" {
        let mut token = String::new();
        let mut table = String::new();
        let mut mode = String::new();
        let mut data = String::new();
        let r: serde_json::error::Result<Value> = serde_json::from_str(body.as_str());
        if let Ok(js) = r {
            if let Some(x) = js["token"].as_str() {
                token = x.to_string();
                // println!("Token {}", token);
            }
            if let Some(x) = js["table"].as_str() {
                table = x.to_string();
                // println!("Table {}", table);
            }
            if let Some(x) = js["mode"].as_str() {
                mode = x.to_string();
                // println!("Mode {}", mode);
            }
            if let Some(x) = js["data"].as_str() {
                data = x.to_string();
                // println!("Data {}", data);
            }
        }
        if token.is_empty() || token != daten.benutzer_id {
            error = format!("Not allowed {}", token);
        }
        match client_service::replicate_table(&daten, &table, &mode, &data) {
            Ok(json) => {
                contenttype = "application/json; charset=utf-8";
                content = json;
            }
            Err(err) => error = format!("{}", err),
        }
    } else if path == "/" {
        let version =
            env!("CARGO_PKG_NAME").to_string() + ", version: " + env!("CARGO_PKG_VERSION");
        content = format!(
            "<h1>Hällo!</h1><h2>Anfrage: {}</h2><h3>{}</h3><h3>{}</h3>",
            path,
            daten.get_now(),
            version,
        );
    } else {
        error = format!("Unknown resource {}", path);
    }
    if !error.is_empty() {
        status_code = String::from("401 Unauthorized");
        content = error;
    }
    if !options {
        rh.insert("Content-type".into(), contenttype.into());
        rh.insert("Pragma".into(), "no-cache".into());
        rh.insert("Cache-control".into(), "no-cache".into());
        rh.insert("Expires".into(), "-1".into());
    }
    if !origin.is_empty() {
        rh.insert("Access-Control-Allow-Origin".into(), origin.to_string());
        rh.insert("Vary".into(), "Origin".into());
    }
    // if content.is_empty() {
    //     content = "Content".into();
    // }
    HttpResponse {
        status_code,
        headers: rh,
        content,
    }
}

pub fn start() {
    let addr: net::SocketAddr = "0.0.0.0:4202".parse().unwrap();
    // addr.set_port(args.flag_port.unwrap_or(443));
    let config = make_config(/*&args*/);
    let mut listener = TcpListener::bind(addr).expect("cannot listen on port");
    let mut poll = mio::Poll::new().unwrap();
    poll.registry()
        .register(&mut listener, LISTENER, mio::Interest::READABLE)
        .unwrap();

    let _mode = ServerMode::Echo;
    let mode = ServerMode::Http;

    let mut tlsserv = TlsServer::new(listener, mode, config);
    let mut events = mio::Events::with_capacity(256);
    unsafe {
        STOP = false;
    }
    loop {
        // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 4, kind: Interrupted, message: "Unterbrechung während des Betriebssystemaufrufs" }', src/services/https_server.rs:604:38
        unsafe {
            if STOP {
                println!("server stopped");
                break;
            }
        }
        if let Err(err) = poll.poll(&mut events, None) {
            if err.kind() == io::ErrorKind::Interrupted {
                functions::mach_nichts();
            } else {
                println!("error polling {:?}", err);
            }
        } else {
            functions::mach_nichts();
        }
        for event in events.iter() {
            match event.token() {
                LISTENER => {
                    tlsserv
                        .accept(poll.registry())
                        .expect("error accepting socket");
                }
                _ => tlsserv.conn_event(poll.registry(), &event),
            }
        }
    }
    // curl --insecure -X GET https://localhost:4202/hallo
}

// plaintext buffer "POST / HTTP/1.1\r\nHost: ubuntu-w65-67sr:4202\r\nUser-Agent: Mozilla/5.0 (Android 7.1.2; Mobile; rv:92.0) Gecko/92.0 Firefox/92.0\r\nAccept: application/json\r\nAccept-Language: de-DE,en-DE;q=0.7,fr-FR;q=0.3\r\nAccept-Encoding: gzip, deflate, br\r\nReferer: https://jshh.cwkuehl.de/\r\nContent-Type: text/plain\r\nOrigin: https://jshh.cwkuehl.de\r\nContent-Length: 1004\r\nConnection: keep-alive\r\nSec-Fetch-Dest: empty\r\nSec-Fetch-Mode: cors\r\nSec-Fetch-Site: cross-site\r\n\r\n{\"token\":\"wolfgang\",\"table\":\"TB_Eintrag\",\"mode\":\"read_10d\",\"data\":\"{\\\"TB_Eintrag\\\":[{\\\"datum\\\":\\\"2021-09-08\\\",\\\"eintrag\\\":\\\"6.15 Aufstehen, Übungen, Packen, 7.35 mit Claudia, Koffer, Rucksack zum Bahnhof gelaufen, 7.56 Zug nach Mainz, 8.20 Zug nach Hamburg, Hbf 14.30, Tourist Information, 3 Tage Hamburg Card gekauft, Fahrt mit U3 zur Lübecker Straße, City Apartment Hotel, Zimmer 17, Auspacken, 16.00 Spaziergang zur Außenalster, Hbf, Bus zum Kaisertor, Elbphilharmonie, Karten gekauft, Landungsbrücken, Fish&Chips, Pommes (11 € C), S1 zum Berliner Tor, zu Fuß zum Rewe, 20.20 Apartment, Ausruhen, 20.45 mit U1 und S1 zur Reeperbahn, Spaziergang Richtung Sternschanze, 22.00 Zoë II mit 2G-Regeln, Bitter Lemon (7,50 € C), Rote Flora, Sternschanze, Rückfahrt mit S21 und U3, Fernsehen,\\\",\\\"replid\\\":\\\"dfc64879229c-4897-99fb-947c8b60a8da\\\",\\\"angelegtAm\\\":\\\"2021-09-08T08:21:59.000Z\\\",\\\"angelegtVon\\\":\\\"wolfgang\\\",\\\"geaendertAm\\\":\\\"2021-09-13T07:22:16.000Z\\\",\\\"geaendertVon\\\":\\\"wolfgang\\\"}]}\"}"

// plaintext buffer "POST / HTTP/1.1\r\nHost: ubuntu-w65-67sr:4202\r\nUser-Agent: Mozilla/5.0 (Android 7.1.2; Mobile; rv:92.0) Gecko/92.0 Firefox/92.0\r\nAccept: application/json\r\nAccept-Language: de-DE,en-DE;q=0.7,fr-FR;q=0.3\r\nAccept-Encoding: gzip, deflate, br\r\nReferer: https://jshh.cwkuehl.de/\r\nContent-Type: text/plain\r\nOrigin: https://jshh.cwkuehl.de\r\nContent-Length: 84\r\nConnection: keep-alive\r\nSec-Fetch-Dest: empty\r\nSec-Fetch-Mode: cors\r\nSec-Fetch-Site: cross-site\r\n\r\n{\"token\":\"wolfgang\",\"table\":\"HH_Konto\",\"mode\":\"read_10d\",\"data\":\"{\\\"HH_Konto\\\":[]}\"}"

// plaintext buffer "POST / HTTP/1.1\r\nHost: ubuntu-w65-67sr:4202\r\nUser-Agent: Mozilla/5.0 (Android 7.1.2; Mobile; rv:92.0) Gecko/92.0 Firefox/92.0\r\nAccept: application/json\r\nAccept-Language: de-DE,en-DE;q=0.7,fr-FR;q=0.3\r\nAccept-Encoding: gzip, deflate, br\r\nReferer: https://jshh.cwkuehl.de/\r\nContent-Type: text/plain\r\nOrigin: https://jshh.cwkuehl.de\r\nContent-Length: 90\r\nConnection: keep-alive\r\nSec-Fetch-Dest: empty\r\nSec-Fetch-Mode: cors\r\nSec-Fetch-Site: cross-site\r\n\r\n{\"token\":\"wolfgang\",\"table\":\"HH_Ereignis\",\"mode\":\"read_10d\",\"data\":\"{\\\"HH_Ereignis\\\":[]}\"}"

// plaintext buffer "POST / HTTP/1.1\r\nHost: ubuntu-w65-67sr:4202\r\nUser-Agent: Mozilla/5.0 (Android 7.1.2; Mobile; rv:92.0) Gecko/92.0 Firefox/92.0\r\nAccept: application/json\r\nAccept-Language: de-DE,en-DE;q=0.7,fr-FR;q=0.3\r\nAccept-Encoding: gzip, deflate, br\r\nReferer: https://jshh.cwkuehl.de/\r\nContent-Type: text/plain\r\nOrigin: https://jshh.cwkuehl.de\r\nContent-Length: 5645\r\nConnection: keep-alive\r\nSec-Fetch-Dest: empty\r\nSec-Fetch-Mode: cors\r\nSec-Fetch-Site: cross-site\r\n\r\n{\"token\":\"wolfgang\",\"table\":\"HH_Buchung\",\"mode\":\"read_10d\",\"data\":\"{\\\"HH_Buchung\\\":[{\\\"uid\\\":\\\"52e676df52a2-4490-8997-6f97add1dfa6\\\",\\\"sollValuta\\\":\\\"2021-09-12T00:00:00.000Z\\\",\\\"habenValuta\\\":\\\"2021-09-12T00:00:00.000Z\\\",\\\"kz\\\":\\\"A\\\",\\\"betrag\\\":7.04,\\\"ebetrag\\\":3.6,\\\"sollKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f8e\\\",\\\"habenKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f70\\\",\\\"btext\\\":\\\"U-Bahn, Hamburg\\\",\\\"belegNr\\\":null,\\\"belegDatum\\\":\\\"2021-09-12T00:00:00.000Z\\\",\\\"replid\\\":\\\"935be29111ad-49c9-a358-21e5bea46d95\\\",\\\"angelegtAm\\\":\\\"2021-09-12T07:15:08.000Z\\\",\\\"angelegtVon\\\":\\\"wolfgang\\\",\\\"geaendertAm\\\":null,\\\"geaendertVon\\\":null},{\\\"uid\\\":\\\"27e76dc06751-417f-aee3-e7a482677922\\\",\\\"sollValuta\\\":\\\"2021-09-11T00:00:00.000Z\\\",\\\"habenValuta\\\":\\\"2021-09-11T00:00:00.000Z\\\",\\\"kz\\\":\\\"A\\\",\\\"betrag\\\":25.82,\\\"ebetrag\\\":13.2,\\\"sollKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f90\\\",\\\"habenKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f75\\\",\\\"btext\\\":\\\"Kartoffelpuffer, Backkartoffeln, Hamburg\\\",\\\"belegNr\\\":null,\\\"belegDatum\\\":\\\"2021-09-11T00:00:00.000Z\\\",\\\"replid\\\":\\\"bd5ab56ff627-4fdb-88cf-b0847ac3b49f\\\",\\\"angelegtAm\\\":\\\"2021-09-11T13:02:29.000Z\\\",\\\"angelegtVon\\\":\\\"wolfgang\\\",\\\"geaendertAm\\\":\\\"2021-09-11T13:05:06.000Z\\\",\\\"geaendertVon\\\":\\\"wolfgang\\\"},{\\\"uid\\\":\\\"f70a41f876d5-49c4-950c-8c45ce32141e\\\",\\\"sollValuta\\\":\\\"2021-09-11T00:00:00.000Z\\\",\\\"habenValuta\\\":\\\"2021-09-11T00:00:00.000Z\\\",\\\"kz\\\":\\\"A\\\",\\\"betrag\\\":39.12,\\\"ebetrag\\\":20,\\\"sollKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f8f\\\",\\\"habenKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f70\\\",\\\"btext\\\":\\\"BallinStadt Auswanderer-Museum, Hamburg\\\",\\\"belegNr\\\":null,\\\"belegDatum\\\":\\\"2021-09-11T00:00:00.000Z\\\",\\\"replid\\\":\\\"868f520e3e34-461b-a543-63896efed05c\\\",\\\"angelegtAm\\\":\\\"2021-09-11T09:37:41.000Z\\\",\\\"angelegtVon\\\":\\\"wolfgang\\\",\\\"geaendertAm\\\":null,\\\"geaendertVon\\\":null},{\\\"uid\\\":\\\"28a8ff4e3e7e-4502-9134-2f13a1ec455d\\\",\\\"sollValuta\\\":\\\"2021-09-11T00:00:00.000Z\\\",\\\"habenValuta\\\":\\\"2021-09-11T00:00:00.000Z\\\",\\\"kz\\\":\\\"A\\\",\\\"betrag\\\":7.43,\\\"ebetrag\\\":3.8,\\\"sollKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f90\\\",\\\"habenKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f70\\\",\\\"btext\\\":\\\"Bäckerei, Hamburg\\\",\\\"belegNr\\\":null,\\\"belegDatum\\\":\\\"2021-09-11T00:00:00.000Z\\\",\\\"replid\\\":\\\"2cd49018d33d-4893-af85-d2a9daec3564\\\",\\\"angelegtAm\\\":\\\"2021-09-11T08:35:25.000Z\\\",\\\"angelegtVon\\\":\\\"wolfgang\\\",\\\"geaendertAm\\\":null,\\\"geaendertVon\\\":null},{\\\"uid\\\":\\\"7b54ec297d45-4861-82fe-bb694ebaa987\\\",\\\"sollValuta\\\":\\\"2021-09-10T00:00:00.000Z\\\",\\\"habenValuta\\\":\\\"2021-09-10T00:00:00.000Z\\\",\\\"kz\\\":\\\"A\\\",\\\"betrag\\\":25.23,\\\"ebetrag\\\":12.9,\\\"sollKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f90\\\",\\\"habenKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f70\\\",\\\"btext\\\":\\\"Currywurst, Spaghetti, Hamburg\\\",\\\"belegNr\\\":null,\\\"belegDatum\\\":\\\"2021-09-10T00:00:00.000Z\\\",\\\"replid\\\":\\\"e4d955728c4c-411a-aff2-419229910e46\\\",\\\"angelegtAm\\\":\\\"2021-09-10T12:04:08.000Z\\\",\\\"angelegtVon\\\":\\\"wolfgang\\\",\\\"geaendertAm\\\":null,\\\"geaendertVon\\\":null},{\\\"uid\\\":\\\"3fd48214fbfd-4935-a9e2-6477cf13f398\\\",\\\"sollValuta\\\":\\\"2021-09-10T00:00:00.000Z\\\",\\\"habenValuta\\\":\\\"2021-09-10T00:00:00.000Z\\\",\\\"kz\\\":\\\"A\\\",\\\"betrag\\\":9.78,\\\"ebetrag\\\":5,\\\"sollKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f90\\\",\\\"habenKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f70\\\",\\\"btext\\\":\\\"Eis, Hamburg\\\",\\\"belegNr\\\":null,\\\"belegDatum\\\":\\\"2021-09-10T00:00:00.000Z\\\",\\\"replid\\\":\\\"a21f5605c300-41d9-a7e7-dfdb018a175b\\\",\\\"angelegtAm\\\":\\\"2021-09-10T14:06:46.000Z\\\",\\\"angelegtVon\\\":\\\"wolfgang\\\",\\\"geaendertAm\\\":null,\\\"geaendertVon\\\":null},{\\\"uid\\\":\\\"c6d246a67dd9-4a9e-a3df-8249b9fc07e5\\\",\\\"sollValuta\\\":\\\"2021-09-10T00:00:00.000Z\\\",\\\"habenValuta\\\":\\\"2021-09-10T00:00:00.000Z\\\",\\\"kz\\\":\\\"A\\\",\\\"betrag\\\":6.85,\\\"ebetrag\\\":3.5,\\\"sollKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f90\\\",\\\"habenKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f70\\\",\\\"btext\\\":\\\"Bäckerei, Hamburg\\\",\\\"belegNr\\\":null,\\\"belegDatum\\\":\\\"2021-09-10T00:00:00.000Z\\\",\\\"replid\\\":\\\"07c4b10825ff-4aab-8e28-8354613ca861\\\",\\\"angelegtAm\\\":\\\"2021-09-10T06:08:04.000Z\\\",\\\"angelegtVon\\\":\\\"wolfgang\\\",\\\"geaendertAm\\\":null,\\\"geaendertVon\\\":null},{\\\"uid\\\":\\\"cb90d275989a-4f6d-89c5-ba2eb0dd8cb0\\\",\\\"sollValuta\\\":\\\"2021-09-09T00:00:00.000Z\\\",\\\"habenValuta\\\":\\\"2021-09-09T00:00:00.000Z\\\",\\\"kz\\\":\\\"A\\\",\\\"betrag\\\":5.67,\\\"ebetrag\\\":2.9,\\\"sollKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f90\\\",\\\"habenKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f70\\\",\\\"btext\\\":\\\"Bäckerei, Hamburg\\\",\\\"belegNr\\\":null,\\\"belegDatum\\\":\\\"2021-09-09T00:00:00.000Z\\\",\\\"replid\\\":\\\"0e7d880a4ca6-448e-b2d8-5edc0764eb91\\\",\\\"angelegtAm\\\":\\\"2021-09-09T10:36:01.000Z\\\",\\\"angelegtVon\\\":\\\"wolfgang\\\",\\\"geaendertAm\\\":\\\"2021-09-09T10:36:41.000Z\\\",\\\"geaendertVon\\\":\\\"wolfgang\\\"},{\\\"uid\\\":\\\"e5426ee8ce13-4138-9d61-23b8692bdbd6\\\",\\\"sollValuta\\\":\\\"2021-09-08T00:00:00.000Z\\\",\\\"habenValuta\\\":\\\"2021-09-08T00:00:00.000Z\\\",\\\"kz\\\":\\\"A\\\",\\\"betrag\\\":14.67,\\\"ebetrag\\\":7.5,\\\"sollKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f90\\\",\\\"habenKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f70\\\",\\\"btext\\\":\\\"Bitter Lemon, Zoė II, Hamburg\\\",\\\"belegNr\\\":null,\\\"belegDatum\\\":\\\"2021-09-08T00:00:00.000Z\\\",\\\"replid\\\":\\\"235724a20607-4399-ac35-4984ca9a0ec9\\\",\\\"angelegtAm\\\":\\\"2021-09-08T20:09:54.000Z\\\",\\\"angelegtVon\\\":\\\"wolfgang\\\",\\\"geaendertAm\\\":null,\\\"geaendertVon\\\":null},{\\\"uid\\\":\\\"3b4fab3b79ef-4639-9505-3c17f06aeae3\\\",\\\"sollValuta\\\":\\\"2021-09-08T00:00:00.000Z\\\",\\\"habenValuta\\\":\\\"2021-09-08T00:00:00.000Z\\\",\\\"kz\\\":\\\"A\\\",\\\"betrag\\\":21.51,\\\"ebetrag\\\":11,\\\"sollKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f90\\\",\\\"habenKontoUid\\\":\\\"5f0d190d:13e2884456a:-7f70\\\",\\\"btext\\\":\\\"Fisch, Pommes, Landungsbrücken\\\",\\\"belegNr\\\":null,\\\"belegDatum\\\":\\\"2021-09-08T00:00:00.000Z\\\",\\\"replid\\\":\\\"1f0d586b9e61-4e7d-bec8-b3f65bece58c\\\",\\\"angelegtAm\\\":\\\"2021-09-08T17:05:45.000Z\\\",\\\"angelegtVon\\\":\\\"wolfgang\\\",\\\"geaendertAm\\\":null,\\\"geaendertVon\\\":null}]}\"}"
