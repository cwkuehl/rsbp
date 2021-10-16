use super::{
    reps::{self, DbContext},
    undo::UndoRedoStack,
};
use crate::{apis::services::ServiceDaten, base::functions, config::RsbpError, Result};
use chrono::{Datelike, NaiveDate};
use diesel::Connection;
use rsbp_rep::{
    models::{TbEintrag, TbOrt},
    models_ext::TbEintragOrtExt,
};

/// Get a diary entry.
/// * daten: Service data for database access.
/// * date: Affected date.
/// * returns: Diary entry or possibly errors.
pub fn get_entry<'a>(daten: &'a ServiceDaten, date: &NaiveDate) -> Result<Option<TbEintrag>> {
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    let e = reps::tb_eintrag::get(&db, &daten.mandant_nr, date)?;
    Ok(e)
}

/// Get a position list for a date.
/// * daten: Service data for database access.
/// * date: Affected date.
/// * returns: Position list or possibly errors.
pub fn get_entry_position_list<'a>(
    daten: &'a ServiceDaten,
    date: &NaiveDate,
) -> Result<Option<Vec<TbEintragOrtExt>>> {
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    let e = reps::tb_eintrag_ort::get_list_ext(&db, &daten.mandant_nr, date)?;
    Ok(Some(e))
}

/// Get a bool array of a month if there is an entry.
/// * daten: Service data for database access.
/// * date: Affected date.
/// * returns: Bool array or possibly errors.
pub fn get_month<'a>(daten: &'a ServiceDaten, date: &NaiveDate) -> Result<Vec<bool>> {
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    if let (Some(from), Some(to)) = (
        functions::nd_add_dmy(date, -(date.day() as i32) + 1, 0, 0),
        functions::nd_add_dmy(date, -(date.day() as i32) + 1, 1, 0),
    ) {
        // println!("get_month {:?} from {:?} to {:?}", date, from, to);
        let mut v = Vec::<bool>::new();
        let l = reps::tb_eintrag::get_list_ext(&db, &daten.mandant_nr, &from, &to)?;
        for e in l {
            while v.len() < e.datum.day() as usize - 1 {
                v.push(false);
            }
            v.push(true);
        }
        while v.len() < 31 {
            v.push(false);
        }
        return Ok(v);
    }
    Err(RsbpError::NotFound)
}

/// Save a diary entry.
/// * daten: Service data for database access.
/// * date: Affected date.
/// * entry: Affected text entry.
/// * returns: Possibly errors.
pub fn save_entry<'a>(daten: &'a ServiceDaten, date: &NaiveDate, entry: &String) -> Result<()> {
    let e = entry.trim();
    let empty = e.is_empty();
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<(), RsbpError, _>(|| {
        if let Some(ref mut tb) = reps::tb_eintrag::get(&db, &daten.mandant_nr, date)? {
            if empty {
                // Delete empty entry and leave positions.
                reps::tb_eintrag::delete(&mut db, tb)?;
            } else if e != tb.eintrag {
                if tb.replikation_uid.is_none() {
                    tb.replikation_uid = Some(functions::get_uid());
                }
                tb.eintrag = e.to_string();
                tb.geaendert_am = Some(daten.get_now());
                tb.geaendert_von = Some(daten.benutzer_id.to_string());
                reps::tb_eintrag::update(&mut db, tb)?;
            }
        } else if !empty {
            let tb = TbEintrag {
                mandant_nr: daten.mandant_nr,
                datum: date.clone(),
                eintrag: e.to_string(),
                angelegt_am: Some(daten.get_now()),
                angelegt_von: Some(daten.benutzer_id.to_string()),
                geaendert_am: None,
                geaendert_von: None,
                replikation_uid: Some(functions::get_uid()),
            };
            reps::tb_eintrag::insert(&mut db, &tb)?;
        }
        // TODO Save positions.
        Ok(())
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }
    tr
}

/// Get a list of positions.
/// * daten: Service data for database access.
/// * puid: Affected position ID.
/// * text: Affected text.
/// * returns: Diary entry or possibly errors.
pub fn get_position_list<'a>(
    daten: &'a ServiceDaten,
    puid: &Option<String>,
    text: &Option<String>,
) -> Result<Vec<TbOrt>> {
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    let e = reps::tb_ort::get_list_ext(&db, &daten.mandant_nr, puid, text)?;
    Ok(e)
}
