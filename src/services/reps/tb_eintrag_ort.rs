use super::DbContext;
use crate::{config::RsbpError, services::undo::UndoEntry, Result};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use rsbp_rep::{
    models::{TbEintragOrt, TbOrt},
    models_ext::TbEintragOrtExt,
    schema::*,
};

/// Undo a dataset.
pub fn undo(db: &mut DbContext, or: &String, ac: &String) -> Result<()> {
    let oo = UndoEntry::from_str::<TbEintragOrt>(or)?;
    let oa = UndoEntry::from_str::<TbEintragOrt>(ac)?;
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
    let oo = UndoEntry::from_str::<TbEintragOrt>(or)?;
    let oa = UndoEntry::from_str::<TbEintragOrt>(ac)?;
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
    ort_uid_: &String,
    datum_von_: &NaiveDate,
    datum_bis_: &NaiveDate,
    angelegt_von_: &Option<String>,
    angelegt_am_: &Option<NaiveDateTime>,
    geaendert_von_: &Option<String>,
    geaendert_am_: &Option<NaiveDateTime>,
) -> Result<TbEintragOrt> {
    let op = TB_EINTRAG_ORT::table
        .filter(
            TB_EINTRAG_ORT::mandant_nr
                .eq(mandant_nr_)
                .and(TB_EINTRAG_ORT::ort_uid.eq(ort_uid_.clone()))
                .and(TB_EINTRAG_ORT::datum_von.eq(datum_von_.clone()))
                .and(TB_EINTRAG_ORT::datum_bis.eq(datum_bis_.clone())),
        )
        .first::<TbEintragOrt>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    let mut p = TbEintragOrt {
        mandant_nr: *mandant_nr_,
        ort_uid: ort_uid_.clone(),
        datum_von: datum_von_.clone(),
        datum_bis: datum_bis_.clone(),
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
    ort_uid_: &String,
    datum_von_: &NaiveDate,
    datum_bis_: &NaiveDate,
) -> Result<TbEintragOrt> {
    save0(
        db,
        mandant_nr_,
        ort_uid_,
        datum_von_,
        datum_bis_,
        &None,
        &None,
        &None,
        &None,
    )
}

/// Get dataset by primary key.
#[allow(dead_code)]
pub fn get(
    db: &DbContext,
    mandant_nr_: &i32,
    ort_uid_: &String,
    datum_von_: &NaiveDate,
    datum_bis_: &NaiveDate,
) -> Result<Option<TbEintragOrt>> {
    let p = TB_EINTRAG_ORT::table
        .filter(
            TB_EINTRAG_ORT::mandant_nr
                .eq(mandant_nr_)
                .and(TB_EINTRAG_ORT::ort_uid.eq(ort_uid_.clone()))
                .and(TB_EINTRAG_ORT::datum_von.eq(datum_von_.clone()))
                .and(TB_EINTRAG_ORT::datum_bis.eq(datum_bis_.clone())),
        )
        .first::<TbEintragOrt>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(p)
}

/// Get dataset by primary key.
pub fn get2(db: &DbContext, b: &TbEintragOrt) -> Result<Option<TbEintragOrt>> {
    let p = TB_EINTRAG_ORT::table
        .filter(
            TB_EINTRAG_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG_ORT::ort_uid.eq(b.ort_uid.clone()))
                .and(TB_EINTRAG_ORT::datum_von.eq(b.datum_von.clone()))
                .and(TB_EINTRAG_ORT::datum_bis.eq(b.datum_bis.clone())),
        )
        .first::<TbEintragOrt>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(p)
}

/// Get list.
#[allow(dead_code)]
pub fn get_list(db: &DbContext, mandant_nr_: i32) -> Result<Vec<TbEintragOrt>> {
    let list = TB_EINTRAG_ORT::table
        .filter(TB_EINTRAG_ORT::mandant_nr.eq(mandant_nr_))
        .load::<TbEintragOrt>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(list)
}

/// Insert a dataset.
pub fn insert<'a>(db: &mut DbContext, b: &'a TbEintragOrt) -> Result<&'a TbEintragOrt> {
    let rows = diesel::insert_into(TB_EINTRAG_ORT::table)
        .values(b)
        .execute(db.c)?;
    if rows <= 0 {
        return Err(RsbpError::NotFound);
    }
    db.ul.add(&UndoEntry::tb_eintrag_ort(None, Some(b)));
    Ok(b)
}

/// Update a dataset.
pub fn update<'a>(db: &mut DbContext, b: &'a TbEintragOrt) -> Result<&'a TbEintragOrt> {
    let oo = get2(&db, b)?;
    let rows = diesel::update(
        TB_EINTRAG_ORT::table.filter(
            TB_EINTRAG_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG_ORT::ort_uid.eq(b.ort_uid.clone()))
                .and(TB_EINTRAG_ORT::datum_von.eq(b.datum_von.clone()))
                .and(TB_EINTRAG_ORT::datum_bis.eq(b.datum_bis.clone())),
        ),
    )
    .set((
        TB_EINTRAG_ORT::angelegt_von.eq(b.angelegt_von.as_ref()),
        TB_EINTRAG_ORT::angelegt_am.eq(b.angelegt_am),
        TB_EINTRAG_ORT::geaendert_von.eq(b.geaendert_von.as_ref()),
        TB_EINTRAG_ORT::geaendert_am.eq(b.geaendert_am),
    ))
    .execute(db.c)?;
    if rows <= 0 || oo.is_none() {
        return Err(RsbpError::NotFound);
    }
    if let Some(o) = oo {
        db.ul.add(&UndoEntry::tb_eintrag_ort(Some(&o), Some(b)));
    }
    Ok(b)
}

/// Delete a dataset.
pub fn delete(db: &mut DbContext, b: &TbEintragOrt) -> Result<()> {
    let oo = get2(db, b)?;
    let rows = diesel::delete(
        TB_EINTRAG_ORT::table.filter(
            TB_EINTRAG_ORT::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG_ORT::ort_uid.eq(b.ort_uid.clone()))
                .and(TB_EINTRAG_ORT::datum_von.eq(b.datum_von.clone()))
                .and(TB_EINTRAG_ORT::datum_bis.eq(b.datum_bis.clone())),
        ),
    )
    .execute(db.c)?;
    if rows <= 0 || oo.is_none() {
        return Err(RsbpError::NotFound);
    }
    if let Some(o) = oo {
        db.ul.add(&UndoEntry::tb_eintrag_ort(Some(&o), None));
    }
    Ok(())
}

/// Get list.
pub fn get_list_ext(
    db: &DbContext,
    mandant_nr_: &i32,
    date: &NaiveDate,
) -> Result<Vec<TbEintragOrtExt>> {
    let join = TB_EINTRAG_ORT::table
        .filter(
            TB_EINTRAG_ORT::mandant_nr.eq(mandant_nr_).and(
                TB_EINTRAG_ORT::datum_von
                    .le(date)
                    .and(TB_EINTRAG_ORT::datum_bis.ge(date)),
            ),
        )
        .inner_join(
            TB_ORT::table.on(TB_ORT::mandant_nr
                .eq(TB_EINTRAG_ORT::mandant_nr)
                .and(TB_ORT::uid.eq(TB_EINTRAG_ORT::ort_uid))),
        )
        .load::<(TbEintragOrt, TbOrt)>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    let mut l: Vec<TbEintragOrtExt> = Vec::new();
    for j in join {
        l.push(TbEintragOrtExt {
            mandant_nr: j.0.mandant_nr,
            ort_uid: j.0.ort_uid,
            datum_von: j.0.datum_von,
            datum_bis: j.0.datum_bis,
            angelegt_von: j.0.angelegt_von,
            angelegt_am: j.0.angelegt_am,
            geaendert_von: j.0.geaendert_von,
            geaendert_am: j.0.geaendert_am,
            bezeichnung: j.1.bezeichnung,
            breite: j.1.breite,
            laenge: j.1.laenge,
            hoehe: j.1.hoehe,
            notiz: j.1.notiz,
        });
    }
    Ok(l)
}
