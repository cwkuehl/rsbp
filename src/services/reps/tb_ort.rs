use super::DbContext;
use crate::{config::RsbpError, services::undo::UndoEntry, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rsbp_rep::{models::TbOrt, schema::*};

/// Undo a dataset.
pub fn undo(db: &mut DbContext, or: &String, ac: &String) -> Result<()> {
    let oo = UndoEntry::from_str::<TbOrt>(or)?;
    let oa = UndoEntry::from_str::<TbOrt>(ac)?;
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
    let oo = UndoEntry::from_str::<TbOrt>(or)?;
    let oa = UndoEntry::from_str::<TbOrt>(ac)?;
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
    bezeichnung_: &String,
    breite_: &f64,
    laenge_: &f64,
    hoehe_: &f64,
    notiz_: &String,
    angelegt_von_: &Option<String>,
    angelegt_am_: &Option<NaiveDateTime>,
    geaendert_von_: &Option<String>,
    geaendert_am_: &Option<NaiveDateTime>,
) -> Result<TbOrt> {
    let op = TB_ORT::table
        .filter(
            TB_ORT::mandant_nr
                .eq(mandant_nr_)
                .and(TB_ORT::uid.eq(uid_.clone())),
        )
        .first::<TbOrt>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    let mut p = TbOrt {
        mandant_nr: *mandant_nr_,
        uid: uid_.clone(),
        bezeichnung: bezeichnung_.clone(),
        breite: *breite_,
        laenge: *laenge_,
        hoehe: *hoehe_,
        notiz: notiz_.clone(),
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
    bezeichnung_: &String,
    breite_: &f64,
    laenge_: &f64,
    hoehe_: &f64,
    notiz_: &String,
) -> Result<TbOrt> {
    save0(
        db,
        mandant_nr_,
        uid_,
        bezeichnung_,
        breite_,
        laenge_,
        hoehe_,
        notiz_,
        &None,
        &None,
        &None,
        &None,
    )
}

/// Get dataset by primary key.
#[allow(dead_code)]
pub fn get(db: &DbContext, mandant_nr_: &i32, uid_: &String) -> Result<Option<TbOrt>> {
    let p = TB_ORT::table
        .filter(
            TB_ORT::mandant_nr
                .eq(mandant_nr_)
                .and(TB_ORT::uid.eq(uid_.clone())),
        )
        .first::<TbOrt>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(p)
}

/// Get dataset by primary key.
pub fn get2(db: &DbContext, b: &TbOrt) -> Result<Option<TbOrt>> {
    let p = TB_ORT::table
        .filter(
            TB_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_ORT::uid.eq(b.uid.clone())),
        )
        .first::<TbOrt>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(p)
}

/// Get list.
#[allow(dead_code)]
pub fn get_list(db: &DbContext, mandant_nr_: i32) -> Result<Vec<TbOrt>> {
    let list = TB_ORT::table
        .filter(TB_ORT::mandant_nr.eq(mandant_nr_))
        .load::<TbOrt>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(list)
}

/// Insert a dataset.
pub fn insert<'a>(db: &mut DbContext, b: &'a TbOrt) -> Result<&'a TbOrt> {
    let rows = diesel::insert_into(TB_ORT::table).values(b).execute(db.c)?;
    if rows <= 0 {
        return Err(RsbpError::NotFound);
    }
    db.ul.add(&UndoEntry::tb_ort(None, Some(b)));
    Ok(b)
}

/// Update a dataset.
pub fn update<'a>(db: &mut DbContext, b: &'a TbOrt) -> Result<&'a TbOrt> {
    let oo = get2(&db, b)?;
    let rows = diesel::update(
        TB_ORT::table.filter(
            TB_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_ORT::uid.eq(b.uid.clone())),
        ),
    )
    .set((
        TB_ORT::bezeichnung.eq(b.bezeichnung.as_str()),
        TB_ORT::breite.eq(b.breite),
        TB_ORT::laenge.eq(b.laenge),
        TB_ORT::hoehe.eq(b.hoehe),
        TB_ORT::notiz.eq(b.notiz.as_str()),
        TB_ORT::angelegt_von.eq(b.angelegt_von.as_ref()),
        TB_ORT::angelegt_am.eq(b.angelegt_am),
        TB_ORT::geaendert_von.eq(b.geaendert_von.as_ref()),
        TB_ORT::geaendert_am.eq(b.geaendert_am),
    ))
    .execute(db.c)?;
    if rows <= 0 || oo.is_none() {
        return Err(RsbpError::NotFound);
    }
    if let Some(o) = oo {
        db.ul.add(&UndoEntry::tb_ort(Some(&o), Some(b)));
    }
    Ok(b)
}

/// Delete a dataset.
pub fn delete(db: &mut DbContext, b: &TbOrt) -> Result<()> {
    let oo = get2(db, b)?;
    let rows = diesel::delete(
        TB_ORT::table.filter(
            TB_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_ORT::uid.eq(b.uid.clone())),
        ),
    )
    .execute(db.c)?;
    if rows <= 0 || oo.is_none() {
        return Err(RsbpError::NotFound);
    }
    if let Some(o) = oo {
        db.ul.add(&UndoEntry::tb_ort(Some(&o), None));
    }
    Ok(())
}

/// Get list.
pub fn get_list_ext(
    db: &DbContext,
    mandant_nr_: &i32,
    puid: &Option<String>,
    text: &Option<String>,
) -> Result<Vec<TbOrt>> {
    let mut q = TB_ORT::table
        .into_boxed()
        .filter(TB_ORT::mandant_nr.eq(mandant_nr_));
    if let Some(id) = puid {
        q = q.filter(TB_ORT::uid.eq(id));
    }
    if let Some(t) = text {
        q = q.filter(TB_ORT::bezeichnung.like(t).or(TB_ORT::notiz.like(t)));
    }
    let list = q
        .load::<TbOrt>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(list)
}
