use super::reps::DbContext;
use crate::{base::functions, services::reps, Result};
use lazy_static::lazy_static;
use rsbp_rep::models::{
    AdAdresse, AdPerson, AdSitz, Benutzer, ByteDaten, FzBuch, FzBuchautor, FzBuchserie,
    FzBuchstatus, FzFahrrad, FzFahrradstand, FzNotiz, HhBilanz, HhBuchung, HhEreignis, HhKonto,
    HhPeriode, MaMandant, MaParameter, SbEreignis, SbFamilie, SbKind, SbPerson, SbQuelle, SoKurse,
    TbEintrag, TbEintragOrt, TbOrt, WpAnlage, WpBuchung, WpKonfiguration, WpStand, WpWertpapier,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum UndoEntry {
    AdAdresse { original: String, actual: String },
    AdPerson { original: String, actual: String },
    AdSitz { original: String, actual: String },
    Benutzer { original: String, actual: String },
    ByteDaten { original: String, actual: String },
    FzBuch { original: String, actual: String },
    FzBuchautor { original: String, actual: String },
    FzBuchserie { original: String, actual: String },
    FzBuchstatus { original: String, actual: String },
    FzFahrrad { original: String, actual: String },
    FzFahrradstand { original: String, actual: String },
    FzNotiz { original: String, actual: String },
    HhBilanz { original: String, actual: String },
    HhBuchung { original: String, actual: String },
    HhEreignis { original: String, actual: String },
    HhKonto { original: String, actual: String },
    HhPeriode { original: String, actual: String },
    MaMandant { original: String, actual: String },
    MaParameter { original: String, actual: String },
    SbEreignis { original: String, actual: String },
    SbFamilie { original: String, actual: String },
    SbKind { original: String, actual: String },
    SbPerson { original: String, actual: String },
    SbQuelle { original: String, actual: String },
    SoKurse { original: String, actual: String },
    TbEintrag { original: String, actual: String },
    TbEintragOrt { original: String, actual: String },
    TbOrt { original: String, actual: String },
    WpAnlage { original: String, actual: String },
    WpBuchung { original: String, actual: String },
    WpKonfiguration { original: String, actual: String },
    WpStand { original: String, actual: String },
    WpWertpapier { original: String, actual: String },
}

#[allow(dead_code)]
impl UndoEntry {
    pub fn ad_adresse(original: Option<&AdAdresse>, actual: Option<&AdAdresse>) -> Self {
        UndoEntry::AdAdresse {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn ad_person(original: Option<&AdPerson>, actual: Option<&AdPerson>) -> Self {
        UndoEntry::AdPerson {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn ad_sitz(original: Option<&AdSitz>, actual: Option<&AdSitz>) -> Self {
        UndoEntry::AdSitz {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn benutzer(original: Option<&Benutzer>, actual: Option<&Benutzer>) -> Self {
        UndoEntry::Benutzer {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn byte_daten(original: Option<&ByteDaten>, actual: Option<&ByteDaten>) -> Self {
        UndoEntry::ByteDaten {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_buch(original: Option<&FzBuch>, actual: Option<&FzBuch>) -> Self {
        UndoEntry::FzBuch {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_buchautor(original: Option<&FzBuchautor>, actual: Option<&FzBuchautor>) -> Self {
        UndoEntry::FzBuchautor {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_buchserie(original: Option<&FzBuchserie>, actual: Option<&FzBuchserie>) -> Self {
        UndoEntry::FzBuchserie {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_buchstatus(original: Option<&FzBuchstatus>, actual: Option<&FzBuchstatus>) -> Self {
        UndoEntry::FzBuchstatus {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_fahrrad(original: Option<&FzFahrrad>, actual: Option<&FzFahrrad>) -> Self {
        UndoEntry::FzFahrrad {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_fahrradstand(
        original: Option<&FzFahrradstand>,
        actual: Option<&FzFahrradstand>,
    ) -> Self {
        UndoEntry::FzFahrradstand {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn fz_notiz(original: Option<&FzNotiz>, actual: Option<&FzNotiz>) -> Self {
        UndoEntry::FzNotiz {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn hh_bilanz(original: Option<&HhBilanz>, actual: Option<&HhBilanz>) -> Self {
        UndoEntry::HhBilanz {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn hh_buchung(original: Option<&HhBuchung>, actual: Option<&HhBuchung>) -> Self {
        UndoEntry::HhBuchung {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn hh_ereignis(original: Option<&HhEreignis>, actual: Option<&HhEreignis>) -> Self {
        UndoEntry::HhEreignis {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn hh_konto(original: Option<&HhKonto>, actual: Option<&HhKonto>) -> Self {
        UndoEntry::HhKonto {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn hh_periode(original: Option<&HhPeriode>, actual: Option<&HhPeriode>) -> Self {
        UndoEntry::HhPeriode {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn ma_mandant(original: Option<&MaMandant>, actual: Option<&MaMandant>) -> Self {
        UndoEntry::MaMandant {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn ma_parameter(original: Option<&MaParameter>, actual: Option<&MaParameter>) -> Self {
        UndoEntry::MaParameter {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn sb_ereignis(original: Option<&SbEreignis>, actual: Option<&SbEreignis>) -> Self {
        UndoEntry::SbEreignis {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn sb_familie(original: Option<&SbFamilie>, actual: Option<&SbFamilie>) -> Self {
        UndoEntry::SbFamilie {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn sb_kind(original: Option<&SbKind>, actual: Option<&SbKind>) -> Self {
        UndoEntry::SbKind {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn sb_person(original: Option<&SbPerson>, actual: Option<&SbPerson>) -> Self {
        UndoEntry::SbPerson {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn sb_quelle(original: Option<&SbQuelle>, actual: Option<&SbQuelle>) -> Self {
        UndoEntry::SbQuelle {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn so_kurse(original: Option<&SoKurse>, actual: Option<&SoKurse>) -> Self {
        UndoEntry::SoKurse {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn tb_eintrag(original: Option<&TbEintrag>, actual: Option<&TbEintrag>) -> Self {
        UndoEntry::TbEintrag {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn tb_eintrag_ort(original: Option<&TbEintragOrt>, actual: Option<&TbEintragOrt>) -> Self {
        UndoEntry::TbEintragOrt {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn tb_ort(original: Option<&TbOrt>, actual: Option<&TbOrt>) -> Self {
        UndoEntry::TbOrt {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn wp_anlage(original: Option<&WpAnlage>, actual: Option<&WpAnlage>) -> Self {
        UndoEntry::WpAnlage {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn wp_buchung(original: Option<&WpBuchung>, actual: Option<&WpBuchung>) -> Self {
        UndoEntry::WpBuchung {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn wp_konfiguration(
        original: Option<&WpKonfiguration>,
        actual: Option<&WpKonfiguration>,
    ) -> Self {
        UndoEntry::WpKonfiguration {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn wp_stand(original: Option<&WpStand>, actual: Option<&WpStand>) -> Self {
        UndoEntry::WpStand {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }
    pub fn wp_wertpapier(original: Option<&WpWertpapier>, actual: Option<&WpWertpapier>) -> Self {
        UndoEntry::WpWertpapier {
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }
    }

    fn to_string<T>(ser: Option<&T>) -> String
    where
        T: ?Sized + Serialize,
    {
        let mut o = String::new();
        if let Some(e) = ser {
            o = serde_json::to_string(e).unwrap_or(o);
        }
        o
    }

    pub fn from_str<'a, T>(s: &'a String) -> Result<Option<T>>
    where
        T: Deserialize<'a>,
    {
        if s.is_empty() {
            return Ok(None);
        }
        let e: T = serde_json::from_str::<'a, T>(s.as_str()).unwrap();
        Ok(Some(e))
    }
}

#[derive(Debug)]
pub struct UndoList {
    list: Vec<UndoEntry>,
}

impl UndoList {
    pub fn new() -> Self {
        return UndoList { list: Vec::new() };
    }

    pub fn add(&mut self, e: &UndoEntry) {
        self.list.push(e.clone());
    }

    pub fn is_empty(&self) -> bool {
        self.list.len() <= 0
    }

    pub fn clone(&self) -> Arc<UndoList> {
        let mut aul = UndoList::new();
        for e in self.list.iter() {
            aul.add(&e.clone());
        }
        Arc::new(aul)
    }
}

lazy_static! {
    static ref UNDO_STACK: Arc<RwLock<UndoRedoStack>> = Arc::new(RwLock::new(UndoRedoStack::new()));
}

#[derive(Clone, Debug)]
pub struct UndoRedoStack {
    undo: Vec<Arc<UndoList>>,
    redo: Vec<Arc<UndoList>>,
}

impl UndoRedoStack {
    pub fn new() -> Self {
        return UndoRedoStack {
            undo: Vec::new(),
            redo: Vec::new(),
        };
    }

    /// UndoList zum Stack nach Commit hinzufügen.
    pub fn add_undo(ul: &mut UndoList) {
        if ul.is_empty() {
            return;
        }
        {
            let mut guard = match UNDO_STACK.write() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            (*guard).undo.push(ul.clone());
            (*guard).redo.clear(); // Alle Redos sind durch das neue Commit ungültig.
            if cfg!(debug_assertions) {
                println!(
                    "undo: {}  redo: {}",
                    (*guard).undo.len(),
                    (*guard).redo.len()
                )
            }
        }
    }

    fn remove_undo(&mut self, ul: &UndoList) {
        let li = self.undo.len() - 1;
        self.undo.remove(li);
        self.redo.push(ul.clone());
    }

    fn remove_redo(&mut self, ul: &UndoList) {
        let li = self.redo.len() - 1;
        self.redo.remove(li);
        self.undo.push(ul.clone());
    }

    /// Eine Transaktion zurücksetzen.
    /// * db: Kontext für Datenbank-Zugriff.
    /// * returns: Wurde etwas geändert?
    #[allow(unused_variables)]
    pub fn undo(db: &mut DbContext) -> Result<bool> {
        let mut guard = match UNDO_STACK.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let mut r = false;
        if let Some(ul0) = (*guard).undo.last() {
            let ul = &ul0.clone();
            for e in ul.list.iter() {
                //println!("e: {:?}", e);
                match e {
                    UndoEntry::AdAdresse { original, actual } => {
                        // TODO reps::ad_adresse::undo(db, original, actual)?;
                    }
                    UndoEntry::AdPerson { original, actual } => {
                        reps::ad_person::undo(db, original, actual)?;
                    }
                    UndoEntry::AdSitz { original, actual } => {
                        // reps::ad_sitz::undo(db, original, actual)?;
                    }
                    UndoEntry::Benutzer { original, actual } => {
                        reps::benutzer::undo(db, original, actual)?;
                    }
                    UndoEntry::ByteDaten { original, actual } => {
                        // reps::byte_daten::undo(db, original, actual)?;
                    }
                    UndoEntry::FzBuch { original, actual } => {
                        // reps::fz_buch::undo(db, original, actual)?;
                    }
                    UndoEntry::FzBuchautor { original, actual } => {
                        // reps::fz_buchautor::undo(db, original, actual)?;
                    }
                    UndoEntry::FzBuchserie { original, actual } => {
                        // reps::fz_buchserie::undo(db, original, actual)?;
                    }
                    UndoEntry::FzBuchstatus { original, actual } => {
                        // reps::fz_buchstatus::undo(db, original, actual)?;
                    }
                    UndoEntry::FzFahrrad { original, actual } => {
                        // reps::fz_fahrrad::undo(db, original, actual)?;
                    }
                    UndoEntry::FzFahrradstand { original, actual } => {
                        // reps::fz_fahrradstand::undo(db, original, actual)?;
                    }
                    UndoEntry::FzNotiz { original, actual } => {
                        // reps::fz_notiz::undo(db, original, actual)?;
                    }
                    UndoEntry::HhBilanz { original, actual } => {
                        // reps::hh_bilanz::undo(db, original, actual)?;
                    }
                    UndoEntry::HhBuchung { original, actual } => {
                        // reps::hh_buchung::undo(db, original, actual)?;
                    }
                    UndoEntry::HhEreignis { original, actual } => {
                        // reps::hh_ereignis::undo(db, original, actual)?;
                    }
                    UndoEntry::HhKonto { original, actual } => {
                        // reps::hh_konto::undo(db, original, actual)?;
                    }
                    UndoEntry::HhPeriode { original, actual } => {
                        // reps::hh_periode::undo(db, original, actual)?;
                    }
                    UndoEntry::MaMandant { original, actual } => {
                        reps::ma_mandant::undo(db, original, actual)?;
                    }
                    UndoEntry::MaParameter { original, actual } => {
                        reps::ma_parameter::undo(db, original, actual)?;
                    }
                    UndoEntry::SbEreignis { original, actual } => {
                        // reps::sb_ereignis::undo(db, original, actual)?;
                    }
                    UndoEntry::SbFamilie { original, actual } => {
                        // reps::sb_familie::undo(db, original, actual)?;
                    }
                    UndoEntry::SbKind { original, actual } => {
                        // reps::sb_kind::undo(db, original, actual)?;
                    }
                    UndoEntry::SbPerson { original, actual } => {
                        // reps::sb_person::undo(db, original, actual)?;
                    }
                    UndoEntry::SbQuelle { original, actual } => {
                        // reps::sb_quelle::undo(db, original, actual)?;
                    }
                    UndoEntry::SoKurse { original, actual } => {
                        // reps::so_kurse::undo(db, original, actual)?;
                    }
                    UndoEntry::TbEintrag { original, actual } => {
                        reps::tb_eintrag::undo(db, original, actual)?;
                    }
                    UndoEntry::TbEintragOrt { original, actual } => {
                        reps::tb_eintrag_ort::undo(db, original, actual)?;
                    }
                    UndoEntry::TbOrt { original, actual } => {
                        reps::tb_ort::undo(db, original, actual)?;
                    }
                    UndoEntry::WpAnlage { original, actual } => {
                        // reps::wp_anlage::undo(db, original, actual)?;
                    }
                    UndoEntry::WpBuchung { original, actual } => {
                        // reps::wp_buchung::undo(db, original, actual)?;
                    }
                    UndoEntry::WpKonfiguration { original, actual } => {
                        // reps::wp_konfiguration::undo(db, original, actual)?;
                    }
                    UndoEntry::WpStand { original, actual } => {
                        // reps::wp_stand::undo(db, original, actual)?;
                    }
                    UndoEntry::WpWertpapier { original, actual } => {
                        // reps::wp_wertpapier::undo(db, original, actual)?;
                    }
                };
                functions::mach_nichts();
            }
            (*guard).remove_undo(&ul);
            r = true;
        }
        if cfg!(debug_assertions) {
            println!(
                "undo: {}  redo: {}",
                (*guard).undo.len(),
                (*guard).redo.len()
            )
        }
        Ok(r)
    }

    /// Eine Transaktion wiederherstellen.
    /// * db: Kontext für Datenbank-Zugriff.
    /// * returns: Wurde etwas geändert?
    #[allow(unused_variables)]
    pub fn redo(db: &mut DbContext) -> Result<bool> {
        let mut guard = match UNDO_STACK.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let mut r = false;
        if let Some(ul0) = (*guard).redo.last() {
            let ul = &ul0.clone();
            for e in ul.list.iter() {
                //println!("e: {:?}", e);
                match e {
                    UndoEntry::AdAdresse { original, actual } => {
                        // TODO reps::ad_adresse::redo(db, original, actual)?;
                    }
                    UndoEntry::AdPerson { original, actual } => {
                        reps::ad_person::redo(db, original, actual)?;
                    }
                    UndoEntry::AdSitz { original, actual } => {
                        // reps::ad_sitz::redo(db, original, actual)?;
                    }
                    UndoEntry::Benutzer { original, actual } => {
                        reps::benutzer::redo(db, original, actual)?;
                    }
                    UndoEntry::ByteDaten { original, actual } => {
                        // reps::byte_daten::redo(db, original, actual)?;
                    }
                    UndoEntry::FzBuch { original, actual } => {
                        // reps::fz_buch::redo(db, original, actual)?;
                    }
                    UndoEntry::FzBuchautor { original, actual } => {
                        // reps::fz_buchautor::redo(db, original, actual)?;
                    }
                    UndoEntry::FzBuchserie { original, actual } => {
                        // reps::fz_buchserie::redo(db, original, actual)?;
                    }
                    UndoEntry::FzBuchstatus { original, actual } => {
                        // reps::fz_buchstatus::redo(db, original, actual)?;
                    }
                    UndoEntry::FzFahrrad { original, actual } => {
                        // reps::fz_fahrrad::redo(db, original, actual)?;
                    }
                    UndoEntry::FzFahrradstand { original, actual } => {
                        // reps::fz_fahrradstand::redo(db, original, actual)?;
                    }
                    UndoEntry::FzNotiz { original, actual } => {
                        // reps::fz_notiz::redo(db, original, actual)?;
                    }
                    UndoEntry::HhBilanz { original, actual } => {
                        // reps::hh_bilanz::redo(db, original, actual)?;
                    }
                    UndoEntry::HhBuchung { original, actual } => {
                        // reps::hh_buchung::redo(db, original, actual)?;
                    }
                    UndoEntry::HhEreignis { original, actual } => {
                        // reps::hh_ereignis::redo(db, original, actual)?;
                    }
                    UndoEntry::HhKonto { original, actual } => {
                        // reps::hh_konto::redo(db, original, actual)?;
                    }
                    UndoEntry::HhPeriode { original, actual } => {
                        // reps::hh_periode::redo(db, original, actual)?;
                    }
                    UndoEntry::MaMandant { original, actual } => {
                        reps::ma_mandant::redo(db, original, actual)?;
                    }
                    UndoEntry::MaParameter { original, actual } => {
                        reps::ma_parameter::redo(db, original, actual)?;
                    }
                    UndoEntry::SbEreignis { original, actual } => {
                        // reps::sb_ereignis::redo(db, original, actual)?;
                    }
                    UndoEntry::SbFamilie { original, actual } => {
                        // reps::sb_familie::redo(db, original, actual)?;
                    }
                    UndoEntry::SbKind { original, actual } => {
                        // reps::sb_kind::redo(db, original, actual)?;
                    }
                    UndoEntry::SbPerson { original, actual } => {
                        // reps::sb_person::redo(db, original, actual)?;
                    }
                    UndoEntry::SbQuelle { original, actual } => {
                        // reps::sb_quelle::redo(db, original, actual)?;
                    }
                    UndoEntry::SoKurse { original, actual } => {
                        // reps::so_kurse::redo(db, original, actual)?;
                    }
                    UndoEntry::TbEintrag { original, actual } => {
                        reps::tb_eintrag::redo(db, original, actual)?;
                    }
                    UndoEntry::TbEintragOrt { original, actual } => {
                        reps::tb_eintrag_ort::redo(db, original, actual)?;
                    }
                    UndoEntry::TbOrt { original, actual } => {
                        reps::tb_ort::redo(db, original, actual)?;
                    }
                    UndoEntry::WpAnlage { original, actual } => {
                        // reps::wp_anlage::redo(db, original, actual)?;
                    }
                    UndoEntry::WpBuchung { original, actual } => {
                        // reps::wp_buchung::redo(db, original, actual)?;
                    }
                    UndoEntry::WpKonfiguration { original, actual } => {
                        // reps::wp_konfiguration::redo(db, original, actual)?;
                    }
                    UndoEntry::WpStand { original, actual } => {
                        // reps::wp_stand::redo(db, original, actual)?;
                    }
                    UndoEntry::WpWertpapier { original, actual } => {
                        // reps::wp_wertpapier::redo(db, original, actual)?;
                    }
                };
            }
            (*guard).remove_redo(&ul);
            r = true;
        }
        if cfg!(debug_assertions) {
            println!(
                "undo: {}  redo: {}",
                (*guard).undo.len(),
                (*guard).redo.len()
            )
        }
        Ok(r)
    }
}
