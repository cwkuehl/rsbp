use super::{
    reps::{self, DbContext},
    undo::UndoRedoStack,
};
use crate::{
    apis::{enums::SearchDirectionEnum, services::ServiceDaten},
    base::functions,
    config::RsbpError,
    Result,
};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
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
    let e = reps::tb_eintrag_ort::get_list_ext2(&db, date)?;
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
/// * pos: Affected position list.
/// * returns: Possibly errors.
pub fn save_entry<'a>(
    daten: &'a ServiceDaten,
    date: &NaiveDate,
    entry: &String,
    pos: &Vec<TbEintragOrtExt>,
) -> Result<()> {
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
        // Save positions.
        // bestehende Orte lesen
        let mut liste = reps::tb_eintrag_ort::get_list_ext(&db, date, &0, None, None)?;
        for i in pos {
            let puid = i.ort_uid.clone();
            let mut from = i.datum_von;
            let mut to = i.datum_bis;
            if to < from {
                to = from;
            }
            if *date < from || *date > to {
                from = *date;
                to = *date;
            }
            let listep =
                reps::tb_eintrag_ort::get_list_ext(&db, &from, &0, Some(&to), Some(&puid))?;
            let ovop = listep.first();
            if let Some(p) = liste.iter().position(|a| a.ort_uid == puid) {
                liste.remove(p); // nicht mehr löschen
            }
            if listep.is_empty() || ovop.is_none() {
                // Zeitraum leer
                optimize_positions(&mut db, &puid, &from, &to, &None, &None)?;
            } else if let Some(vop) = ovop {
                if listep.len() == 1 {
                    let mfrom = functions::min_date(&vop.datum_von, &from);
                    let mto = functions::max_date(&vop.datum_bis, &to);
                    if !(vop.datum_von == mfrom && vop.datum_bis == mto) {
                        // Maximaler Zeitraum
                        optimize_positions(
                            &mut db,
                            &puid,
                            &mfrom,
                            &mto,
                            &vop.angelegt_von,
                            &vop.angelegt_am,
                        )?;
                        reps::tb_eintrag_ort::delete(&mut db, vop)?;
                    }
                } else {
                    // listep.Count >= 1
                    let mut mfrom = from;
                    let mut mto = to;
                    for p in listep.iter() {
                        if p.datum_von < mfrom {
                            mfrom = p.datum_von;
                        }
                        if p.datum_bis > mto {
                            mto = p.datum_bis;
                        }
                        reps::tb_eintrag_ort::delete(&mut db, p)?;
                    }
                    // Maximaler Zeitraum
                    optimize_positions(
                        &mut db,
                        &puid,
                        &mfrom,
                        &mto,
                        &vop.angelegt_von,
                        &vop.angelegt_am,
                    )?;
                }
            }
        }
        // überflüssige Orte löschen.
        for vo in liste {
            if vo.datum_von == vo.datum_bis {
                reps::tb_eintrag_ort::delete(&mut db, &vo)?; // Eintrag löschen
            } else if vo.datum_von == *date {
                // Einen Tag vorne verkürzen
                if let Some(d) = functions::nd_add_dmy(date, 1, 0, 0) {
                    reps::tb_eintrag_ort::save0(
                        &mut db,
                        &daten.mandant_nr,
                        &vo.ort_uid,
                        &d,
                        &vo.datum_bis,
                        &vo.angelegt_von,
                        &vo.angelegt_am,
                        &None,
                        &None,
                    )?;
                    reps::tb_eintrag_ort::delete(&mut db, &vo)?;
                }
            } else if vo.datum_bis == *date {
                // Einen Tag hinten verkürzen
                if let Some(d) = functions::nd_add_dmy(date, -1, 0, 0) {
                    reps::tb_eintrag_ort::save0(
                        &mut db,
                        &daten.mandant_nr,
                        &vo.ort_uid,
                        &vo.datum_von,
                        &d,
                        &vo.angelegt_von,
                        &vo.angelegt_am,
                        &None,
                        &None,
                    )?;
                    reps::tb_eintrag_ort::delete(&mut db, &vo)?;
                }
            } else {
                // Einen Tag herausschneiden
                if let (Some(dp), Some(dm)) = (
                    functions::nd_add_dmy(date, 1, 0, 0),
                    functions::nd_add_dmy(date, -1, 0, 0),
                ) {
                    reps::tb_eintrag_ort::save0(
                        &mut db,
                        &daten.mandant_nr,
                        &vo.ort_uid,
                        &vo.datum_von,
                        &dm,
                        &vo.angelegt_von,
                        &vo.angelegt_am,
                        &None,
                        &None,
                    )?;
                    reps::tb_eintrag_ort::save0(
                        &mut db,
                        &daten.mandant_nr,
                        &vo.ort_uid,
                        &dp,
                        &vo.datum_bis,
                        &vo.angelegt_von,
                        &vo.angelegt_am,
                        &None,
                        &None,
                    )?;
                    reps::tb_eintrag_ort::delete(&mut db, &vo)?;
                }
            }
        }
        Ok(())
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }
    tr
}

/// Optimieren der Positionen, d.h. verlängern oder Lücke füllen.
/// * daten: Service data for database access.
/// * puid: Affected position ID.
/// * from: Affected from date.
/// * to: Affected to date.
/// * created_by: Affected creation user id.
/// * created_at: Affection creation time.
fn optimize_positions<'a>(
    db: &'a mut DbContext,
    puid: &String,
    from: &NaiveDate,
    to: &NaiveDate,
    created_by: &Option<String>,
    created_at: &Option<NaiveDateTime>,
) -> Result<()> {
    let listeb = reps::tb_eintrag_ort::get_list_ext(&db, &from, &-1, None, Some(&puid))?;
    let listea = reps::tb_eintrag_ort::get_list_ext(&db, &to, &1, None, Some(&puid))?;
    let obef = listeb.first();
    let oaft = listea.first();

    if let Some(bef) = obef {
        if let Some(aft) = oaft {
            // Lücke füllen
            reps::tb_eintrag_ort::save0(
                db,
                &db.daten.mandant_nr,
                &puid,
                &bef.datum_von,
                &aft.datum_bis,
                &bef.angelegt_von,
                &bef.angelegt_am,
                &None,
                &None,
            )?;
            reps::tb_eintrag_ort::delete(db, bef)?;
            reps::tb_eintrag_ort::delete(db, aft)?;
        } else {
            // Zeitraum hinten anhängen
            reps::tb_eintrag_ort::save0(
                db,
                &db.daten.mandant_nr,
                &puid,
                &bef.datum_von,
                to,
                &bef.angelegt_von,
                &bef.angelegt_am,
                &None,
                &None,
            )?;
            reps::tb_eintrag_ort::delete(db, bef)?;
        }
    } else if let Some(aft) = oaft {
        // Zeitraum vorne anhängen
        reps::tb_eintrag_ort::save0(
            db,
            &db.daten.mandant_nr,
            &puid,
            from,
            &aft.datum_bis,
            &aft.angelegt_von,
            &aft.angelegt_am,
            &None,
            &None,
        )?;
        reps::tb_eintrag_ort::delete(db, aft)?;
    } else {
        // Neu
        reps::tb_eintrag_ort::save0(
            db,
            &db.daten.mandant_nr,
            &puid,
            from,
            to,
            created_by,
            created_at,
            &None,
            &None,
        )?;
    }
    Ok(())
}

/// Get a position.
/// * daten: Service data for database access.
/// * uid: Affected position ID.
/// * returns: Position or possibly errors.
pub fn get_position<'a>(daten: &'a ServiceDaten, uid: &String) -> Result<Option<TbOrt>> {
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    let e = reps::tb_ort::get(&db, &daten.mandant_nr, &uid)?;
    Ok(e)
}

/// Get a list of positions.
/// * daten: Service data for database access.
/// * puid: Affected position ID.
/// * text: Affected text.
/// * returns: Position list or possibly errors.
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

/// Search for the date of fitting entry.
/// * daten: Service data for database access.
/// * dir: Affected direction.
/// * date: Affected base date.
/// * search: Affected search strings.
/// * returns: Position list or possibly errors.
pub fn search_date<'a>(
    daten: &'a ServiceDaten,
    dir: &SearchDirectionEnum,
    date: &Option<NaiveDate>,
    search: &[String; 9],
) -> Result<Option<NaiveDate>> {
    if let Some(_d) = date {
        let s = check_search(search);
        // println!("{:?}", s);
        let c = reps::establish_connection(daten);
        let db = DbContext::new(daten, &c);
        let l = reps::tb_eintrag::get_list_search(&db, dir, date, &s)?;
        if let Some(f) = l.get(0) {
            if *dir == SearchDirectionEnum::Last && s[0] == "%" && s[3] == "" && s[6] == "" {
                if let Some(d) = functions::nd_add_dmy(&f.datum, 1, 0, 0) {
                    return Ok(Some(d));
                }
            }
            return Ok(Some(f.datum));
        }
    }
    Ok(None)
}

fn check_search(search: &[String; 9]) -> [String; 9] {
    const COLUMNS: usize = 3;
    const ROWS: usize = 3;
    let mut s = search.clone();
    for i in 0..(COLUMNS * ROWS) {
        if !functions::is_like(s[i].as_str()) {
            s[i] = String::new();
        }
    }
    // Pack search pattern
    for y in 0..ROWS {
        let mut i = 0;
        for x in 0..(COLUMNS - 1) {
            if s[y * COLUMNS + x].is_empty() {
                if !s[y * COLUMNS + x + 1].is_empty() {
                    s[y * COLUMNS + i] = s[y * COLUMNS + x + 1].to_string();
                    s[y * COLUMNS + x + 1] = String::new();
                    i = i + 1;
                }
            } else {
                i = i + 1;
            }
        }
    }
    if s[0].is_empty() {
        s[0] = "%".to_string();
    }
    s
}
