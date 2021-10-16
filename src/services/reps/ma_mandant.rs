use super::DbContext;
use crate::{config::RsbpError, services::undo::UndoEntry, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rsbp_rep::{models::MaMandant, schema::*};

/// Undo a dataset.
pub fn undo(db: &mut DbContext, or: &String, ac: &String) -> Result<()> {
    let oo = UndoEntry::from_str::<MaMandant>(or)?;
    let oa = UndoEntry::from_str::<MaMandant>(ac)?;
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
    let oo = UndoEntry::from_str::<MaMandant>(or)?;
    let oa = UndoEntry::from_str::<MaMandant>(ac)?;
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
    nr_: &i32,
    beschreibung_: &String,
    angelegt_von_: &Option<String>,
    angelegt_am_: &Option<NaiveDateTime>,
    geaendert_von_: &Option<String>,
    geaendert_am_: &Option<NaiveDateTime>,
) -> Result<MaMandant> {
    let op = MA_MANDANT::table
        .filter(MA_MANDANT::nr.eq(nr_))
        .first::<MaMandant>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    let mut p = MaMandant {
        nr: *nr_,
        beschreibung: beschreibung_.clone(),
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
pub fn save(db: &mut DbContext, nr_: &i32, beschreibung_: &String) -> Result<MaMandant> {
    save0(db, nr_, beschreibung_, &None, &None, &None, &None)
}

/// Get dataset by primary key.
#[allow(dead_code)]
pub fn get(db: &DbContext, nr_: &i32) -> Result<Option<MaMandant>> {
    let p = MA_MANDANT::table
        .filter(MA_MANDANT::nr.eq(nr_))
        .first::<MaMandant>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(p)
}

/// Get dataset by primary key.
pub fn get2(db: &DbContext, b: &MaMandant) -> Result<Option<MaMandant>> {
    let p = MA_MANDANT::table
        .filter(MA_MANDANT::nr.eq(b.nr))
        .first::<MaMandant>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(p)
}

/// Get list.
#[allow(dead_code)]
pub fn get_list(db: &DbContext) -> Result<Vec<MaMandant>> {
    let list = MA_MANDANT::table
        .load::<MaMandant>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(list)
}

/// Insert a dataset.
pub fn insert<'a>(db: &mut DbContext, b: &'a MaMandant) -> Result<&'a MaMandant> {
    let rows = diesel::insert_into(MA_MANDANT::table)
        .values(b)
        .execute(db.c)?;
    if rows <= 0 {
        return Err(RsbpError::NotFound);
    }
    db.ul.add(&UndoEntry::ma_mandant(None, Some(b)));
    Ok(b)
}

/// Update a dataset.
pub fn update<'a>(db: &mut DbContext, b: &'a MaMandant) -> Result<&'a MaMandant> {
    let oo = get2(&db, b)?;
    let rows = diesel::update(MA_MANDANT::table.filter(MA_MANDANT::nr.eq(b.nr)))
        .set((
            MA_MANDANT::beschreibung.eq(b.beschreibung.as_str()),
            MA_MANDANT::angelegt_von.eq(b.angelegt_von.as_ref()),
            MA_MANDANT::angelegt_am.eq(b.angelegt_am),
            MA_MANDANT::geaendert_von.eq(b.geaendert_von.as_ref()),
            MA_MANDANT::geaendert_am.eq(b.geaendert_am),
        ))
        .execute(db.c)?;
    if rows <= 0 || oo.is_none() {
        return Err(RsbpError::NotFound);
    }
    if let Some(o) = oo {
        db.ul.add(&UndoEntry::ma_mandant(Some(&o), Some(b)));
    }
    Ok(b)
}

/// Delete a dataset.
pub fn delete(db: &mut DbContext, b: &MaMandant) -> Result<()> {
    let oo = get2(db, b)?;
    let rows = diesel::delete(MA_MANDANT::table.filter(MA_MANDANT::nr.eq(b.nr))).execute(db.c)?;
    if rows <= 0 || oo.is_none() {
        return Err(RsbpError::NotFound);
    }
    if let Some(o) = oo {
        db.ul.add(&UndoEntry::ma_mandant(Some(&o), None));
    }
    Ok(())
}
