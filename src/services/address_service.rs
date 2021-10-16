use super::reps::{self, DbContext};
use crate::{
    apis::services::ServiceDaten, base::functions, config::RsbpError, res::messages::M,
    services::undo::UndoRedoStack, Result,
};
use chrono::{Datelike, NaiveDate};
use diesel::Connection;

/// Get list with clients.
/// * daten: Service data for database access.
/// * date: Affected date.
/// * days: Number of days for interval.
/// * returns: List with clients.
pub fn get_birthday_list<'a>(
    daten: &'a ServiceDaten,
    date: &NaiveDate,
    days: i32,
) -> Result<Vec<String>> {
    let mut v: Vec<String> = vec![];
    let from = functions::ond_add_days(&Some(*date), -days.abs()).unwrap_or(*date);
    let to = functions::ond_add_days(&Some(*date), days.abs()).unwrap_or(*date);
    let j = date.year();
    let f = (from.month() as i32) * 100 + (from.day() as i32);
    v.push(M::ad001(&from, &to, daten.config.is_de()));
    let i = functions::iif_i32(
        j != from.year(),
        1,
        functions::iif_i32(j != to.year(), 2, 0),
    );
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<(), RsbpError, _>(|| {
        let _p = reps::ad_person::save(
            &mut db,
            &daten.mandant_nr,
            &"1".to_string(),
            &0,
            &"M".to_string(),
            &Some(NaiveDate::from_ymd(1970, 9, 1)),
            &901,
            &0,
            &0,
            &"Name1".to_string(),
            &Some("Name2".to_string()),
            &Some("Praedikat".to_string()),
            &Some("Vorname".to_string()),
            &Some("Titel".to_string()),
            &0,
        )?;
        Ok(())
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }

    // let l0 = reps::ad_person::get_list_test(&db)?;
    // for uid in l0 {
    //     println!("{}", uid);
    //     if !uid.starts_with("253") {
    //         let l1 = reps::ad_person::get(&db, &daten.mandant_nr, &uid)?.unwrap();
    //         println!("{:?}", l1.geburt);
    //     }
    // }
    let l = reps::ad_person::get_list_ext(&db, &from, &to)?;
    for p in l {
        let d = p.geburtk;
        if let Some(g) = p.geburt {
            let y = functions::ond_year(&p.geburt);
            let j1: i32;
            if i == 0 {
                j1 = j - y;
            } else if f <= d {
                j1 = from.year() - y;
            } else {
                j1 = to.year() - y;
            }
            v.push(M::ad002(&g, &p.name(), j1, daten.config.is_de()));
        }
    }
    Ok(v)
}
