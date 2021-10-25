use super::DbContext;
use crate::{config::RsbpError, services::undo::UndoEntry, Result};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use rsbp_rep::{models::AdPerson, schema::*};

/// Undo a dataset.
pub fn undo(db: &mut DbContext, or: &String, ac: &String) -> Result<()> {
    let oo = UndoEntry::from_str::<AdPerson>(or)?;
    let oa = UndoEntry::from_str::<AdPerson>(ac)?;
    if let (Some(o), Some(_a)) = (&oo, &oa) {
        // Update
        update(db, o)?;
    } else if let Some(a) = &oa {
        // Insert
        delete(db, a)?;
    } else if let Some(o) = &oo {
        // Delete
        insert(db, o)?;
    }
    Ok(())
}

/// Redo a dataset.
pub fn redo(db: &mut DbContext, or: &String, ac: &String) -> Result<()> {
    let oo = UndoEntry::from_str::<AdPerson>(or)?;
    let oa = UndoEntry::from_str::<AdPerson>(ac)?;
    if let (Some(_o), Some(a)) = (&oo, &oa) {
        // Update
        update(db, a)?;
    } else if let Some(a) = &oa {
        // Insert
        insert(db, a)?;
    } else if let Some(o) = &oo {
        // Delete
        delete(db, o)?;
    }
    Ok(())
}

/// Save dataset with all values.
#[allow(dead_code)]
pub fn save0(
    db: &mut DbContext,
    mandant_nr_: &i32,
    uid_: &String,
    typ_: &i32,
    geschlecht_: &String,
    geburt_: &Option<NaiveDate>,
    geburtk_: &i32,
    anrede_: &i32,
    fanrede_: &i32,
    name1_: &String,
    name2_: &Option<String>,
    praedikat_: &Option<String>,
    vorname_: &Option<String>,
    titel_: &Option<String>,
    person_status_: &i32,
    angelegt_von_: &Option<String>,
    angelegt_am_: &Option<NaiveDateTime>,
    geaendert_von_: &Option<String>,
    geaendert_am_: &Option<NaiveDateTime>,
) -> Result<AdPerson> {
    let op = AD_PERSON::table
        .filter(
            AD_PERSON::mandant_nr
                .eq(mandant_nr_)
                .and(AD_PERSON::uid.eq(uid_.clone())),
        )
        .first::<AdPerson>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    let mut p = AdPerson {
        mandant_nr: *mandant_nr_,
        uid: uid_.clone(),
        typ: *typ_,
        geschlecht: geschlecht_.clone(),
        geburt: geburt_.clone(),
        geburtk: *geburtk_,
        anrede: *anrede_,
        fanrede: *fanrede_,
        name1: name1_.clone(),
        name2: name2_.clone(),
        praedikat: praedikat_.clone(),
        vorname: vorname_.clone(),
        titel: titel_.clone(),
        person_status: *person_status_,
        angelegt_von: None,
        angelegt_am: None,
        geaendert_von: None,
        geaendert_am: None,
    };
    if let Some(pu) = op {
        if p != pu {
            p.angelegt_von = pu.angelegt_von;
            p.angelegt_am = pu.angelegt_am;
            p.geaendert_von = pu.geaendert_von;
            p.geaendert_am = pu.geaendert_am;
            if p.angelegt_von.is_none() || !angelegt_von_.is_none() {
                super::mach_angelegt(&mut p, db.daten, angelegt_von_, angelegt_am_);
            }
            super::mach_geaendert(&mut p, db.daten, geaendert_von_, geaendert_am_);
            update(db, &p)?;
        }
    } else {
        super::mach_angelegt(&mut p, db.daten, angelegt_von_, angelegt_am_);
        if !geaendert_von_.is_none() {
            super::mach_geaendert(&mut p, db.daten, geaendert_von_, geaendert_am_);
        }
        insert(db, &p)?;
    }
    return Ok(p);
}

/// Save dataset without revision columns.
#[allow(dead_code)]
pub fn save(
    db: &mut DbContext,
    mandant_nr_: &i32,
    uid_: &String,
    typ_: &i32,
    geschlecht_: &String,
    geburt_: &Option<NaiveDate>,
    geburtk_: &i32,
    anrede_: &i32,
    fanrede_: &i32,
    name1_: &String,
    name2_: &Option<String>,
    praedikat_: &Option<String>,
    vorname_: &Option<String>,
    titel_: &Option<String>,
    person_status_: &i32,
) -> Result<AdPerson> {
    save0(
        db,
        mandant_nr_,
        uid_,
        typ_,
        geschlecht_,
        geburt_,
        geburtk_,
        anrede_,
        fanrede_,
        name1_,
        name2_,
        praedikat_,
        vorname_,
        titel_,
        person_status_,
        &None,
        &None,
        &None,
        &None,
    )
}

/// Get dataset by primary key.
#[allow(dead_code)]
pub fn get(db: &DbContext, mandant_nr_: &i32, uid_: &String) -> Result<Option<AdPerson>> {
    let p = AD_PERSON::table
        .filter(
            AD_PERSON::mandant_nr
                .eq(mandant_nr_)
                .and(AD_PERSON::uid.eq(uid_.clone())),
        )
        .first::<AdPerson>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(p)
}

/// Get dataset by primary key.
pub fn get2(db: &DbContext, b: &AdPerson) -> Result<Option<AdPerson>> {
    let p = AD_PERSON::table
        .filter(
            AD_PERSON::mandant_nr
                .eq(b.mandant_nr)
                .and(AD_PERSON::uid.eq(b.uid.clone())),
        )
        .first::<AdPerson>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(p)
}

/// Get list.
#[allow(dead_code)]
pub fn get_list(db: &DbContext, mandant_nr_: i32) -> Result<Vec<AdPerson>> {
    let list = AD_PERSON::table
        .filter(AD_PERSON::mandant_nr.eq(mandant_nr_))
        .load::<AdPerson>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(list)
}

/// Insert a dataset.
pub fn insert<'a>(db: &mut DbContext, b: &'a AdPerson) -> Result<&'a AdPerson> {
    let rows = diesel::insert_into(AD_PERSON::table)
        .values(b)
        .execute(db.c)?;
    if rows <= 0 {
        return Err(RsbpError::NotFound);
    }
    db.ul.add(&UndoEntry::ad_person(None, Some(b)));
    Ok(b)
}

/// Update a dataset.
pub fn update<'a>(db: &mut DbContext, b: &'a AdPerson) -> Result<&'a AdPerson> {
    let oo = get2(&db, b)?;
    let rows = diesel::update(
        AD_PERSON::table.filter(
            AD_PERSON::mandant_nr
                .eq(b.mandant_nr)
                .and(AD_PERSON::uid.eq(b.uid.clone())),
        ),
    )
    .set((
        AD_PERSON::typ.eq(b.typ),
        AD_PERSON::geschlecht.eq(b.geschlecht.as_str()),
        AD_PERSON::geburt.eq(b.geburt),
        AD_PERSON::geburtk.eq(b.geburtk),
        AD_PERSON::anrede.eq(b.anrede),
        AD_PERSON::fanrede.eq(b.fanrede),
        AD_PERSON::name1.eq(b.name1.as_str()),
        AD_PERSON::name2.eq(b.name2.as_ref()),
        AD_PERSON::praedikat.eq(b.praedikat.as_ref()),
        AD_PERSON::vorname.eq(b.vorname.as_ref()),
        AD_PERSON::titel.eq(b.titel.as_ref()),
        AD_PERSON::person_status.eq(b.person_status),
        AD_PERSON::angelegt_von.eq(b.angelegt_von.as_ref()),
        AD_PERSON::angelegt_am.eq(b.angelegt_am),
        AD_PERSON::geaendert_von.eq(b.geaendert_von.as_ref()),
        AD_PERSON::geaendert_am.eq(b.geaendert_am),
    ))
    .execute(db.c)?;
    if rows <= 0 || oo.is_none() {
        return Err(RsbpError::NotFound);
    }
    if let Some(o) = oo {
        db.ul.add(&UndoEntry::ad_person(Some(&o), Some(b)));
    }
    Ok(b)
}

/// Delete a dataset.
pub fn delete(db: &mut DbContext, b: &AdPerson) -> Result<()> {
    let oo = get2(db, b)?;
    let rows = diesel::delete(
        AD_PERSON::table.filter(
            AD_PERSON::mandant_nr
                .eq(b.mandant_nr)
                .and(AD_PERSON::uid.eq(b.uid.clone())),
        ),
    )
    .execute(db.c)?;
    if rows <= 0 || oo.is_none() {
        return Err(RsbpError::NotFound);
    }
    if let Some(o) = oo {
        db.ul.add(&UndoEntry::ad_person(Some(&o), None));
    }
    Ok(())
}

/// Get birthday list.
pub fn get_list_ext(db: &DbContext, from: &NaiveDate, to: &NaiveDate) -> Result<Vec<AdPerson>> {
    let f = (from.month() * 100 + from.day()) as i32;
    let t = (to.month() * 100 + to.day()) as i32;
    let mut q = AD_PERSON::table.into_boxed().filter(
        AD_PERSON::mandant_nr
            .eq(db.daten.mandant_nr)
            .and(AD_PERSON::person_status.eq(0))
            .and(AD_PERSON::geburtk.ne(0)),
    );
    if from.year() == to.year() {
        q = q.filter(AD_PERSON::geburtk.ge(f).and(AD_PERSON::geburtk.le(t)));
    } else {
        q = q.filter(AD_PERSON::geburtk.lt(t).or(AD_PERSON::geburtk.ge(f)));
    }
    let list = q
        .order((
            AD_PERSON::geburtk,
            AD_PERSON::name1,
            AD_PERSON::vorname,
            AD_PERSON::uid,
        ))
        .limit(-1)
        .load::<AdPerson>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(list)
}

/// Get test list.
#[allow(dead_code)]
pub fn get_list_test(db: &DbContext) -> Result<Vec<String>> {
    let list = AD_PERSON::table
        .filter(AD_PERSON::mandant_nr.eq(db.daten.mandant_nr))
        .select(AD_PERSON::uid)
        .load::<String>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(list)
}
