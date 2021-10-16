use gtk::prelude::*;
use log::{error, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use rsbp::{
    base::{functions, parameter::Parameter},
    config::{self, RsbpConfig, RsbpError},
    res,
};
use std::env::args;
use std::process;

fn main() -> Result<(), RsbpError> {
    // Konfiguration bestimmen.
    let config = RsbpConfig::new(args().collect()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    config::set_config(&config);

    // Parameter initialisieren.
    Parameter::init(&config, -1).unwrap_or_else(|err| {
        println!("Problem loading parameters: {}", err);
        process::exit(2);
    });

    // Logger konfigurieren.
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l:5.5} {M} - {m}{n}")))
        .build("output.log")
        .map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
    let cfg = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
    log4rs::init_config(cfg).map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
    if functions::mach_nichts() != 0 {
        error!("Hello, world!");
    }

    // GTK-Anwendung starten.
    let application = gtk::Application::new(Some(res::APP_ID), Default::default());
    application.connect_activate(rsbp::forms::main_window::build_ui);
    application.run();
    Ok(())
}
