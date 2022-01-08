use crate::{
    revision::Revision,
    schema::{
        AD_ADRESSE, AD_PERSON, AD_SITZ, BENUTZER, BYTE_DATEN, FZ_BUCH, FZ_BUCHAUTOR, FZ_BUCHSERIE,
        FZ_BUCHSTATUS, FZ_FAHRRAD, FZ_FAHRRADSTAND, FZ_NOTIZ, HH_BILANZ, HH_BUCHUNG, HH_EREIGNIS,
        HH_KONTO, HH_PERIODE, MA_MANDANT, MA_PARAMETER, SB_EREIGNIS, SB_FAMILIE, SB_KIND,
        SB_PERSON, SB_QUELLE, SO_KURSE, TB_EINTRAG, TB_EINTRAG_ORT, TB_ORT, WP_ANLAGE, WP_BUCHUNG,
        WP_KONFIGURATION, WP_STAND, WP_WERTPAPIER,
    },
};
use chrono::{NaiveDate, NaiveDateTime};
//use diesel_derives::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "AD_ADRESSE"]
#[allow(non_snake_case)]
pub struct AdAdresse {
    pub mandant_nr: i32,
    pub uid: String,
    pub staat: Option<String>,
    pub plz: Option<String>,
    pub ort: String,
    pub strasse: Option<String>,
    pub hausnr: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for AdAdresse {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            staat: self.staat.clone(),
            plz: self.plz.clone(),
            ort: self.ort.clone(),
            strasse: self.strasse.clone(),
            hausnr: self.hausnr.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for AdAdresse {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.staat == other.staat
            && self.plz == other.plz
            && self.ort == other.ort
            && self.strasse == other.strasse
            && self.hausnr == other.hausnr
    }
}

impl Revision for AdAdresse {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "AD_PERSON"]
#[allow(non_snake_case)]
pub struct AdPerson {
    pub mandant_nr: i32,
    pub uid: String,
    pub typ: i32,
    pub geschlecht: String,
    pub geburt: Option<NaiveDate>,
    pub geburtk: i32,
    pub anrede: i32,
    pub fanrede: i32,
    pub name1: String,
    pub name2: Option<String>,
    pub praedikat: Option<String>,
    pub vorname: Option<String>,
    pub titel: Option<String>,
    pub person_status: i32,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for AdPerson {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            typ: self.typ,
            geschlecht: self.geschlecht.clone(),
            geburt: self.geburt.clone(),
            geburtk: self.geburtk,
            anrede: self.anrede,
            fanrede: self.fanrede,
            name1: self.name1.clone(),
            name2: self.name2.clone(),
            praedikat: self.praedikat.clone(),
            vorname: self.vorname.clone(),
            titel: self.titel.clone(),
            person_status: self.person_status,
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for AdPerson {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.typ == other.typ
            && self.geschlecht == other.geschlecht
            && self.geburt == other.geburt
            && self.geburtk == other.geburtk
            && self.anrede == other.anrede
            && self.fanrede == other.fanrede
            && self.name1 == other.name1
            && self.name2 == other.name2
            && self.praedikat == other.praedikat
            && self.vorname == other.vorname
            && self.titel == other.titel
            && self.person_status == other.person_status
    }
}

impl Revision for AdPerson {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "AD_SITZ"]
#[allow(non_snake_case)]
pub struct AdSitz {
    pub mandant_nr: i32,
    pub person_uid: String,
    pub reihenfolge: i32,
    pub uid: String,
    pub typ: i32,
    pub name: String,
    pub adresse_uid: Option<String>,
    pub telefon: Option<String>,
    pub fax: Option<String>,
    pub mobil: Option<String>,
    pub email: Option<String>,
    pub homepage: Option<String>,
    pub postfach: Option<String>,
    pub bemerkung: Option<String>,
    pub sitz_status: i32,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for AdSitz {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            person_uid: self.person_uid.clone(),
            reihenfolge: self.reihenfolge,
            uid: self.uid.clone(),
            typ: self.typ,
            name: self.name.clone(),
            adresse_uid: self.adresse_uid.clone(),
            telefon: self.telefon.clone(),
            fax: self.fax.clone(),
            mobil: self.mobil.clone(),
            email: self.email.clone(),
            homepage: self.homepage.clone(),
            postfach: self.postfach.clone(),
            bemerkung: self.bemerkung.clone(),
            sitz_status: self.sitz_status,
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for AdSitz {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.person_uid == other.person_uid
            && self.reihenfolge == other.reihenfolge
            && self.uid == other.uid
            && self.typ == other.typ
            && self.name == other.name
            && self.adresse_uid == other.adresse_uid
            && self.telefon == other.telefon
            && self.fax == other.fax
            && self.mobil == other.mobil
            && self.email == other.email
            && self.homepage == other.homepage
            && self.postfach == other.postfach
            && self.bemerkung == other.bemerkung
            && self.sitz_status == other.sitz_status
    }
}

impl Revision for AdSitz {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "BENUTZER"]
#[allow(non_snake_case)]
pub struct Benutzer {
    pub mandant_nr: i32,
    pub benutzer_id: String,
    pub passwort: Option<String>,
    pub berechtigung: i32,
    pub akt_periode: i32,
    pub person_nr: i32,
    pub geburt: Option<NaiveDate>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for Benutzer {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            benutzer_id: self.benutzer_id.clone(),
            passwort: self.passwort.clone(),
            berechtigung: self.berechtigung,
            akt_periode: self.akt_periode,
            person_nr: self.person_nr,
            geburt: self.geburt.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for Benutzer {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.benutzer_id == other.benutzer_id
            && self.passwort == other.passwort
            && self.berechtigung == other.berechtigung
            && self.akt_periode == other.akt_periode
            && self.person_nr == other.person_nr
            && self.geburt == other.geburt
    }
}

impl Revision for Benutzer {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "BYTE_DATEN"]
#[allow(non_snake_case)]
pub struct ByteDaten {
    pub mandant_nr: i32,
    pub typ: String,
    pub uid: String,
    pub lfd_nr: i32,
    pub metadaten: Option<String>,
    pub bytes: Option<Vec<u8>>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for ByteDaten {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            typ: self.typ.clone(),
            uid: self.uid.clone(),
            lfd_nr: self.lfd_nr,
            metadaten: self.metadaten.clone(),
            bytes: self.bytes.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for ByteDaten {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.typ == other.typ
            && self.uid == other.uid
            && self.lfd_nr == other.lfd_nr
            && self.metadaten == other.metadaten
            && self.bytes == other.bytes
    }
}

impl Revision for ByteDaten {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "FZ_BUCH"]
#[allow(non_snake_case)]
pub struct FzBuch {
    pub mandant_nr: i32,
    pub uid: String,
    pub autor_uid: String,
    pub serie_uid: String,
    pub seriennummer: i32,
    pub titel: String,
    pub untertitel: Option<String>,
    pub seiten: i32,
    pub sprache_nr: i32,
    pub notiz: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for FzBuch {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            autor_uid: self.autor_uid.clone(),
            serie_uid: self.serie_uid.clone(),
            seriennummer: self.seriennummer,
            titel: self.titel.clone(),
            untertitel: self.untertitel.clone(),
            seiten: self.seiten,
            sprache_nr: self.sprache_nr,
            notiz: self.notiz.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for FzBuch {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.autor_uid == other.autor_uid
            && self.serie_uid == other.serie_uid
            && self.seriennummer == other.seriennummer
            && self.titel == other.titel
            && self.untertitel == other.untertitel
            && self.seiten == other.seiten
            && self.sprache_nr == other.sprache_nr
            && self.notiz == other.notiz
    }
}

impl Revision for FzBuch {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "FZ_BUCHAUTOR"]
#[allow(non_snake_case)]
pub struct FzBuchautor {
    pub mandant_nr: i32,
    pub uid: String,
    pub name: String,
    pub vorname: Option<String>,
    pub notiz: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for FzBuchautor {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            name: self.name.clone(),
            vorname: self.vorname.clone(),
            notiz: self.notiz.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for FzBuchautor {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.name == other.name
            && self.vorname == other.vorname
            && self.notiz == other.notiz
    }
}

impl Revision for FzBuchautor {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "FZ_BUCHSERIE"]
#[allow(non_snake_case)]
pub struct FzBuchserie {
    pub mandant_nr: i32,
    pub uid: String,
    pub name: String,
    pub notiz: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for FzBuchserie {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            name: self.name.clone(),
            notiz: self.notiz.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for FzBuchserie {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.name == other.name
            && self.notiz == other.notiz
    }
}

impl Revision for FzBuchserie {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "FZ_BUCHSTATUS"]
#[allow(non_snake_case)]
pub struct FzBuchstatus {
    pub mandant_nr: i32,
    pub buch_uid: String,
    pub ist_besitz: bool,
    pub lesedatum: Option<NaiveDate>,
    pub hoerdatum: Option<NaiveDate>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
    pub replikation_uid: Option<String>,
}

impl Clone for FzBuchstatus {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            buch_uid: self.buch_uid.clone(),
            ist_besitz: self.ist_besitz,
            lesedatum: self.lesedatum.clone(),
            hoerdatum: self.hoerdatum.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
            replikation_uid: self.replikation_uid.clone(),
        }
    }
}

impl PartialEq for FzBuchstatus {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.buch_uid == other.buch_uid
            && self.ist_besitz == other.ist_besitz
            && self.lesedatum == other.lesedatum
            && self.hoerdatum == other.hoerdatum
    }
}

impl Revision for FzBuchstatus {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "FZ_FAHRRAD"]
#[allow(non_snake_case)]
pub struct FzFahrrad {
    pub mandant_nr: i32,
    pub uid: String,
    pub bezeichnung: String,
    pub typ: i32,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for FzFahrrad {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            bezeichnung: self.bezeichnung.clone(),
            typ: self.typ,
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for FzFahrrad {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.bezeichnung == other.bezeichnung
            && self.typ == other.typ
    }
}

impl Revision for FzFahrrad {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "FZ_FAHRRADSTAND"]
#[allow(non_snake_case)]
pub struct FzFahrradstand {
    pub mandant_nr: i32,
    pub fahrrad_uid: String,
    pub datum: NaiveDate,
    pub nr: i32,
    pub zaehler_km: f64,
    pub periode_km: f64,
    pub periode_schnitt: f64,
    pub beschreibung: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
    pub replikation_uid: Option<String>,
}

impl Clone for FzFahrradstand {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            fahrrad_uid: self.fahrrad_uid.clone(),
            datum: self.datum.clone(),
            nr: self.nr,
            zaehler_km: self.zaehler_km,
            periode_km: self.periode_km,
            periode_schnitt: self.periode_schnitt,
            beschreibung: self.beschreibung.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
            replikation_uid: self.replikation_uid.clone(),
        }
    }
}

impl PartialEq for FzFahrradstand {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.fahrrad_uid == other.fahrrad_uid
            && self.datum == other.datum
            && self.nr == other.nr
            && self.zaehler_km == other.zaehler_km
            && self.periode_km == other.periode_km
            && self.periode_schnitt == other.periode_schnitt
            && self.beschreibung == other.beschreibung
    }
}

impl Revision for FzFahrradstand {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "FZ_NOTIZ"]
#[allow(non_snake_case)]
pub struct FzNotiz {
    pub mandant_nr: i32,
    pub uid: String,
    pub thema: String,
    pub notiz: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for FzNotiz {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            thema: self.thema.clone(),
            notiz: self.notiz.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for FzNotiz {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.thema == other.thema
            && self.notiz == other.notiz
    }
}

impl Revision for FzNotiz {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "HH_BILANZ"]
#[allow(non_snake_case)]
pub struct HhBilanz {
    pub mandant_nr: i32,
    pub periode: i32,
    pub kz: String,
    pub konto_uid: String,
    pub sh: String,
    pub betrag: f64,
    pub esh: String,
    pub ebetrag: f64,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for HhBilanz {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            periode: self.periode,
            kz: self.kz.clone(),
            konto_uid: self.konto_uid.clone(),
            sh: self.sh.clone(),
            betrag: self.betrag,
            esh: self.esh.clone(),
            ebetrag: self.ebetrag,
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for HhBilanz {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.periode == other.periode
            && self.kz == other.kz
            && self.konto_uid == other.konto_uid
            && self.sh == other.sh
            && self.betrag == other.betrag
            && self.esh == other.esh
            && self.ebetrag == other.ebetrag
    }
}

impl Revision for HhBilanz {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "HH_BUCHUNG"]
#[allow(non_snake_case)]
pub struct HhBuchung {
    pub mandant_nr: i32,
    pub uid: String,
    pub soll_valuta: NaiveDate,
    pub haben_valuta: NaiveDate,
    pub kz: Option<String>,
    pub betrag: f64,
    pub ebetrag: f64,
    pub soll_konto_uid: String,
    pub haben_konto_uid: String,
    pub btext: String,
    pub beleg_nr: Option<String>,
    pub beleg_datum: NaiveDate,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for HhBuchung {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            soll_valuta: self.soll_valuta.clone(),
            haben_valuta: self.haben_valuta.clone(),
            kz: self.kz.clone(),
            betrag: self.betrag,
            ebetrag: self.ebetrag,
            soll_konto_uid: self.soll_konto_uid.clone(),
            haben_konto_uid: self.haben_konto_uid.clone(),
            btext: self.btext.clone(),
            beleg_nr: self.beleg_nr.clone(),
            beleg_datum: self.beleg_datum.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for HhBuchung {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.soll_valuta == other.soll_valuta
            && self.haben_valuta == other.haben_valuta
            && self.kz == other.kz
            && self.betrag == other.betrag
            && self.ebetrag == other.ebetrag
            && self.soll_konto_uid == other.soll_konto_uid
            && self.haben_konto_uid == other.haben_konto_uid
            && self.btext == other.btext
            && self.beleg_nr == other.beleg_nr
            && self.beleg_datum == other.beleg_datum
    }
}

impl Revision for HhBuchung {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "HH_EREIGNIS"]
#[allow(non_snake_case)]
pub struct HhEreignis {
    pub mandant_nr: i32,
    pub uid: String,
    pub kz: Option<String>,
    pub soll_konto_uid: String,
    pub haben_konto_uid: String,
    pub bezeichnung: String,
    pub etext: String,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for HhEreignis {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            kz: self.kz.clone(),
            soll_konto_uid: self.soll_konto_uid.clone(),
            haben_konto_uid: self.haben_konto_uid.clone(),
            bezeichnung: self.bezeichnung.clone(),
            etext: self.etext.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for HhEreignis {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.kz == other.kz
            && self.soll_konto_uid == other.soll_konto_uid
            && self.haben_konto_uid == other.haben_konto_uid
            && self.bezeichnung == other.bezeichnung
            && self.etext == other.etext
    }
}

impl Revision for HhEreignis {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "HH_KONTO"]
#[allow(non_snake_case)]
pub struct HhKonto {
    pub mandant_nr: i32,
    pub uid: String,
    pub sortierung: String,
    pub art: String,
    pub kz: Option<String>,
    pub name: String,
    pub gueltig_von: Option<NaiveDate>,
    pub gueltig_bis: Option<NaiveDate>,
    pub periode_von: i32,
    pub periode_bis: i32,
    pub betrag: f64,
    pub ebetrag: f64,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for HhKonto {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            sortierung: self.sortierung.clone(),
            art: self.art.clone(),
            kz: self.kz.clone(),
            name: self.name.clone(),
            gueltig_von: self.gueltig_von.clone(),
            gueltig_bis: self.gueltig_bis.clone(),
            periode_von: self.periode_von,
            periode_bis: self.periode_bis,
            betrag: self.betrag,
            ebetrag: self.ebetrag,
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for HhKonto {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.sortierung == other.sortierung
            && self.art == other.art
            && self.kz == other.kz
            && self.name == other.name
            && self.gueltig_von == other.gueltig_von
            && self.gueltig_bis == other.gueltig_bis
            && self.periode_von == other.periode_von
            && self.periode_bis == other.periode_bis
            && self.betrag == other.betrag
            && self.ebetrag == other.ebetrag
    }
}

impl Revision for HhKonto {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "HH_PERIODE"]
#[allow(non_snake_case)]
pub struct HhPeriode {
    pub mandant_nr: i32,
    pub nr: i32,
    pub datum_von: NaiveDate,
    pub datum_bis: NaiveDate,
    pub art: i32,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for HhPeriode {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            nr: self.nr,
            datum_von: self.datum_von.clone(),
            datum_bis: self.datum_bis.clone(),
            art: self.art,
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for HhPeriode {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.nr == other.nr
            && self.datum_von == other.datum_von
            && self.datum_bis == other.datum_bis
            && self.art == other.art
    }
}

impl Revision for HhPeriode {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "MA_MANDANT"]
#[allow(non_snake_case)]
pub struct MaMandant {
    pub nr: i32,
    pub beschreibung: String,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for MaMandant {
    fn clone(&self) -> Self {
        Self {
            nr: self.nr,
            beschreibung: self.beschreibung.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for MaMandant {
    fn eq(&self, other: &Self) -> bool {
        self.nr == other.nr && self.beschreibung == other.beschreibung
    }
}

impl Revision for MaMandant {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "MA_PARAMETER"]
#[allow(non_snake_case)]
pub struct MaParameter {
    pub mandant_nr: i32,
    pub schluessel: String,
    pub wert: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
    pub replikation_uid: Option<String>,
}

impl Clone for MaParameter {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            schluessel: self.schluessel.clone(),
            wert: self.wert.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
            replikation_uid: self.replikation_uid.clone(),
        }
    }
}

impl PartialEq for MaParameter {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.schluessel == other.schluessel
            && self.wert == other.wert
    }
}

impl Revision for MaParameter {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "SB_EREIGNIS"]
#[allow(non_snake_case)]
pub struct SbEreignis {
    pub mandant_nr: i32,
    pub person_uid: String,
    pub familie_uid: String,
    pub typ: String,
    pub tag1: i32,
    pub monat1: i32,
    pub jahr1: i32,
    pub tag2: i32,
    pub monat2: i32,
    pub jahr2: i32,
    pub datum_typ: String,
    pub ort: Option<String>,
    pub bemerkung: Option<String>,
    pub quelle_uid: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
    pub replikation_uid: Option<String>,
}

impl Clone for SbEreignis {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            person_uid: self.person_uid.clone(),
            familie_uid: self.familie_uid.clone(),
            typ: self.typ.clone(),
            tag1: self.tag1,
            monat1: self.monat1,
            jahr1: self.jahr1,
            tag2: self.tag2,
            monat2: self.monat2,
            jahr2: self.jahr2,
            datum_typ: self.datum_typ.clone(),
            ort: self.ort.clone(),
            bemerkung: self.bemerkung.clone(),
            quelle_uid: self.quelle_uid.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
            replikation_uid: self.replikation_uid.clone(),
        }
    }
}

impl PartialEq for SbEreignis {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.person_uid == other.person_uid
            && self.familie_uid == other.familie_uid
            && self.typ == other.typ
            && self.tag1 == other.tag1
            && self.monat1 == other.monat1
            && self.jahr1 == other.jahr1
            && self.tag2 == other.tag2
            && self.monat2 == other.monat2
            && self.jahr2 == other.jahr2
            && self.datum_typ == other.datum_typ
            && self.ort == other.ort
            && self.bemerkung == other.bemerkung
            && self.quelle_uid == other.quelle_uid
    }
}

impl Revision for SbEreignis {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "SB_FAMILIE"]
#[allow(non_snake_case)]
pub struct SbFamilie {
    pub mandant_nr: i32,
    pub uid: String,
    pub mann_uid: Option<String>,
    pub frau_uid: Option<String>,
    pub status1: i32,
    pub status2: i32,
    pub status3: i32,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for SbFamilie {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            mann_uid: self.mann_uid.clone(),
            frau_uid: self.frau_uid.clone(),
            status1: self.status1,
            status2: self.status2,
            status3: self.status3,
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for SbFamilie {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.mann_uid == other.mann_uid
            && self.frau_uid == other.frau_uid
            && self.status1 == other.status1
            && self.status2 == other.status2
            && self.status3 == other.status3
    }
}

impl Revision for SbFamilie {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "SB_KIND"]
#[allow(non_snake_case)]
pub struct SbKind {
    pub mandant_nr: i32,
    pub familie_uid: String,
    pub kind_uid: String,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
    pub replikation_uid: Option<String>,
}

impl Clone for SbKind {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            familie_uid: self.familie_uid.clone(),
            kind_uid: self.kind_uid.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
            replikation_uid: self.replikation_uid.clone(),
        }
    }
}

impl PartialEq for SbKind {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.familie_uid == other.familie_uid
            && self.kind_uid == other.kind_uid
    }
}

impl Revision for SbKind {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "SB_PERSON"]
#[allow(non_snake_case)]
pub struct SbPerson {
    pub mandant_nr: i32,
    pub uid: String,
    pub name: String,
    pub vorname: Option<String>,
    pub geburtsname: Option<String>,
    pub geschlecht: Option<String>,
    pub titel: Option<String>,
    pub konfession: Option<String>,
    pub bemerkung: Option<String>,
    pub quelle_uid: Option<String>,
    pub status1: i32,
    pub status2: i32,
    pub status3: i32,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for SbPerson {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            name: self.name.clone(),
            vorname: self.vorname.clone(),
            geburtsname: self.geburtsname.clone(),
            geschlecht: self.geschlecht.clone(),
            titel: self.titel.clone(),
            konfession: self.konfession.clone(),
            bemerkung: self.bemerkung.clone(),
            quelle_uid: self.quelle_uid.clone(),
            status1: self.status1,
            status2: self.status2,
            status3: self.status3,
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for SbPerson {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.name == other.name
            && self.vorname == other.vorname
            && self.geburtsname == other.geburtsname
            && self.geschlecht == other.geschlecht
            && self.titel == other.titel
            && self.konfession == other.konfession
            && self.bemerkung == other.bemerkung
            && self.quelle_uid == other.quelle_uid
            && self.status1 == other.status1
            && self.status2 == other.status2
            && self.status3 == other.status3
    }
}

impl Revision for SbPerson {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "SB_QUELLE"]
#[allow(non_snake_case)]
pub struct SbQuelle {
    pub mandant_nr: i32,
    pub uid: String,
    pub beschreibung: String,
    pub zitat: Option<String>,
    pub bemerkung: Option<String>,
    pub autor: String,
    pub status1: i32,
    pub status2: i32,
    pub status3: i32,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for SbQuelle {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            beschreibung: self.beschreibung.clone(),
            zitat: self.zitat.clone(),
            bemerkung: self.bemerkung.clone(),
            autor: self.autor.clone(),
            status1: self.status1,
            status2: self.status2,
            status3: self.status3,
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for SbQuelle {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.beschreibung == other.beschreibung
            && self.zitat == other.zitat
            && self.bemerkung == other.bemerkung
            && self.autor == other.autor
            && self.status1 == other.status1
            && self.status2 == other.status2
            && self.status3 == other.status3
    }
}

impl Revision for SbQuelle {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "SO_KURSE"]
#[allow(non_snake_case)]
pub struct SoKurse {
    pub datum: NaiveDate,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub price: f64,
    pub bewertung: String,
    pub trend: String,
    pub bemerkung: String,
}

impl Clone for SoKurse {
    fn clone(&self) -> Self {
        Self {
            datum: self.datum.clone(),
            open: self.open,
            high: self.high,
            low: self.low,
            close: self.close,
            price: self.price,
            bewertung: self.bewertung.clone(),
            trend: self.trend.clone(),
            bemerkung: self.bemerkung.clone(),
        }
    }
}

impl PartialEq for SoKurse {
    fn eq(&self, other: &Self) -> bool {
        self.datum == other.datum
            && self.open == other.open
            && self.high == other.high
            && self.low == other.low
            && self.close == other.close
            && self.price == other.price
            && self.bewertung == other.bewertung
            && self.trend == other.trend
            && self.bemerkung == other.bemerkung
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "TB_EINTRAG"]
#[allow(non_snake_case)]
pub struct TbEintrag {
    pub mandant_nr: i32,
    pub datum: NaiveDate,
    pub eintrag: String,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
    pub replikation_uid: Option<String>,
}

impl Clone for TbEintrag {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            datum: self.datum.clone(),
            eintrag: self.eintrag.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
            replikation_uid: self.replikation_uid.clone(),
        }
    }
}

impl PartialEq for TbEintrag {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.datum == other.datum
            && self.eintrag == other.eintrag
    }
}

impl Revision for TbEintrag {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, Associations, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "TB_EINTRAG_ORT"]
#[allow(non_snake_case)]
pub struct TbEintragOrt {
    pub mandant_nr: i32,
    pub ort_uid: String,
    pub datum_von: NaiveDate,
    pub datum_bis: NaiveDate,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for TbEintragOrt {
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
        }
    }
}

impl PartialEq for TbEintragOrt {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.ort_uid == other.ort_uid
            && self.datum_von == other.datum_von
            && self.datum_bis == other.datum_bis
    }
}

impl Revision for TbEintragOrt {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(
    Queryable, Insertable, Associations, AsChangeset, Debug, Serialize, Deserialize, QueryableByName,
)]
#[table_name = "TB_ORT"]
#[allow(non_snake_case)]
pub struct TbOrt {
    pub mandant_nr: i32,
    pub uid: String,
    pub bezeichnung: String,
    pub breite: f64,
    pub laenge: f64,
    pub hoehe: f64,
    pub notiz: String,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for TbOrt {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            bezeichnung: self.bezeichnung.clone(),
            breite: self.breite,
            laenge: self.laenge,
            hoehe: self.hoehe,
            notiz: self.notiz.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for TbOrt {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.bezeichnung == other.bezeichnung
            && self.breite == other.breite
            && self.laenge == other.laenge
            && self.hoehe == other.hoehe
            && self.notiz == other.notiz
    }
}

impl Revision for TbOrt {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "WP_ANLAGE"]
#[allow(non_snake_case)]
pub struct WpAnlage {
    pub mandant_nr: i32,
    pub uid: String,
    pub wertpapier_uid: String,
    pub bezeichnung: String,
    pub parameter: Option<String>,
    pub notiz: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for WpAnlage {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            wertpapier_uid: self.wertpapier_uid.clone(),
            bezeichnung: self.bezeichnung.clone(),
            parameter: self.parameter.clone(),
            notiz: self.notiz.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for WpAnlage {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.wertpapier_uid == other.wertpapier_uid
            && self.bezeichnung == other.bezeichnung
            && self.parameter == other.parameter
            && self.notiz == other.notiz
    }
}

impl Revision for WpAnlage {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "WP_BUCHUNG"]
#[allow(non_snake_case)]
pub struct WpBuchung {
    pub mandant_nr: i32,
    pub uid: String,
    pub wertpapier_uid: String,
    pub anlage_uid: String,
    pub datum: NaiveDate,
    pub zahlungsbetrag: f64,
    pub rabattbetrag: f64,
    pub anteile: f64,
    pub zinsen: f64,
    pub btext: String,
    pub notiz: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for WpBuchung {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            wertpapier_uid: self.wertpapier_uid.clone(),
            anlage_uid: self.anlage_uid.clone(),
            datum: self.datum.clone(),
            zahlungsbetrag: self.zahlungsbetrag,
            rabattbetrag: self.rabattbetrag,
            anteile: self.anteile,
            zinsen: self.zinsen,
            btext: self.btext.clone(),
            notiz: self.notiz.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for WpBuchung {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.wertpapier_uid == other.wertpapier_uid
            && self.anlage_uid == other.anlage_uid
            && self.datum == other.datum
            && self.zahlungsbetrag == other.zahlungsbetrag
            && self.rabattbetrag == other.rabattbetrag
            && self.anteile == other.anteile
            && self.zinsen == other.zinsen
            && self.btext == other.btext
            && self.notiz == other.notiz
    }
}

impl Revision for WpBuchung {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "WP_KONFIGURATION"]
#[allow(non_snake_case)]
pub struct WpKonfiguration {
    pub mandant_nr: i32,
    pub uid: String,
    pub bezeichnung: String,
    pub parameter: String,
    pub status: String,
    pub notiz: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for WpKonfiguration {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            bezeichnung: self.bezeichnung.clone(),
            parameter: self.parameter.clone(),
            status: self.status.clone(),
            notiz: self.notiz.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for WpKonfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.bezeichnung == other.bezeichnung
            && self.parameter == other.parameter
            && self.status == other.status
            && self.notiz == other.notiz
    }
}

impl Revision for WpKonfiguration {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "WP_STAND"]
#[allow(non_snake_case)]
pub struct WpStand {
    pub mandant_nr: i32,
    pub wertpapier_uid: String,
    pub datum: NaiveDate,
    pub stueckpreis: f64,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for WpStand {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            wertpapier_uid: self.wertpapier_uid.clone(),
            datum: self.datum.clone(),
            stueckpreis: self.stueckpreis,
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for WpStand {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.wertpapier_uid == other.wertpapier_uid
            && self.datum == other.datum
            && self.stueckpreis == other.stueckpreis
    }
}

impl Revision for WpStand {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "WP_WERTPAPIER"]
#[allow(non_snake_case)]
pub struct WpWertpapier {
    pub mandant_nr: i32,
    pub uid: String,
    pub bezeichnung: String,
    pub kuerzel: String,
    pub parameter: Option<String>,
    pub datenquelle: String,
    pub status: String,
    pub relation_uid: Option<String>,
    pub notiz: Option<String>,
    pub angelegt_von: Option<String>,
    pub angelegt_am: Option<NaiveDateTime>,
    pub geaendert_von: Option<String>,
    pub geaendert_am: Option<NaiveDateTime>,
}

impl Clone for WpWertpapier {
    fn clone(&self) -> Self {
        Self {
            mandant_nr: self.mandant_nr,
            uid: self.uid.clone(),
            bezeichnung: self.bezeichnung.clone(),
            kuerzel: self.kuerzel.clone(),
            parameter: self.parameter.clone(),
            datenquelle: self.datenquelle.clone(),
            status: self.status.clone(),
            relation_uid: self.relation_uid.clone(),
            notiz: self.notiz.clone(),
            angelegt_von: self.angelegt_von.clone(),
            angelegt_am: self.angelegt_am.clone(),
            geaendert_von: self.geaendert_von.clone(),
            geaendert_am: self.geaendert_am.clone(),
        }
    }
}

impl PartialEq for WpWertpapier {
    fn eq(&self, other: &Self) -> bool {
        self.mandant_nr == other.mandant_nr
            && self.uid == other.uid
            && self.bezeichnung == other.bezeichnung
            && self.kuerzel == other.kuerzel
            && self.parameter == other.parameter
            && self.datenquelle == other.datenquelle
            && self.status == other.status
            && self.relation_uid == other.relation_uid
            && self.notiz == other.notiz
    }
}

impl Revision for WpWertpapier {
    fn get_angelegt_von(&self) -> Option<String> {
        self.angelegt_von.clone()
    }
    fn set_angelegt_von(&mut self, von: &Option<String>) {
        self.angelegt_von = von.clone();
    }
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {
        self.angelegt_am
    }
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {
        self.angelegt_am = am.clone();
    }
    fn get_geaendert_von(&self) -> Option<String> {
        self.geaendert_von.clone()
    }
    fn set_geaendert_von(&mut self, von: &Option<String>) {
        self.geaendert_von = von.clone();
    }
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {
        self.geaendert_am
    }
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {
        self.geaendert_am = am.clone();
    }
}
