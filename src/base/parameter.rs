use crate::{
    apis::services,
    base::functions,
    config::{self, RsbpConfig, RsbpError},
    res,
    res::messages::M,
    services::login_service,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug)]
/// Speichern aller möglichen Parameter in XML-Datei und Datenbank.
pub struct Parameter {
    /// Holt oder setzt den Schlüssel.
    key: &'static str,
    /// Holt oder setzt den Wert als String.
    value: Option<String>,
    /// Holt oder setzt den Standardwert.
    default: Option<String>,
    /// Holt oder setzt die Beschreibung zum Parameter.
    comment: Option<String>,
    /// Holt oder setzt einen Wert, der angibt, ob der Wert getrimmt wird.
    _trim: bool,
    ///// Holt oder setzt einen Wert, der angibt, ob der Wert verschlüsselt ist.
    //crypted: bool,
    /// Holt oder setzt einen Wert, der angibt, ob der Wert aus den Benutzer-Einstellungen gelesen wurde?
    loaded: bool,
    /// Holt oder setzt einen Wert, der angibt, ob der Wert in den Benutzer-Einstellungen gespeichert wird?
    setting: Option<&'static str>,
    /// Holt oder setzt einen Wert, der angibt, ob der Wert in der Datenbank gespeichert wird?
    database: bool,
    /// Holt oder setzt die Mandantennummer, 0 heißt Mandant 0, -1 heißt Mandant != 0.
    mandant_nr: i32,
}

/// Parameter-Key: DB_DRIVER_CONNECT.
pub const DB_DRIVER_CONNECT: &str = "DB_DRIVER_CONNECT";

/// Parameter-Key: AG_ANWENDUNGS_TITEL.
const AG_ANWENDUNGS_TITEL: &str = "AG_ANWENDUNGS_TITEL";

/// Parameter-Key: AG_HILFE_DATEI.
const AG_HILFE_DATEI: &str = "AG_HILFE_DATEI";

/// Parameter-Key: AG_TEST_PRODUKTION.
const AG_TEST_PRODUKTION: &str = "AG_TEST_PRODUKTION";

/// Parameter-Key: AG_STARTDIALOGE.
const AG_STARTDIALOGE: &str = "AG_STARTDIALOGE";

/// Parameter-Key: AG_TEMP_PFAD.
const AG_TEMP_PFAD: &str = "AG_TEMP_PFAD";

lazy_static! {
    /// Sammlung von festen Parametern mit Erklärungen.
    static ref PARAMS: Arc<RwLock<HashMap<&'static str, Parameter>>> = {
        let mut map = HashMap::new();
        map.insert(
            DB_DRIVER_CONNECT,
            Parameter {
                key: DB_DRIVER_CONNECT,
                value: None,
                default: Some("Data Source=rsbp.db".to_string()),
                comment: None,
                _trim: true,
                loaded: false,
                setting: Some("ConnectionString"),
                database: false,
                mandant_nr: 0,
            },
        );
        map.insert(
            AG_ANWENDUNGS_TITEL,
            Parameter {
                key: AG_ANWENDUNGS_TITEL,
                value: None,
                default: Some("RSBP".to_string()),
                comment: None,
                _trim: true,
                loaded: false,
                setting: Some("Title"),
                database: true,
                mandant_nr: -1,
            },
        );
        map.insert(
            AG_TEST_PRODUKTION,
            Parameter {
                key: AG_TEST_PRODUKTION,
                value: None,
                default: Some("RSBP".to_string()),
                comment: None,
                _trim: true,
                loaded: false,
                setting: Some("TestProduktion"),
                database: true,
                mandant_nr: 0,
            },
        );
        map.insert(
          AG_HILFE_DATEI,
            Parameter {
                key: AG_HILFE_DATEI,
                value: None,
                default: Some("".to_string()),
                comment: None,
                _trim: true,
                loaded: false,
                setting: Some("HelpFile"),
                database: false,
                mandant_nr: -1,
            },
        );
        map.insert(
            AG_STARTDIALOGE,
            Parameter {
                key: AG_STARTDIALOGE,
                value: None,
                default: Some("".to_string()),
                comment: None,
                _trim: true,
                loaded: false,
                setting: Some("StartingDialogs"),
                database: true,
                mandant_nr: -1,
            },
        );
        map.insert(
            AG_TEMP_PFAD,
            Parameter {
                key: AG_TEMP_PFAD,
                value: None,
                default: Some("".to_string()),
                comment: None,
                _trim: true,
                loaded: false,
                setting: Some("TempPath"),
                database: false,
                mandant_nr: -1,
            },
        );
        let m = Arc::new(RwLock::new(map));
        m
    };
    /// Sammlung von allen Parametern in der Setting-Datei.
    static ref PARAMS2: Arc<RwLock<HashMap<String, Option<String>>>> = {
      let map = HashMap::new();
      let m = Arc::new(RwLock::new(map));
      m
    };
}

impl<'a> Parameter {
    /// Initialisierung aller Parameter aus Setting-Datei und Datenbank.
    /// * config: Betroffene Konfiguration.
    /// * mnr: Standard-Mandant-Nummer.
    pub fn init(config: &RsbpConfig, mnr: i32) -> Result<(), RsbpError> {
        let mut guard = match PARAMS.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let mut guard2 = match PARAMS2.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let mut m_nr = mnr;
        if m_nr < 0 {
            let path: PathBuf = config.get_settingfilename().into();
            let str = fs::read_to_string(path).unwrap_or("{}".to_string());
            let js: Value = serde_json::from_str(str.as_str())
                .map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
            for x in (*guard).values_mut().into_iter() {
                if !x.loaded {
                    let key = format!("parm_{}_value", x.key);
                    x.default = Some(M::ms(key.as_str(), config.is_de()).into_owned());
                    let key = format!("parm_{}_text", x.key);
                    x.comment = Some(M::ms(key.as_str(), config.is_de()).into_owned());
                    if let Some(setting) = x.setting {
                        let jv = js[setting].as_str();
                        if let Some(vstr) = jv {
                            x.value = Some(vstr.to_string());
                        } else if let Some(d) = &x.default {
                            x.value = Some(d.to_string());
                        }
                    }
                    x.loaded = true;
                }
            }
            let jso = js.as_object();
            if let Some(obj) = jso {
                for x in obj.iter() {
                    if x.1.is_string() {
                        let v = match x.1.as_str() {
                            Some(str) => Some(str.to_string()),
                            _ => None,
                        };
                        (*guard2).insert(x.0.to_string(), v);
                    }
                }
            }
            m_nr = js[LOGIN_CLIENT].as_i64().unwrap_or(1) as i32;
        }

        // Parameter aus Datenbank lesen.
        let daten = services::get_daten();
        let liste = login_service::get_list_param(&daten, m_nr)?;
        for x in (*guard).values_mut().into_iter() {
            if x.database {
                if let Some(p) = liste.iter().find(|a| a.schluessel == x.key) {
                    x.value = p.wert.clone();
                }
            }
        }
        //Err(RsbpError::ConfigError)
        Ok(())
    }

    /// Wird der Wert in der Datenbank gespeichert?
    pub fn is_database(&self) -> bool {
        self.database
    }

    /// Wert des Parameters.
    pub fn get_value0(&self) -> Option<String> {
        self.value.clone()
    }

    /// Mandantennummer für Parameter.
    pub fn get_client(&self, m_nr: i32) -> i32 {
        if self.mandant_nr == 0 {
            return 0;
        }
        m_nr
    }

    /// Liefert den Wert eines Parameters.
    fn get_value(key: &str) -> Option<String> {
        let guard = match PARAMS.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(v) = (*guard).get(key) {
            // Setting first.
            if let Some(sk) = v.setting {
                let guard2 = match PARAMS2.read() {
                    Ok(guard) => guard,
                    Err(poisoned) => poisoned.into_inner(),
                };
                if let Some(v) = (*guard2).get(sk) {
                    return v.clone();
                }
            }
            return v.value.clone();
        }
        let guard2 = match PARAMS2.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(v) = (*guard2).get(key) {
            return v.clone();
        }
        None
    }

    /// Schreibt den Wert eines Parameters.
    fn set_value(key: &str, value: &Option<String>) {
        let mut guard = match PARAMS.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(v) = (*guard).get_mut(key) {
            v.value = (*value).clone();
            return;
        }
        let mut guard2 = match PARAMS2.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        //let e = (*guard2).entry(key); //.or_insert((*value).clone());
        if let Some(v) = (*guard2).get_mut(key) {
            *v = (*value).clone();
            return;
        } else {
            (*guard2).insert(key.to_string(), value.clone());
        }
    }
}

/// Speichern der Parameter.
pub fn save() -> Result<(), RsbpError> {
    let mut map = serde_json::Map::new();
    let guard = match PARAMS.read() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    for x in (*guard).iter() {
        let mut key = x.0.to_string();
        if let Some(k) = x.1.setting {
            // Key in setting file.
            key = k.to_string();
        }
        if let Some(v) = &x.1.value {
            map.insert(key, Value::String(v.clone()));
        } else {
            map.insert(key, Value::Null);
        }
    }
    let guard2 = match PARAMS2.read() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    for x in (*guard2).iter() {
        if let Some(v) = &x.1 {
            map.insert(x.0.to_string(), Value::String(v.clone()));
        } else {
            map.insert(x.0.to_string(), Value::Null);
        }
    }
    let s = serde_json::to_string_pretty(&map)
        .map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
    let config = config::get_config();
    fs::write(config.get_settingfilename(), s)
        .map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;

    // Speichern der Parameter in der Datenbank.
    let daten = services::get_daten();
    login_service::save_parameter(&daten, &(*guard))
}

/// Einstellung-Schlüssel: Mandant bei Anmeldung.
const LOGIN_CLIENT: &str = "LoginClient";

/// Lesen des Mandanten für die Anmeldung.
pub fn get_login_client() -> i32 {
    let value = Parameter::get_value(LOGIN_CLIENT);
    let mut m = 1;
    if let Some(v) = value {
        m = functions::to_i32(v.as_str());
    }
    if m <= 0 {
        m = 1;
    }
    m
}

/// Setzen des Mandanten für die nächste Anmeldung.
pub fn set_login_client(m_nr: i32) {
    Parameter::set_value(LOGIN_CLIENT, &Some(m_nr.to_string()));
}

/// Einstellung-Schlüssel: Benutzer bei Anmeldung.
const LOGIN_USER: &str = "LoginUser";

/// Lesen des Benutzers für die Anmeldung.
pub fn get_login_user() -> String {
    if let Some(v) = Parameter::get_value(LOGIN_USER) {
        return v;
    }
    "".into()
}

/// Setzen des Benutzers für die nächste Anmeldung.
pub fn set_login_user(b: &str) {
    Parameter::set_value(LOGIN_USER, &Some(b.to_string()));
}

/// Get application title.
pub fn get_title() -> String {
    if let Some(v) = Parameter::get_value(AG_ANWENDUNGS_TITEL) {
        return v;
    }
    res::APP_NAME.into()
}

/// Is it a test client?
pub fn get_test() -> bool {
    if let Some(v) = &Parameter::get_value(AG_TEST_PRODUKTION) {
        return functions::cmp(v, "TEST");
    }
    true
}

/// Get path to help file.
pub fn get_help_file() -> String {
    if let Some(v) = Parameter::get_value(AG_HILFE_DATEI) {
        return v;
    }
    "".to_string()
}

/// Get amount of day for birthday list.
pub fn get_ad120_days() -> i32 {
    if let Some(v) = Parameter::get_value("AD120Days") {
        return functions::to_i32(v.as_str());
    }
    12
}

/// Set amount of day for birthday list.
pub fn set_ad120_days(v: &str) {
    Parameter::set_value("AD120Days", &Some(v.to_string()));
}

/// Should the birthday list open after login.
pub fn get_ad120_start() -> bool {
    if let Some(v) = Parameter::get_value("AD120Start") {
        return functions::to_bool(v.as_str());
    }
    false
}

/// Set amount of day for birthday list.
pub fn set_ad120_start(v: bool) {
    Parameter::set_value("AD120Start", &Some(functions::bool_to_str(v)));
}

/// Get start dialogs.
pub fn get_start_dialogs() -> String {
    if let Some(v) = Parameter::get_value(AG_STARTDIALOGE) {
        return v;
    }
    "".into()
}

/// Get temp path.
pub fn get_temp_path() -> String {
    if let Some(v) = Parameter::get_value(AG_TEMP_PFAD) {
        return v;
    }
    "".into()
}

/// Set start dialogs.
pub fn set_start_dialogs(sd: &String) {
    Parameter::set_value(AG_STARTDIALOGE, &Some(sd.clone()));
}

/// Reset all dialog sizes.
pub fn reset_dialog_sizes() {
    let mut guard2 = match PARAMS2.write() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let mut keys: Vec<String> = vec![];
    for k in (*guard2).keys() {
        if k.ends_with("_size") {
            keys.push(k.clone());
        }
    }
    for k in keys {
        (*guard2).remove(k.as_str());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// Returns key for a dialog size.
fn get_dialog_key(type_name: &str) -> String {
    format!("{}_size", type_name)
}

/// Returns the dialog size for a type name.
pub fn get_dialog_size(type_name: &str) -> Rectangle {
    let key = get_dialog_key(type_name);
    if let Some(v) = Parameter::get_value(key.as_str()) {
        if let Ok(r) = serde_json::from_str::<Rectangle>(v.as_str()) {
            return r;
        }
    }
    Rectangle {
        x: -1,
        y: -1,
        width: 400,
        height: 300,
    }
}

/// Sets the dialog size for a type name.
pub fn set_dialog_size(type_name: &str, r: &Rectangle) {
    let key = get_dialog_key(type_name);
    if let Ok(js) = serde_json::to_string::<Rectangle>(&r) {
        Parameter::set_value(key.as_str(), &Some(js));
    }
}
