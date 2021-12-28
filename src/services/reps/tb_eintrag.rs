use super::DbContext;
use crate::{
    apis::enums::SearchDirectionEnum, base::functions, config::RsbpError,
    services::undo::UndoEntry, Result,
};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use rsbp_rep::{models::TbEintrag, schema::*};

/// Undo a dataset.
pub fn undo(db: &mut DbContext, or: &String, ac: &String) -> Result<()> {
    let oo = UndoEntry::from_str::<TbEintrag>(or)?;
    let oa = UndoEntry::from_str::<TbEintrag>(ac)?;
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
    let oo = UndoEntry::from_str::<TbEintrag>(or)?;
    let oa = UndoEntry::from_str::<TbEintrag>(ac)?;
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
    datum_: &NaiveDate,
    eintrag_: &String,
    angelegt_von_: &Option<String>,
    angelegt_am_: &Option<NaiveDateTime>,
    geaendert_von_: &Option<String>,
    geaendert_am_: &Option<NaiveDateTime>,
    replikation_uid_: &Option<String>,
) -> Result<TbEintrag> {
    let op = TB_EINTRAG::table
        .filter(
            TB_EINTRAG::mandant_nr
                .eq(mandant_nr_)
                .and(TB_EINTRAG::datum.eq(datum_.clone())),
        )
        .first::<TbEintrag>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    let mut p = TbEintrag {
        mandant_nr: *mandant_nr_,
        datum: datum_.clone(),
        eintrag: eintrag_.clone(),
        angelegt_von: None,
        angelegt_am: None,
        geaendert_von: None,
        geaendert_am: None,
        replikation_uid: replikation_uid_.clone(),
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
    datum_: &NaiveDate,
    eintrag_: &String,
) -> Result<TbEintrag> {
    save0(
        db,
        mandant_nr_,
        datum_,
        eintrag_,
        &None,
        &None,
        &None,
        &None,
        &None,
    )
}

/// Get dataset by primary key.
pub fn get(db: &DbContext, mandant_nr_: &i32, datum_: &NaiveDate) -> Result<Option<TbEintrag>> {
    let p = TB_EINTRAG::table
        .filter(
            TB_EINTRAG::mandant_nr
                .eq(mandant_nr_)
                .and(TB_EINTRAG::datum.eq(datum_.clone())),
        )
        .first::<TbEintrag>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(p)
}

/// Get dataset by primary key.
pub fn get2(db: &DbContext, b: &TbEintrag) -> Result<Option<TbEintrag>> {
    let p = TB_EINTRAG::table
        .filter(
            TB_EINTRAG::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG::datum.eq(b.datum.clone())),
        )
        .first::<TbEintrag>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(p)
}

/// Get list.
#[allow(dead_code)]
pub fn get_list(db: &DbContext, mandant_nr_: i32) -> Result<Vec<TbEintrag>> {
    let list = TB_EINTRAG::table
        .filter(TB_EINTRAG::mandant_nr.eq(mandant_nr_))
        .load::<TbEintrag>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(list)
}

/// Insert a dataset.
pub fn insert<'a>(db: &mut DbContext, b: &'a TbEintrag) -> Result<&'a TbEintrag> {
    let rows = diesel::insert_into(TB_EINTRAG::table)
        .values(b)
        .execute(db.c)?;
    if rows <= 0 {
        return Err(RsbpError::NotFound);
    }
    db.ul.add(&UndoEntry::tb_eintrag(None, Some(b)));
    Ok(b)
}

/// Update a dataset.
pub fn update<'a>(db: &mut DbContext, b: &'a TbEintrag) -> Result<&'a TbEintrag> {
    let oo = get2(&db, b)?;
    let rows = diesel::update(
        TB_EINTRAG::table.filter(
            TB_EINTRAG::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG::datum.eq(b.datum.clone())),
        ),
    )
    .set((
        TB_EINTRAG::eintrag.eq(b.eintrag.as_str()),
        TB_EINTRAG::angelegt_von.eq(b.angelegt_von.as_ref()),
        TB_EINTRAG::angelegt_am.eq(b.angelegt_am),
        TB_EINTRAG::geaendert_von.eq(b.geaendert_von.as_ref()),
        TB_EINTRAG::geaendert_am.eq(b.geaendert_am),
        TB_EINTRAG::replikation_uid.eq(b.replikation_uid.as_ref()),
    ))
    .execute(db.c)?;
    if rows <= 0 || oo.is_none() {
        return Err(RsbpError::NotFound);
    }
    if let Some(o) = oo {
        db.ul.add(&UndoEntry::tb_eintrag(Some(&o), Some(b)));
    }
    Ok(b)
}

/// Delete a dataset.
pub fn delete(db: &mut DbContext, b: &TbEintrag) -> Result<()> {
    let oo = get2(db, b)?;
    let rows = diesel::delete(
        TB_EINTRAG::table.filter(
            TB_EINTRAG::mandant_nr
                .eq(b.mandant_nr)
                .and(TB_EINTRAG::datum.eq(b.datum.clone())),
        ),
    )
    .execute(db.c)?;
    if rows <= 0 || oo.is_none() {
        return Err(RsbpError::NotFound);
    }
    if let Some(o) = oo {
        db.ul.add(&UndoEntry::tb_eintrag(Some(&o), None));
    }
    Ok(())
}

/// Get list.
pub fn get_list_ext(
    db: &DbContext,
    mandant_nr_: &i32,
    from: &NaiveDate,
    to: &NaiveDate,
) -> Result<Vec<TbEintrag>> {
    let list = TB_EINTRAG::table
        .filter(
            TB_EINTRAG::mandant_nr
                .eq(mandant_nr_)
                .and(TB_EINTRAG::datum.ge(from))
                .and(TB_EINTRAG::datum.lt(to)),
        )
        .order_by(TB_EINTRAG::datum)
        .load::<TbEintrag>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(list)
}

/// Get search list.
pub fn get_list_search(
    db: &DbContext,
    dir: &SearchDirectionEnum,
    date: &Option<NaiveDate>,
    search: &[String; 9],
) -> Result<Vec<TbEintrag>> {
    let mut q = TB_EINTRAG::table
        .into_boxed()
        .filter(TB_EINTRAG::mandant_nr.eq(db.daten.mandant_nr));
    if let Some(d) = date {
        if *dir == SearchDirectionEnum::Back {
            q = q.filter(TB_EINTRAG::datum.lt(d));
        }
        if *dir == SearchDirectionEnum::Forward {
            q = q.filter(TB_EINTRAG::datum.gt(d));
        }
    }
    if !search[2].is_empty() {
        q = q.filter(
            TB_EINTRAG::eintrag
                .like(&search[0])
                .or(TB_EINTRAG::eintrag.like(&search[1]))
                .or(TB_EINTRAG::eintrag.like(&search[2])),
        )
    } else if !search[1].is_empty() {
        q = q.filter(
            TB_EINTRAG::eintrag
                .like(&search[0])
                .or(TB_EINTRAG::eintrag.like(&search[1])),
        )
    } else if !search[0].is_empty() {
        q = q.filter(TB_EINTRAG::eintrag.like(&search[0]))
    }
    if !search[5].is_empty() {
        q = q.filter(
            TB_EINTRAG::eintrag
                .like(&search[3])
                .or(TB_EINTRAG::eintrag.like(&search[4]))
                .or(TB_EINTRAG::eintrag.like(&search[5])),
        )
    } else if !search[4].is_empty() {
        q = q.filter(
            TB_EINTRAG::eintrag
                .like(&search[3])
                .or(TB_EINTRAG::eintrag.like(&search[4])),
        )
    } else if !search[3].is_empty() {
        q = q.filter(TB_EINTRAG::eintrag.like(&search[3]))
    }
    if !search[8].is_empty() {
        q = q.filter(
            TB_EINTRAG::eintrag
                .not_like(&search[6])
                .and(TB_EINTRAG::eintrag.not_like(&search[7]))
                .and(TB_EINTRAG::eintrag.not_like(&search[8])),
        )
    } else if !search[7].is_empty() {
        q = q.filter(
            TB_EINTRAG::eintrag
                .not_like(&search[6])
                .and(TB_EINTRAG::eintrag.not_like(&search[7])),
        )
    } else if !search[6].is_empty() {
        q = q.filter(TB_EINTRAG::eintrag.not_like(&search[6]))
    }
    if *dir == SearchDirectionEnum::Back || *dir == SearchDirectionEnum::Last {
        // DESC for MAX
        let list = q
            .order((TB_EINTRAG::datum.desc(),))
            .limit(functions::iif_i64(*dir == SearchDirectionEnum::None, -1, 1))
            .load::<TbEintrag>(db.c)
            .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
        return Ok(list);
    } else {
        // SearchDirectionEnum::None, SearchDirectionEnum::First, SearchDirectionEnum::Forward
        let list = q
            .order((TB_EINTRAG::datum,))
            .limit(functions::iif_i64(*dir == SearchDirectionEnum::None, -1, 1))
            .load::<TbEintrag>(db.c)
            .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
        return Ok(list);
    }
}
