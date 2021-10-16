use super::{
    reps::{self, DbContext},
    undo::UndoRedoStack,
};
use crate::{
    apis::services::ServiceDaten,
    base::{functions, parameter::Parameter},
    config::RsbpError,
    res::{
        self,
        messages::{Messages, M},
    },
    Result,
};
use diesel::prelude::*;
use log::error;
use rsbp_rep::models::MaParameter;
use std::collections::HashMap;

/// Anmeldung prüfen und durchführen.
/// * daten: Service-Daten mit betroffener Benutzer-ID für Datenbank-Zugriff.
/// * password: Zu prüfendes Kennwort.
/// * save: Soll der Benutzer für automatische Anmeldung gespeichert werden?
/// * returns: Genaue Benutzer-ID.
pub fn login<'a>(daten: &'a ServiceDaten, password: &'a str, save: bool) -> Result<String> {
    let mut r: Vec<String> = vec![];
    if daten.mandant_nr <= 0 {
        // Die Anmeldedaten sind ungültig. Mandant ungültig.
        //   throw new MessageException(AM001);
        r.push(Messages::mec(M::AM001, daten.config.is_de()).into_owned());
        return Err(RsbpError::error(&r));
    }
    if daten.benutzer_id.is_empty() {
        // Die Anmeldedaten sind ungültig. Benutzer ungültig.
        //   throw new MessageException(AM001);
        r.push(Messages::msc("AM001", daten.config.is_de()).into_owned());
        return Err(RsbpError::error(&r));
    }

    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<String, RsbpError, _>(|| {
        // use crate::schema::BENUTZER::dsl::*;
        // let updated_row = diesel::update(BENUTZER.filter(benutzer_id.eq("wolfgang")))
        //     .set(benutzer_id.eq("Wolfgang"))
        //     .execute(&c)?;
        let benutzer1 = reps::benutzer::get(&db, &daten.mandant_nr, &daten.benutzer_id)?;
        if let Some(benutzer) = benutzer1 {
            // Benutzer vorhanden.
            let id = benutzer.benutzer_id.as_str();
            error!("{}", M::am003(daten.mandant_nr, id, daten.config.is_de()));

            let wert = get_without_login(&db).unwrap_or("".into());
            if !wert.is_empty() && functions::cmp(wert.as_str(), id) {
                // Anmeldung ohne Kennwort
                save_login(&mut db, daten.mandant_nr, id, save)?;
                return Ok(id.into());
            }
            if functions::is_empty(&benutzer.passwort) && password.is_empty() {
                // Anmeldung mit leerem Kennwort
                save_login(&mut db, daten.mandant_nr, id, save)?;
                return Ok(id.into());
            }
            if !functions::is_empty(&benutzer.passwort) && !password.is_empty() {
                // Anmeldung mit Kennwort
                if benutzer.passwort == Some(password.to_string()) {
                    save_login(&mut db, daten.mandant_nr, id, save)?;
                    return Ok(id.into());
                }
            }
            // Err(RsbpError::ConfigError)
        }

        // Anzumeldenden Benutzer als Benutzer eintragen
        let liste = reps::benutzer::get_list(&db, daten.mandant_nr)?;
        if let Some(b1) = liste.first() {
            if liste.len() == 1 && functions::cmp(res::USER_ID, b1.benutzer_id.as_str())
            //   && !string.Equals(daten.BenutzerId, Constants.USER_ID, StringComparison.InvariantCultureIgnoreCase))
            {
                reps::benutzer::delete(&mut db, b1)?;
                //   BenutzerRep.Save(daten, daten.MandantNr, daten.BenutzerId, kennwort, liste[0].Berechtigung, liste[0].Akt_Periode,
                //     liste[0].Person_Nr, liste[0].Geburt, liste[0].Angelegt_Von, liste[0].Angelegt_Am, daten.BenutzerId, daten.Jetzt);
                let mut b2 = (*b1).clone();
                b2.benutzer_id = daten.benutzer_id.clone();
                b2.passwort = Some(password.to_string());
                b2.geaendert_von = Some(daten.benutzer_id.clone());
                b2.geaendert_am = Some(daten.get_now());
                reps::benutzer::insert(&mut db, &b2)?;
                save_login(&mut db, daten.mandant_nr, daten.benutzer_id.as_str(), save)?;
                return Ok(daten.benutzer_id.to_string());
            }
        }

        // Die Anmeldedaten sind ungültig. Mandant, Benutzer oder Kennwort ungültig.
        r.push(Messages::mec(M::AM001, daten.config.is_de()).into_owned());
        //r.push(Messages::mec(M::AM002, daten.config.is_de()).into_owned());
        return Err(RsbpError::error(&r));
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }
    tr
}

/// Liefert Liste von Parametern.
/// * daten: ServiceDaten mit betroffener Benutzer-ID für Datenbank-Zugriff.
/// * m_nr: Betroffene Mandantennummer.
/// * returns: Liste von Parametern.
pub fn get_list_param<'a>(daten: &'a ServiceDaten, m_nr: i32) -> Result<Vec<MaParameter>> {
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    let list = reps::ma_parameter::get_list(&db, m_nr);
    list
}

/// Letzte Transaktion rückgängig machen.
/// * daten: ServiceDaten für Datenbank-Zugriff.
/// * returns: Wurde etwas geändert?
pub fn undo<'a>(daten: &'a ServiceDaten) -> Result<bool> {
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<bool, RsbpError, _>(|| {
        UndoRedoStack::undo(&mut db)
        // if functions::mach_nichts() == 0 {
        //     return Err(RsbpError::error_msg(M::MIMPL, daten.config.is_de()));
        // }
        // Ok(true)
    });
    tr
}

/// Letzte Transaktion wieder durchführen.
/// * daten: ServiceDaten für Datenbank-Zugriff.
/// * returns: Wurde etwas geändert?
pub fn redo<'a>(daten: &'a ServiceDaten) -> Result<bool> {
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<bool, RsbpError, _>(|| UndoRedoStack::redo(&mut db));
    tr
}

/// Is login wihtout password?
/// * daten: Service data for database access.
/// * returns: Is login wihtout password?
pub fn is_without_password(daten: &ServiceDaten) -> Result<bool> {
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    let user_id = daten.benutzer_id.as_str();
    Ok(functions::cmpo(&get_without_login(&db), user_id))
}

/// Get user id which do not need a login.
/// * db: Kontext für Datenbank-Zugriff.
/// * returns: user id which do not need a login.
fn get_without_login(db: &DbContext) -> Option<String> {
    let mp = reps::ma_parameter::get(
        db,
        &db.daten.mandant_nr,
        &res::EINST_MA_OHNE_ANMELDUNG.to_string(),
    )
    .unwrap_or(None);
    if let Some(p) = mp {
        return p.wert;
    }
    None
}

/// Speichern der Anmelde-Daten.
/// * db: Kontext für Datenbank-Zugriff.
/// * m_nr: Betroffener Mandant.
/// * b_id: Betroffener Benutzer-ID.
/// * save: Soll der automatisch anzumeldende Benutzer gespeichert werden?
fn save_login(db: &mut DbContext, m_nr: i32, b_id: &str, save: bool) -> Result<()> {
    let mut wert: Option<String> = Some("".into());
    let mut op = reps::ma_parameter::get(db, &m_nr, &res::EINST_MA_OHNE_ANMELDUNG.to_string())?;
    if let Some(ref p) = op {
        wert = p.wert.clone();
    } else {
        let p =
            reps::ma_parameter::save(db, &m_nr, &res::EINST_MA_OHNE_ANMELDUNG.to_string(), &wert)?;
        op = Some(p);
    }
    if !save && !functions::is_empty(&wert) && functions::cmpo(&wert, b_id) {
        let mut p = op.unwrap();
        p.wert = Some("".into());
        p.geaendert_von = Some(db.daten.benutzer_id.clone());
        p.geaendert_am = Some(db.daten.get_now());
        reps::ma_parameter::update(db, &p)?;
    } else if save && (functions::is_empty(&wert) || !functions::cmpo(&wert, b_id)) {
        let mut p = op.unwrap();
        p.wert = Some(b_id.to_string());
        p.geaendert_von = Some(db.daten.benutzer_id.clone());
        p.geaendert_am = Some(db.daten.get_now());
        reps::ma_parameter::update(db, &p)?;
    }
    init_mandant(db)
}

/// Initialisierung der Mandanten-Daten.
/// * db: Kontext für Datenbank-Zugriff.
fn init_mandant(db: &mut DbContext) -> Result<()> {
    // Parameter REPLIKATION_UID initialisieren.
    let mut op = reps::ma_parameter::get(
        db,
        &db.daten.mandant_nr,
        &res::EINST_MA_REPLIKATION_UID.to_string(),
    )?;
    let mut wert: Option<String> = None;
    if let Some(ref p) = op {
        wert = p.wert.clone();
        if functions::cmpo(&wert, "") {
            wert = None;
        }
    }
    if wert.is_none() {
        reps::ma_parameter::save(
            db,
            &db.daten.mandant_nr,
            &res::EINST_MA_REPLIKATION_UID.to_string(),
            &Some(functions::get_uid()),
        )?;
    }
    // Parameter löschen: AG_BACKUPS
    op = reps::ma_parameter::get(db, &db.daten.mandant_nr, &"AG_BACKUPS".to_string())?;
    if let Some(ref p) = op {
        reps::ma_parameter::delete(db, p)?;
    }
    // Parameter löschen: ANWENDUNGS_TITEL
    op = reps::ma_parameter::get(db, &db.daten.mandant_nr, &"ANWENDUNGS_TITEL".to_string())?;
    if let Some(ref p) = op {
        reps::ma_parameter::delete(db, p)?;
    }
    Ok(())
}

/// Speichern der Parameter in der Datenbank.
/// * daten: Service-Daten für Datenbank-Zugriff.
/// * password: Zu prüfendes Kennwort.
/// * returns: nichts.
pub fn save_parameter<'a>(
    daten: &'a ServiceDaten,
    params: &'a HashMap<&'static str, Parameter>,
) -> Result<()> {
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<(), RsbpError, _>(|| {
        for p in params.iter() {
            if p.1.is_database() {
                reps::ma_parameter::save(
                    &mut db,
                    &p.1.get_client(daten.mandant_nr),
                    &p.0.to_string(),
                    &p.1.get_value0(),
                )?;
            }
        }
        Ok(())
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }
    tr
}
