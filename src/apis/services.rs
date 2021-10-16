use crate::{config::RsbpConfig, res};
use chrono::{Date, DateTime, Datelike, Local, NaiveDate, NaiveDateTime, Timelike};
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct ServiceDaten {
    pub mandant_nr: i32,
    pub benutzer_id: String,
    pub heute: Date<Local>,
    pub jetzt: DateTime<Local>,
    pub config: RsbpConfig,
}

impl ServiceDaten {
    pub fn init() -> Self {
        ServiceDaten::new0(&RsbpConfig::init(), 0, res::USER_ID)
    }

    pub fn new0(config: &RsbpConfig, mandant_nr: i32, benutzer_id: &str) -> Self {
        let mut now: DateTime<Local> = Local::now();
        now = now.with_nanosecond(0).unwrap_or(now); // nur sekundengenau
        ServiceDaten {
            mandant_nr,
            benutzer_id: String::from(benutzer_id),
            heute: now.date(),
            jetzt: now,
            config: config.clone(),
            //context: None,
        }
    }

    pub fn new() -> Self {
        let daten = get_daten();
        daten
    }

    // pub fn get_now(&self) -> Result<NaiveDateTime, RsbpError> {
    //     let mut s = self.jetzt.to_rfc3339_opts(SecondsFormat::Millis, true);
    //     if s.len() >= 6 {
    //         s = s[..s.len() - 6].to_string();
    //     }
    //     let no_timezone = NaiveDateTime::parse_from_str(s.as_str(), "%Y-%m-%dT%H:%M:%S%.f")
    //         .map_err(|source: chrono::ParseError| RsbpError::ChronoError { source })?;
    //     // let x = no_timezone.to_string();
    //     Ok(no_timezone)
    // }

    pub fn get_now(&self) -> NaiveDateTime {
        let j = &self.jetzt;
        let ndt = NaiveDate::from_ymd(j.year(), j.month(), j.day()).and_hms_nano(
            j.hour(),
            j.minute(),
            j.second(),
            j.timestamp_subsec_nanos(),
        );
        ndt
    }

    pub fn get_today(&self) -> NaiveDate {
        let j = &self.heute;
        let ndt = NaiveDate::from_ymd(j.year(), j.month(), j.day());
        ndt
    }
}

lazy_static! {
    static ref DATEN: Arc<RwLock<ServiceDaten>> = Arc::new(RwLock::new(ServiceDaten::init()));
}

/// Liefert Kopie der globalen ServiceDaten mit aktueller Uhrzeit.
pub fn get_daten() -> ServiceDaten {
    let guard = match DATEN.read() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let daten = ServiceDaten::new0(&guard.config, guard.mandant_nr, guard.benutzer_id.as_str());
    daten
}

/// Setzen der Werte in den globalen ServiceDaten.
pub fn set_config(config: &RsbpConfig) {
    let mut guard = match DATEN.write() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    (*guard).config = config.clone();
}

/// Setzen der Werte in den globalen ServiceDaten
pub fn set_daten(daten: &ServiceDaten) {
    let mut guard = match DATEN.write() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    (*guard).mandant_nr = daten.mandant_nr;
    (*guard).benutzer_id = daten.benutzer_id.to_string();
}
