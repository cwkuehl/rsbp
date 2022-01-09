use super::DbContext;
use crate::{
    apis::enums::SearchDirectionEnum, base::functions, config::RsbpError,
    services::undo::UndoEntry, Result,
};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{
    prelude::*,
    sql_query,
    sql_types::{Date, Integer, Text},
};
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
    puid: &Option<String>,
    from: &Option<NaiveDate>,
    to: &Option<NaiveDate>,
) -> Result<Vec<TbEintrag>> {
    // TODO natives SQL mit Join auf Tabelle TB_Eintrag_Ort
    let limit = functions::iif_i64(*dir == SearchDirectionEnum::None, -1, 1);
    let order = functions::iif(
        *dir == SearchDirectionEnum::Back || *dir == SearchDirectionEnum::Last,
        "a.datum desc",
        "a.datum",
    );
    let mut date1 = 0;
    let mut date2 = 0;
    let mut datesql = db.daten.get_today();
    let mut from1 = 0;
    let mut fromsql = db.daten.get_today();
    let mut to1 = 0;
    let mut tosql = db.daten.get_today();
    let mut puid1 = 0;
    let mut puidsql = "".to_string();
    let mut subsql = " AND (0=? OR a.angelegt_von=?)".to_string();
    if let Some(uid) = puid {
        if uid == "0" {
            subsql = " AND NOT EXISTS(SELECT * FROM TB_Eintrag_Ort b WHERE a.mandant_nr=b.mandant_nr AND b.datum_von<=a.datum AND a.datum<=b.datum_bis AND (0=? OR b.angelegt_von=?))".to_string();
        } else {
            puid1 = 1;
            puidsql = uid.to_string();
            subsql = " AND EXISTS(SELECT * FROM TB_Eintrag_Ort b WHERE a.mandant_nr=b.mandant_nr AND (0=? OR b.ort_uid=?) AND b.datum_von<=a.datum AND a.datum<=b.datum_bis)".to_string();
        }
    }
    let sql = format!( "SELECT a.mandant_nr mandant_nr, a.datum datum, a.eintrag eintrag, a.angelegt_von angelegt_von, a.angelegt_am angelegt_am, a.geaendert_von geaendert_von, a.geaendert_am geaendert_am, a.replikation_uid replikation_uid
      FROM TB_Eintrag a WHERE a.mandant_nr=? AND (0=? OR a.datum<?) AND (0=? OR a.datum>?) AND (0=? OR a.datum>=?) AND (0=? OR a.datum<=?){}
      AND ((0=? OR a.eintrag like ?) OR (0=? OR a.eintrag like ?) OR (0=? OR a.eintrag like ?))
      AND ((0=? OR a.eintrag like ?) OR (0=? OR a.eintrag like ?) OR (0=? OR a.eintrag like ?))
      AND (0=? OR NOT a.eintrag like ?) AND (0=? OR NOT a.eintrag like ?) AND (0=? OR NOT a.eintrag like ?)
      ORDER BY {} LIMIT {}", subsql, order, limit);
    if let Some(d0) = date {
        datesql = *d0;
        if *dir == SearchDirectionEnum::Back {
            date1 = 1;
        }
        if *dir == SearchDirectionEnum::Forward {
            date2 = 1;
        }
    }
    if let Some(d0) = from {
        from1 = 1;
        fromsql = *d0;
    }
    if let Some(d0) = to {
        to1 = 1;
        tosql = *d0;
    }
    let search1 = functions::iif_i32(search[0].is_empty(), 0, 1);
    let search2 = functions::iif_i32(search[1].is_empty(), 0, 1);
    let search3 = functions::iif_i32(search[2].is_empty(), 0, 1);
    let search4 = functions::iif_i32(search[3].is_empty(), 0, 1);
    let search5 = functions::iif_i32(search[4].is_empty(), 0, 1);
    let search6 = functions::iif_i32(search[5].is_empty(), 0, 1);
    let search7 = functions::iif_i32(search[6].is_empty(), 0, 1);
    let search8 = functions::iif_i32(search[7].is_empty(), 0, 1);
    let search9 = functions::iif_i32(search[8].is_empty(), 0, 1);
    let list = sql_query(sql)
        .bind::<Integer, _>(db.daten.mandant_nr)
        .bind::<Integer, _>(date1)
        .bind::<Date, _>(datesql)
        .bind::<Integer, _>(date2)
        .bind::<Date, _>(datesql)
        .bind::<Integer, _>(from1)
        .bind::<Date, _>(fromsql)
        .bind::<Integer, _>(to1)
        .bind::<Date, _>(tosql)
        .bind::<Integer, _>(puid1)
        .bind::<Text, _>(puidsql)
        .bind::<Integer, _>(search1)
        .bind::<Text, _>(search[0].to_string())
        .bind::<Integer, _>(search2)
        .bind::<Text, _>(search[1].to_string())
        .bind::<Integer, _>(search3)
        .bind::<Text, _>(search[2].to_string())
        .bind::<Integer, _>(search4)
        .bind::<Text, _>(search[3].to_string())
        .bind::<Integer, _>(search5)
        .bind::<Text, _>(search[4].to_string())
        .bind::<Integer, _>(search6)
        .bind::<Text, _>(search[5].to_string())
        .bind::<Integer, _>(search7)
        .bind::<Text, _>(search[6].to_string())
        .bind::<Integer, _>(search8)
        .bind::<Text, _>(search[7].to_string())
        .bind::<Integer, _>(search9)
        .bind::<Text, _>(search[8].to_string())
        .load::<TbEintrag>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError { source })?;
    Ok(list)
}
