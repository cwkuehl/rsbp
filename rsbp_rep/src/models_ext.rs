use chrono::{NaiveDate, NaiveDateTime};

use crate::models::{AdPerson, Benutzer};
use crate::res::M;

impl AdPerson {
    /// Get Name as string.
    pub fn name(&self) -> String {
        let mut sb = String::new();
        if self.person_status != 0 {
            sb.push_str("(");
        }
        sb.push_str(self.name1.as_str());
        if let Some(v) = &self.vorname {
            if !v.is_empty() {
                sb.push_str(", ");
                sb.push_str(v.as_str());
            }
        }
        if self.person_status != 0 {
            sb.push_str(")");
        }
        sb
    }
}

impl Benutzer {
    /// Get permission as string.
    pub fn permission(&self, is_de: bool) -> String {
        let m = match self.berechtigung {
            0 => M::Enum_permission_user,
            1 => M::Enum_permission_admin,
            2 => M::Enum_permission_all,
            _ => M::Enum_permission_no,
        };
        M::me(m, is_de).to_string()
    }
}

/// Extension of TbEintragOrt and TbOrt
#[derive(Debug)]
pub struct TbEintragOrtExt {
    pub mandant_nr: i32,
    pub ort_uid: String,
    pub datum_von: NaiveDate,
    pub datum_bis: NaiveDate,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
    pub bezeichnung: String,
    pub breite: f64,
    pub laenge: f64,
    pub hoehe: f64,
    pub notiz: String,
}

impl Clone for TbEintragOrtExt {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            ort_uid: self.ort_uid.clone(),
            datum_von: self.datum_von.clone(),
            datum_bis: self.datum_bis.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
            bezeichnung: self.bezeichnung.clone(),
            breite: self.breite,
            laenge: self.laenge,
            hoehe: self.hoehe,
            notiz: self.notiz.clone(),
        }
    }
}
