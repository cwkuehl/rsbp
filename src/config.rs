use crate::res::messages::M;
use crate::{apis::services, base::parameter};
use lazy_static::lazy_static;
use regex::Regex;
use regex::RegexBuilder;
//use separator::{FixedPlaceSeparatable, Separatable};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct RsbpConfig {
    settingfilename: String,
    dbfilename: String,
    locale: RsbpLocale,
}

const SETTINGFILENAME_INIT: &str = ".rsbp.json";
const DBFILENAME_INIT: &str = "/home/wolfgang/hsqldb/rsbp.db";

impl RsbpConfig {
    pub fn init() -> Self {
        RsbpConfig {
            settingfilename: SETTINGFILENAME_INIT.into(),
            dbfilename: "".into(),
            locale: RsbpLocale::En,
        }
    }

    pub fn new(args: Vec<String>) -> crate::Result<Self> {
        if cfg!(debug_assertions) {
            println!("Debugging enabled");
        }
        lazy_static! {
            static ref RE0: Regex = RegexBuilder::new(format!("^{}=(.+)$", "SETTINGS").as_str())
                .case_insensitive(true)
                .build()
                .unwrap();
            static ref RE: Regex =
                RegexBuilder::new(format!("^{}=(.+)$", parameter::DB_DRIVER_CONNECT).as_str())
                    .case_insensitive(true)
                    .build()
                    .unwrap();
        }
        // if args.len() < 2 {
        //     return Err(RsbpError::ConfigError);
        // }
        let mut settingfilename = SETTINGFILENAME_INIT.to_string();
        let mut dbfilename = DBFILENAME_INIT.to_string();
        for arg in args.into_iter().skip(1) {
            for cap in RE0.captures_iter(arg.as_str()) {
                settingfilename = cap[1].to_string();
            }
            for cap in RE.captures_iter(arg.as_str()) {
                dbfilename = cap[1].to_string();
            }
        }
        if !Path::new(settingfilename.as_str()).is_absolute() {
            // Home-Pfad davorhängen.
            let home_dir = home::home_dir();
            let path: PathBuf = match home_dir {
                None => settingfilename.clone().into(),
                Some(p) => [p, settingfilename.clone().into()].iter().collect(),
            };
            settingfilename = path.into_os_string().into_string().map_err(|err| {
                RsbpError::error_string(err.to_str().unwrap_or("Settingfilename"))
            })?;
        }
        if cfg!(debug_assertions) {
            dbg!(&dbfilename);
        }
        let mut locale = RsbpLocale::En;
        // locale_config::Locale::set_current(locale_config::Locale::new("en-GB").unwrap());
        let cis = locale_config::Locale::current();
        let ci = cis.tags_for("messages").into_iter().next();
        if let Some(sci) = ci {
            if sci.to_string().starts_with("de") {
                locale = RsbpLocale::De
            }
            if cfg!(debug_assertions) {
                println!("Locale {}", sci);
            }
        }
        // println!("{:?}", f.langinfo(langinfo::_NL_ADDRESS_COUNTRY_AB2));
        // println!("{:?}", f.langinfo(langinfo::MON_1));
        // println!("{:?}", f.langinfo(langinfo::ABMON_1));
        // println!("{:?}", f.langinfo(langinfo::DAY_1));
        // println!("{:?}", f.langinfo(langinfo::ABDAY_1));
        // println!("{:?}", f.langinfo(langinfo::CURRENCY_SYMBOL));
        // println!("{:?}", f.langinfo(langinfo::INT_CURR_SYMBOL));
        // println!("{:?}", f.langinfo(langinfo::MON_DECIMAL_POINT));
        // println!("{:?}", f.langinfo(langinfo::MON_THOUSANDS_SEP));
        // println!("{}", locale_config::Locale::user_default());

        if false && cfg!(debug_assertions) {
            // println!("{:.*}", 3, 1000000.12_f32);
            // println!("{}", 1000000.12_f32.separated_string());
            // println!("{}", 1000000.12_f64.separated_string());
            // println!("{}", 1234567890.123456789_f64.separated_string());
            // println!(
            //     "{}",
            //     1234567890.123456789_f64.separated_string_with_fixed_place(3)
            // );
        }
        Ok(RsbpConfig {
            settingfilename,
            dbfilename,
            locale,
        })
    }

    pub fn get_settingfilename(&self) -> &String {
        &self.settingfilename
    }

    pub fn get_dbfilename(&self) -> &String {
        &self.dbfilename
    }

    pub fn is_de(&self) -> bool {
        match self.locale {
            RsbpLocale::De => true,
            _ => false,
        }
    }
}

/// Alle unterstützten Sprachen in RSBP.
#[derive(Clone, Copy, Debug)]
pub enum RsbpLocale {
    /// Deutsch
    De,
    /// English
    En,
}

/// Fehler mit mehreren Strings.
#[derive(Debug)]
pub struct MessagesError {
    errors: Vec<String>,
}

impl std::error::Error for MessagesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for MessagesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut not1 = false;
        for x in self.errors.iter() {
            if not1 {
                write!(f, "\n")?;
            } else {
                not1 = true;
            }
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}

/// Alle Fehlermeldungen von RSBP.
#[derive(Debug)]
pub enum RsbpError {
    ConfigError,
    // ChronoError {
    //     source: chrono::format::ParseError,
    // },
    // /// Represents rusqlite::Error.
    // DbError {
    //     source: rusqlite::Error,
    // },
    DieselError {
        source: diesel::result::Error,
    },
    NotFound,
    /// Fehler aus den Services.
    ServiceError {
        source: MessagesError,
    },
    //// Represents all other cases of `std::io::Error`.
    // IOError(std::io::Error)
}

impl std::error::Error for RsbpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            RsbpError::ConfigError => None,
            // RsbpError::ChronoError { ref source } => Some(source),
            // RsbpError::DbError { ref source } => Some(source),
            RsbpError::DieselError { ref source } => Some(source),
            RsbpError::NotFound => None,
            RsbpError::ServiceError { ref source } => Some(source),
            // RsbpError::IOError(_) => None,
        }
    }
}

impl std::fmt::Display for RsbpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RsbpError::ConfigError => write!(f, "Config error"),
            // RsbpError::ChronoError { ref source } => write!(f, "Chrono error ({})", source),
            // RsbpError::DbError { ref source } => write!(f, "Database error ({})", source),
            RsbpError::DieselError { ref source } => write!(f, "Diesel error ({})", source),
            RsbpError::NotFound => write!(f, "Not found"),
            RsbpError::ServiceError { ref source } => write!(f, "{}", source),
            // RsbpError::IOError(ref err) => { err.fmt(f) }
        }
    }
}

impl From<diesel::result::Error> for RsbpError {
    fn from(source: diesel::result::Error) -> RsbpError {
        RsbpError::DieselError { source }
    }
}

impl RsbpError {
    /// Liefert Error aus Liste von Strings.
    pub fn error(r: &Vec<String>) -> RsbpError {
        RsbpError::ServiceError {
            source: MessagesError { errors: r.clone() },
        }
    }

    /// Liefert Error aus String.
    pub fn error_string(r: &str) -> RsbpError {
        let v = vec![r.to_string()];
        RsbpError::ServiceError {
            source: MessagesError { errors: v },
        }
    }

    /// Liefert Error aus Message.
    pub fn error_msg(key: M, is_de: bool) -> RsbpError {
        let r = M::mec(key, is_de).to_owned().to_string();
        let v = vec![r];
        RsbpError::ServiceError {
            source: MessagesError { errors: v },
        }
    }
}

lazy_static! {
    // RwLock statt Mutex wegen mehreren gleichzeitigen Read verwendet
    static ref CONFIG: Arc<RwLock<RsbpConfig>> = Arc::new(RwLock::new(RsbpConfig::init()));
}

/// Liefert Kopie der globalen Konfiguration
pub fn get_config() -> RsbpConfig {
    let guard = match CONFIG.read() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    guard.clone()
}

/// Setzen der Werte in der globalen Konfiguration
pub fn set_config(config: &RsbpConfig) {
    let mut guard = match CONFIG.write() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    (*guard).settingfilename = config.settingfilename.to_string();
    (*guard).dbfilename = config.dbfilename.to_string();
    (*guard).locale = config.locale;
    services::set_config(config);
}

/// Collect errors and return option result.
pub fn get<'a, T>(errors: &mut Vec<String>, r: &'a crate::Result<Option<T>>) -> Option<&'a T> {
    if let Err(err) = r {
        match err {
            RsbpError::ServiceError { source } => {
                for e in source.errors.iter() {
                    errors.push(e.to_string());
                }
            }
            _ => {
                errors.push(err.to_string());
            }
        };
        return None;
    } else if let Ok(erg) = r {
        return erg.as_ref();
    }
    None
}
