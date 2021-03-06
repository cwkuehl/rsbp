use super::{
    reps::{self, DbContext},
    undo::UndoRedoStack,
};
use crate::{
    apis::{enums::SearchDirectionEnum, services::ServiceDaten},
    base::functions,
    config::RsbpError,
    res::messages::M,
    Result,
};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use diesel::Connection;
use regex::{Regex, RegexBuilder};
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
        let mut liste = reps::tb_eintrag_ort::get_list_ext(&db, Some(date), &0, None, None)?;
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
                reps::tb_eintrag_ort::get_list_ext(&db, Some(&from), &0, Some(&to), Some(&puid))?;
            let ovop = listep.first();
            if let Some(p) = liste.iter().position(|a| a.ort_uid == puid) {
                liste.remove(p); // nicht mehr l??schen
            }
            if listep.is_empty() || ovop.is_none() {
                // Zeitraum leer
                optimize_positions(&mut db, &puid, &from, &to, &None, &None)?;
            } else if let Some(vop) = ovop {
                if listep.len() == 1 {
                    if vop.datum_von == from && vop.datum_bis == to {
                        functions::mach_nichts();
                    } else if vop.datum_von <= from && vop.datum_bis >= to {
                        if from == to {
                            functions::mach_nichts(); // Fall: Aus Versehen gel??scht und wieder hinzugef??gt.
                        } else {
                            // Zeitraum wird verk??rzt.
                            reps::tb_eintrag_ort::save0(
                                &mut db,
                                &daten.mandant_nr,
                                &puid,
                                &from,
                                &to,
                                &vop.angelegt_von,
                                &vop.angelegt_am,
                                &None,
                                &None,
                            )?;
                            reps::tb_eintrag_ort::delete(&mut db, vop)?;
                        }
                    } else {
                        // Nicht verk??rzen.
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
        // ??berfl??ssige Orte l??schen.
        for vo in liste {
            if vo.datum_von == vo.datum_bis {
                reps::tb_eintrag_ort::delete(&mut db, &vo)?; // Eintrag l??schen
            } else if vo.datum_von == *date {
                // Einen Tag vorne verk??rzen
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
                // Einen Tag hinten verk??rzen
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

/// Optimieren der Positionen, d.h. verl??ngern oder L??cke f??llen.
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
    let listeb = reps::tb_eintrag_ort::get_list_ext(&db, Some(&from), &-1, None, Some(&puid))?;
    let listea = reps::tb_eintrag_ort::get_list_ext(&db, Some(&to), &1, None, Some(&puid))?;
    let obef = listeb.first();
    let oaft = listea.first();

    if let Some(bef) = obef {
        if let Some(aft) = oaft {
            // L??cke f??llen
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
            // Zeitraum hinten anh??ngen
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
        // Zeitraum vorne anh??ngen
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
/// * puid: Affected position uid.
/// * from: Affected from date.
/// * to: Affected to date.
/// * returns: Position list or possibly errors.
pub fn search_date<'a>(
    daten: &'a ServiceDaten,
    dir: &SearchDirectionEnum,
    date: &Option<NaiveDate>,
    search: &[String; 9],
    puid: &Option<String>,
    from: &Option<NaiveDate>,
    to: &Option<NaiveDate>,
) -> Result<Option<NaiveDate>> {
    if let Some(_d) = date {
        let s = check_search(search);
        // println!("{:?}", s);
        let c = reps::establish_connection(daten);
        let db = DbContext::new(daten, &c);
        let l = reps::tb_eintrag::get_list_search(&db, dir, date, &s, puid, from, to)?;
        if let Some(f) = l.get(0) {
            if *dir == SearchDirectionEnum::Last
                && s[0] == "%"
                && s[3] == ""
                && s[6] == ""
                && puid.is_none()
            {
                if let Some(d) = functions::nd_add_dmy(&f.datum, 1, 0, 0) {
                    return Ok(Some(d));
                }
            }
            return Ok(Some(f.datum));
        }
    }
    Ok(None)
}

/// Get a vector of all fitting diary entries for storing in a file.
/// * daten: Service data for database access.
/// * search: Affected search strings.
/// * puid: Affected position uid.
/// * from: Affected from date.
/// * to: Affected to date.
/// * returns: Vector of all fitting diary entries.
pub fn get_file<'a>(
    daten: &'a ServiceDaten,
    search: &[String; 9],
    puid: &Option<String>,
    from: &Option<NaiveDate>,
    to: &Option<NaiveDate>,
) -> Result<Vec<String>> {
    let mut s = check_search(search);
    // println!("{:?}", s);
    let mut rf = false; // Reihenfolge-Test
    let mut str = s[0].to_string();
    let mut muster = String::new();

    if str.contains("####") {
        muster = regex::escape(&str).replace("\\#\\#\\#\\#", "\\D*(\\d+)");
        if muster.starts_with("%") {
            muster = muster[1..].to_string();
        }
        if muster.ends_with("%") {
            let l = muster.len();
            muster = muster[..l - 1].to_string();
        }
        str = str.replace("####", "");
        rf = true;
    }
    s[0] = str;
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    let l = reps::tb_eintrag::get_list_search(
        &db,
        &SearchDirectionEnum::None,
        &None,
        &s,
        puid,
        from,
        to,
    )?;
    let is_de = daten.config.is_de();
    let mut v: Vec<String> = vec![];
    v.push(M::tb002(&daten.get_now(), is_de));
    v.push(M::tb003(&s, is_de));
    if let Some(uid) = puid {
        if let Some(p) = reps::tb_ort::get(&db, &daten.mandant_nr, uid)? {
            v.push(M::tb010(&p.bezeichnung, is_de));
        }
    }
    if from.is_some() || to.is_some() {
        v.push(M::tb011(&from, &to, is_de));
    }
    v.push("".into());
    if rf {
        // Z??hler pr??fen.
        let mut z: i64 = -1;
        let p: Regex = RegexBuilder::new(&muster)
            .case_insensitive(true)
            .build()
            .map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
        for e in l {
            let str = e.eintrag;
            v.push(M::tb006(&e.datum, &str, is_de));
            if !str.is_empty() {
                for m in p.captures_iter(&str) {
                    let l = functions::to_i64(&m[1]);
                    if z < 0 {
                        z = l;
                    } else if z != l {
                        // Falscher Z??hler am %1$s: %2$s, erwartet: %3$s
                        return Err(RsbpError::error_string(
                            M::tb004(&e.datum, &m[1], &z.to_string().as_str(), is_de).as_str(),
                        ));
                    }
                    z = z + 1;
                }
            }
        }
    } else {
        for e in l {
            v.push(M::tb006(&e.datum, &e.eintrag, is_de));
        }
    }
    Ok(v)
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

/// Save a position.
/// * daten: Service data for database access.
/// * uid: Affected ID.
/// * desc: Affected description.
/// * lat: Affected latitude.
/// * lon: Affected longitude.
/// * alt: Affected altitude.
/// * memo: Affected memos.
/// * returns: Possibly errors.
pub fn save_position<'a>(
    daten: &'a ServiceDaten,
    uid: &String,
    desc: &String,
    lat: &String,
    lon: &String,
    alt: &String,
    memo: &String,
) -> Result<()> {
    let mut r: Vec<String> = vec![];
    let d = desc.trim().to_string();
    let m = memo.trim().to_string();
    let is_de = daten.config.is_de();
    if d.len() <= 0 {
        r.push(M::mec(M::TB007, is_de).into_owned());
    }
    let la = functions::to_f64(lat.as_str(), is_de);
    if la < -90.0 || la > 90.0 {
        r.push(M::mec(M::TB008, is_de).into_owned());
    }
    let lo = functions::to_f64(lon.as_str(), is_de);
    if lo < -180.0 || lo > 180.0 {
        r.push(M::mec(M::TB009, is_de).into_owned());
    }
    let al = functions::to_f64(alt.as_str(), is_de);
    if r.len() > 0 {
        return Err(RsbpError::error(&r));
    }
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<(), RsbpError, _>(|| {
        reps::tb_ort::save0(
            &mut db,
            &daten.mandant_nr,
            &uid,
            &d,
            &la,
            &lo,
            &al,
            &m,
            &None,
            &None,
            &None,
            &None,
        )?;
        Ok(())
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }
    tr
}

/// Delete a position.
/// * daten: Service data for database access.
/// * e: Affected Entity.
/// * returns: Possibly errors.
pub fn delete_position<'a>(daten: &'a ServiceDaten, e: &TbOrt) -> Result<()> {
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<(), RsbpError, _>(|| {
        let plist = reps::tb_eintrag_ort::get_list_ext(&db, None, &0, None, Some(&e.uid))?;
        if let Some(p) = plist.first() {
            return Err(RsbpError::error_string(
                M::tb013(&p.datum_von, daten.config.is_de()).as_str(),
            ));
        }
        reps::tb_ort::delete(&mut db, e)?;
        Ok(())
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }
    tr
}
