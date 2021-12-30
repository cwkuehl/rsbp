pub mod ad_person;
pub mod benutzer;
pub mod ma_mandant;
pub mod ma_parameter;
pub mod tb_eintrag;
pub mod tb_eintrag_ort;
pub mod tb_ort;

use super::undo::UndoList;
use crate::{apis::services::ServiceDaten, res};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rsbp_rep::revision::Revision;

pub fn establish_connection<'a>(daten: &'a ServiceDaten) -> SqliteConnection {
    let database_url = String::from(daten.config.get_dbfilename());
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn mach_angelegt(
    e: &mut dyn Revision,
    daten: &ServiceDaten,
    von: &Option<String>,
    am: &Option<NaiveDateTime>,
) {
    if von.is_none() {
        e.set_angelegt_von(&Some(daten.benutzer_id.clone()));
        e.set_angelegt_am(&Some(daten.get_now()));
    } else {
        e.set_angelegt_von(von);
        e.set_angelegt_am(am);
    }
}

fn mach_geaendert(
    e: &mut dyn Revision,
    daten: &ServiceDaten,
    von: &Option<String>,
    am: &Option<NaiveDateTime>,
) {
    if von.is_none() {
        let mut datum: Option<NaiveDateTime> = e.get_geaendert_am();
        if datum.is_none() {
            datum = e.get_angelegt_am();
        }
        let mut dauer = res::AEND_ZEIT + 1;
        let jetzt = daten.get_now();
        if let Some(d) = datum {
            // println!("Jetzt: {}  Datum: {}", jetzt, d);
            dauer = jetzt.timestamp_millis() - d.timestamp_millis();
        }
        if datum.is_none() || dauer > res::AEND_ZEIT {
            e.set_geaendert_von(&Some(daten.benutzer_id.clone()));
            e.set_geaendert_am(&Some(jetzt));
        }
    } else {
        e.set_geaendert_von(von);
        e.set_geaendert_am(am);
    }
}

/// Zusammenfassen von ServiceDaten, Datenbank-Verbindung und UndoList, damit weniger Parameter übergeben werden müssen.
pub struct DbContext<'a> {
    pub daten: &'a ServiceDaten,
    pub c: &'a SqliteConnection,
    pub ul: UndoList,
}

impl<'a> DbContext<'a> {
    /// Initialisierung des Datenbank-Kontextes.
    /// * daten: Betroffene Service-Daten.
    /// * c: Betroffene Datenbank-Verbindung.
    pub fn new(daten: &'a ServiceDaten, c: &'a SqliteConnection) -> Self {
        DbContext {
            daten,
            c,
            ul: UndoList::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::functions;
    use heck::ToUpperCamelCase;
    use quick_xml::{events::Event, Reader};

    struct Column {
        name: String,
        type_: String,
        length: i32,
        nullable: bool,
        extension: bool,
        revision: bool,
        primary_key: bool,
    }

    struct Table {
        name: String,
        columns: Vec<Column>,
    }

    /// Generieren aller Dateien für Repositories.
    #[test]
    fn generate_reps() {
        let mut tables: Vec<Table> = Vec::new();
        read_tables(&mut tables);
        // Dateien generieren
        let t = tables
            .iter()
            .filter(|a| {
                !a.name.starts_with("HP_")
                    && !a.name.starts_with("MO_")
                    && !a.name.starts_with("VM_")
                    && a.name == "MA_Parameter"
            })
            .collect::<Vec<_>>();
        if functions::mach_nichts() == 0 {
            let sb = create_reps(&t);
            println!("{}", sb);
        } else {
            let sb = create_undo_entry(&t);
            println!("{}", sb);
            std::fs::write("/home/wolfgang/rust/rsbp/src/schema.rs", create_schema(&t)).unwrap();
            std::fs::write("/home/wolfgang/rust/rsbp/src/models.rs", create_models(&t)).unwrap();
        }
    }

    /// Repositories zusammenstellen.
    fn create_reps(tables: &Vec<&Table>) -> String {
        let mut sb = String::new();
        for t in tables.iter() {
            sb.push_str(
                format!(
                    "use super::DbContext;
use crate::{{config::RsbpError, services::undo::UndoEntry, Result}};
use chrono::{{NaiveDate, NaiveDateTime}};
use diesel::prelude::*;
use rsbp_rep::{{models::{}, schema::*}};
",
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );

            // Undo
            sb.push_str(
                format!(
                    "
/// Undo a dataset.
pub fn undo(db: &mut DbContext, or: &String, ac: &String) -> Result<()> {{
    let oo = UndoEntry::from_str::<{}>(or)?;
    let oa = UndoEntry::from_str::<{}>(ac)?;
    if let (Some(o), Some(_a)) = (&oo, &oa) {{
        // Update
        update(db, o)?;
    }} else if let Some(a) = &oa {{
        // Insert
        delete(db, a)?;
    }} else if let Some(o) = &oo {{
        // Delete
        insert(db, o)?;
    }}
    Ok(())
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );

            // Redo
            sb.push_str(
                format!(
                    "
/// Redo a dataset.
pub fn redo(db: &mut DbContext, or: &String, ac: &String) -> Result<()> {{
    let oo = UndoEntry::from_str::<{}>(or)?;
    let oa = UndoEntry::from_str::<{}>(ac)?;
    if let (Some(_o), Some(a)) = (&oo, &oa) {{
        // Update
        update(db, a)?;
    }} else if let Some(a) = &oa {{
        // Insert
        insert(db, a)?;
    }} else if let Some(o) = &oo {{
        // Delete
        delete(db, o)?;
    }}
    Ok(())
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );

            // Save0
            let parms = t
                .columns
                .iter()
                .map(|a| {
                    format!(
                        "
    {}_: &{}",
                        a.name.to_lowercase(),
                        get_rust_type(a)
                    )
                })
                .collect::<Vec<String>>()
                .join(",");
            let mut cf = 0;
            let filter = t
                .columns
                .iter()
                .filter(|a| a.primary_key)
                .map(|a| {
                    cf += 1;
                    format!(
                        "
            {}{}::{}.eq({}_{}){}",
                        functions::iif(cf > 1, ".and(", ""),
                        t.name.to_uppercase(),
                        a.name.to_lowercase(),
                        a.name.to_lowercase(),
                        get_rust_type_clone(a),
                        functions::iif(cf > 1, ")", ""),
                    )
                })
                .collect::<Vec<String>>()
                .join("");
            let init = t
                .columns
                .iter()
                .map(|a| {
                    if a.revision && !a.name.to_lowercase().starts_with("replikation_uid") {
                        return format!(
                            "
        {}: None,",
                            a.name.to_lowercase()
                        );
                    }
                    let cl = get_rust_type_clone(a);
                    format!(
                        "
        {}: {}{}_{},",
                        a.name.to_lowercase(),
                        functions::iif(cl.len() <= 0, "*", ""),
                        a.name.to_lowercase(),
                        cl,
                    )
                })
                .collect::<Vec<String>>()
                .join("");
            sb.push_str(
                format!(
                    "
/// Save dataset with all values.
#[allow(dead_code)]
pub fn save0(
    db: &mut DbContext,{}
) -> Result<{}> {{
    let op = {}::table
        .filter({},
        )
        .first::<{}>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError {{ source }})?;
    let mut p = {} {{{}
    }};
    if let Some(pu) = op {{
        if p != pu {{
        p.angelegt_von = pu.angelegt_von;
        p.angelegt_am = pu.angelegt_am;
        p.geaendert_von = pu.geaendert_von;
        p.geaendert_am = pu.geaendert_am;
        if p.angelegt_von.is_none() || !angelegt_von_.is_none() {{
                super::mach_angelegt(&mut p, db.daten, angelegt_von_, angelegt_am_);
            }}
            super::mach_geaendert(&mut p, db.daten, geaendert_von_, geaendert_am_);
            update(db, &p)?;
        }}
    }} else {{
        super::mach_angelegt(&mut p, db.daten, angelegt_von_, angelegt_am_);
        if !geaendert_von_.is_none() {{
            super::mach_geaendert(&mut p, db.daten, geaendert_von_, geaendert_am_);
        }}
        insert(db, &p)?;
    }}
    return Ok(p);
}}
",
                    parms,
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    filter,
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                    init,
                )
                .as_str(),
            );

            // Save
            let parms_rv = t
                .columns
                .iter()
                .filter(|a| !a.revision)
                .map(|a| {
                    format!(
                        "
    {}_: &{}",
                        a.name.to_lowercase(),
                        get_rust_type(a)
                    )
                })
                .collect::<Vec<String>>()
                .join(",");
            let parms2 = t
                .columns
                .iter()
                .map(|a| {
                    if a.revision {
                        return format!(
                            "
        &None,"
                        );
                    }
                    format!(
                        "
        {}_,",
                        a.name.to_lowercase(),
                    )
                })
                .collect::<Vec<String>>()
                .join("");
            sb.push_str(
                format!(
                    "
/// Save dataset without revision columns.
#[allow(dead_code)]
pub fn save(
    db: &mut DbContext,{}
) -> Result<{}> {{
    save0(
        db,{}
    )
}}
",
                    parms_rv,
                    t.name.to_upper_camel_case(),
                    parms2,
                )
                .as_str(),
            );

            // Get
            let parms_pk = t
                .columns
                .iter()
                .filter(|a| a.primary_key)
                .map(|a| {
                    format!(
                        "
    {}_: &{}",
                        a.name.to_lowercase(),
                        get_rust_type(a)
                    )
                })
                .collect::<Vec<String>>()
                .join(",");
            sb.push_str(
                format!(
                    "
/// Get dataset by primary key.
#[allow(dead_code)]
pub fn get(
    db: &DbContext,{}
) -> Result<Option<{}>> {{
    let p = {}::table
        .filter({},
        )
        .first::<{}>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError {{ source }})?;
    Ok(p)
}}
",
                    parms_pk,
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    filter,
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );

            // Get2
            cf = 0;
            let filter2 = t
                .columns
                .iter()
                .filter(|a| a.primary_key)
                .map(|a| {
                    cf += 1;
                    format!(
                        "
            {}{}::{}.eq(b.{}{}){}",
                        functions::iif(cf > 1, ".and(", ""),
                        t.name.to_uppercase(),
                        a.name.to_lowercase(),
                        a.name.to_lowercase(),
                        get_rust_type_clone(a),
                        functions::iif(cf > 1, ")", ""),
                    )
                })
                .collect::<Vec<String>>()
                .join("");
            sb.push_str(
                format!(
                    "
/// Get dataset by primary key.
pub fn get2(db: &DbContext, b: &{}) -> Result<Option<{}>> {{
    let p = {}::table
        .filter({},
        )
        .first::<{}>(db.c)
        .optional()
        .map_err(|source: diesel::result::Error| RsbpError::DieselError {{ source }})?;
    Ok(p)
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    filter2,
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );

            // Get_List
            let with_client = t
                .columns
                .iter()
                .any(|a| a.name.to_lowercase() == "mandant_nr");
            sb.push_str(
                format!(
                    "
/// Get list.
#[allow(dead_code)]
pub fn get_list(db: &DbContext{}) -> Result<Vec<{}>> {{
    let list = {}::table{}
        .load::<{}>(db.c)
        .map_err(|source: diesel::result::Error| RsbpError::DieselError {{ source }})?;
    Ok(list)
}}
",
                    functions::iif(with_client, ", mandant_nr_: i32", ""),
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    functions::iif(
                        with_client,
                        format!(
                            "
        .filter({}::mandant_nr.eq(mandant_nr_))",
                            t.name.to_uppercase()
                        )
                        .as_str(),
                        ""
                    ),
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );

            // Insert
            sb.push_str(
                format!(
                    "
/// Insert a dataset.
pub fn insert<'a>(db: &mut DbContext, b: &'a {}) -> Result<&'a {}> {{
    let rows = diesel::insert_into({}::table).values(b).execute(db.c)?;
    if rows <= 0 {{
        return Err(RsbpError::NotFound);
    }}
    db.ul.add(&UndoEntry::{}(None, Some(b)));
    Ok(b)
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    t.name.to_lowercase(),
                )
                .as_str(),
            );

            // Update
            let set = t
                .columns
                .iter()
                .filter(|a| !a.primary_key)
                .map(|a| {
                    cf += 1;
                    format!(
                        "
        {}::{}.eq(b.{}{}),",
                        t.name.to_uppercase(),
                        a.name.to_lowercase(),
                        a.name.to_lowercase(),
                        get_rust_type_as_ref(a),
                    )
                })
                .collect::<Vec<String>>()
                .join("");
            sb.push_str(
                format!(
                    "
/// Update a dataset.
pub fn update<'a>(db: &mut DbContext, b: &'a {}) -> Result<&'a {}> {{
    let oo = get2(&db, b)?;
    let rows = diesel::update(
        {}::table.filter({},
        ),
    )
    .set(({}
    ))
    .execute(db.c)?;
    if rows <= 0 || oo.is_none() {{
        return Err(RsbpError::NotFound);
    }}
    if let Some(o) = oo {{
        db.ul.add(&UndoEntry::{}(Some(&o), Some(b)));
    }}
    Ok(b)
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    filter2,
                    set,
                    t.name.to_lowercase(),
                )
                .as_str(),
            );

            // Delete
            sb.push_str(
                format!(
                    "
/// Delete a dataset.
pub fn delete(db: &mut DbContext, b: &{}) -> Result<()> {{
    let oo = get2(db, b)?;
    let rows = diesel::delete(
        {}::table.filter({},
        ),
    )
    .execute(db.c)?;
    if rows <= 0 || oo.is_none() {{
        return Err(RsbpError::NotFound);
    }}
    if let Some(o) = oo {{
        db.ul.add(&UndoEntry::{}(Some(&o), None));
    }}
    Ok(())
}}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_uppercase(),
                    filter2,
                    t.name.to_lowercase(),
                )
                .as_str(),
            );
        }
        sb
    }

    /// UndoEntry zusammenstellen.
    fn create_undo_entry(tables: &Vec<&Table>) -> String {
        let mut sb = String::new();
        let j = tables
            .iter()
            .map(|a| a.name.to_upper_camel_case())
            .collect::<Vec<String>>()
            .join(", ");
        sb.push_str(
            format!(
                "use super::reps::DbContext;
use crate::{{
    base::functions,
    models::{{{}}},
    services::reps,
    Result,
}};
use lazy_static::lazy_static;
use serde::{{Deserialize, Serialize}};
use std::sync::{{Arc, RwLock}};

#[derive(Clone, Debug)]
pub enum UndoEntry {{
",
                j.as_str()
            )
            .as_str(),
        );
        for t in tables.iter() {
            sb.push_str(
                format!(
                    "    {} {{ original: String, actual: String }},
",
                    t.name.to_upper_camel_case()
                )
                .as_str(),
            );
        }
        sb.push_str(
            "}

impl UndoEntry {
",
        );
        for t in tables.iter() {
            sb.push_str(
                format!(
                    "    pub fn {}(original: Option<&{}>, actual: Option<&{}>) -> Self {{
        UndoEntry::{} {{
            original: UndoEntry::to_string(original),
            actual: UndoEntry::to_string(actual),
        }}
    }}
",
                    t.name.to_lowercase(),
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );
        }
        sb.push_str(
            "}
",
        );

        // Undo
        for t in tables.iter() {
            sb.push_str(
                format!(
                    "    UndoEntry::{} {{ original, actual }} => {{
        reps::{}::undo(db, original, actual)?;
    }}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_lowercase(),
                )
                .as_str(),
            );
        }

        // Redo
        for t in tables.iter() {
            sb.push_str(
                format!(
                    "    UndoEntry::{} {{ original, actual }} => {{
        reps::{}::redo(db, original, actual)?;
    }}
",
                    t.name.to_upper_camel_case(),
                    t.name.to_lowercase(),
                )
                .as_str(),
            );
        }
        sb
    }

    /// Datei schema.rs zusammenstellen.
    fn create_schema(tables: &Vec<&Table>) -> String {
        let mut sb = r#"use diesel::table;
"#
        .to_string();
        for t in tables.iter() {
            // Table
            let mut pk = String::new();
            for (i, c) in t.columns.iter().enumerate() {
                if c.primary_key {
                    if i > 0 {
                        pk.push_str(", ");
                    }
                    pk.push_str(c.name.to_lowercase().as_str());
                }
            }
            sb.push_str(
                format!(
                    r#"
table! {{
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    {} ({}) {{
"#,
                    t.name.to_uppercase(),
                    pk,
                )
                .as_str(),
            );
            for c in t.columns.iter() {
                sb.push_str(
                    format!(
                        r#"        {} -> {},
"#,
                        c.name.to_lowercase(),
                        get_diesel_type(c),
                    )
                    .as_str(),
                );
            }
            sb.push_str(
                r#"    }
}
"#,
            );
        }
        sb
    }

    /// Datei models.rs zusammenstellen.
    fn create_models(tables: &Vec<&Table>) -> String {
        let j = tables
            .iter()
            .map(|a| a.name.to_uppercase())
            .collect::<Vec<String>>()
            .join(", ");
        let mut sb = format!(
            r#"use crate::{{
    apis::revision::Revision,
    schema::{{{}}},
}};
use chrono::{{NaiveDate, NaiveDateTime}};
use serde::{{Deserialize, Serialize}};
"#,
            j
        );
        for t in tables.iter() {
            // Model
            sb.push_str(
                format!(
                    r#"
#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "{}"]
#[allow(non_snake_case)]
pub struct {} {{
"#,
                    t.name.to_uppercase(),
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );
            for c in t.columns.iter() {
                sb.push_str(
                    format!(
                        r#"    pub {}: {},
"#,
                        c.name.to_lowercase(),
                        get_rust_type(c),
                    )
                    .as_str(),
                );
            }
            sb.push_str(
                r#"}
"#,
            );

            // Clone
            sb.push_str(
                format!(
                    r#"
impl Clone for {} {{
    fn clone(&self) -> Self {{
        Self {{
"#,
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );
            for c in t.columns.iter() {
                sb.push_str(
                    format!(
                        r#"            {}: self.{}{},
"#,
                        c.name.to_lowercase(),
                        c.name.to_lowercase(),
                        get_rust_type_clone(c),
                    )
                    .as_str(),
                );
            }
            sb.push_str(
                r#"        }
    }
}
"#,
            );

            // PartialEq, Vergleich ohne Revisionsdaten
            sb.push_str(
                format!(
                    r#"
impl PartialEq for {} {{
    fn eq(&self, other: &Self) -> bool {{
"#,
                    t.name.to_upper_camel_case(),
                )
                .as_str(),
            );
            for (i, c) in t.columns.iter().filter(|a| !a.revision).enumerate() {
                sb.push_str(
                    format!(
                        r#"        {}self.{} == other.{}
"#,
                        functions::iif(i == 0, "", "    && "),
                        c.name.to_lowercase(),
                        c.name.to_lowercase(),
                    )
                    .as_str(),
                );
            }
            sb.push_str(
                r#"    }
}
"#,
            );

            if !t.name.starts_with("SO_") {
                // Revision
                sb.push_str(
                    format!(
                        r#"
impl Revision for {} {{
    fn get_angelegt_von(&self) -> Option<String> {{
        self.angelegt_von.clone()
    }}
    fn set_angelegt_von(&mut self, von: &Option<String>) {{
        self.angelegt_von = von.clone();
    }}
    fn get_angelegt_am(&self) -> Option<NaiveDateTime> {{
        self.angelegt_am
    }}
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>) {{
        self.angelegt_am = am.clone();
    }}
    fn get_geaendert_von(&self) -> Option<String> {{
        self.geaendert_von.clone()
    }}
    fn set_geaendert_von(&mut self, von: &Option<String>) {{
        self.geaendert_von = von.clone();
    }}
    fn get_geaendert_am(&self) -> Option<NaiveDateTime> {{
        self.geaendert_am
    }}
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>) {{
        self.geaendert_am = am.clone();
    }}
}}
"#,
                        t.name.to_upper_camel_case(),
                    )
                    .as_str(),
                );
            }
        }
        sb.to_string()
    }

    /// Liefert den diesel-Typ einer Spalte.
    fn get_diesel_type(c: &Column) -> String {
        let t = match c.type_.as_str() {
            "INTEGER" => "Integer",
            "VARCHAR" => "Text",
            "DATE" => "Date",
            "TIMESTAMP" => "Timestamp",
            "BOOLEAN" => "Bool",
            "DECIMAL(21,4)" => "Double",
            "BLOB" => "Binary",
            _ => c.type_.as_str(),
        };
        if c.nullable {
            return format!("Nullable<{}>", t);
        }
        t.to_string()
    }

    /// Liefert den rust-Typ einer Spalte.
    fn get_rust_type(c: &Column) -> String {
        let t = match c.type_.as_str() {
            "INTEGER" => "i32",
            "VARCHAR" => "String",
            "DATE" => "NaiveDate",
            "TIMESTAMP" => "NaiveDateTime",
            "BOOLEAN" => "bool",
            "DECIMAL(21,4)" => "f64",
            "BLOB" => "Vec<u8>",
            _ => c.type_.as_str(),
        };
        if c.nullable {
            return format!("Option<{}>", t);
        }
        if c.length < 0 || c.extension {
            functions::mach_nichts();
        }
        t.to_string()
    }

    /// Liefert die clone-Funktion einer Spalte.
    fn get_rust_type_clone(c: &Column) -> String {
        let t = match c.type_.as_str() {
            "INTEGER" => "",
            "VARCHAR" => ".clone()",
            "DATE" => ".clone()",
            "TIMESTAMP" => ".clone()",
            "BOOLEAN" => "",
            "DECIMAL(21,4)" => "",
            "BLOB" => ".clone()",
            _ => c.type_.as_str(),
        };
        t.to_string()
    }

    /// Liefert die as_ref-Funktion einer Spalte.
    fn get_rust_type_as_ref(c: &Column) -> String {
        let t = match c.type_.as_str() {
            "INTEGER" => "",
            "VARCHAR" => functions::iif(c.nullable, ".as_ref()", ".as_str()"),
            "DATE" => "",      //".clone()",
            "TIMESTAMP" => "", //".clone()",
            "BOOLEAN" => "",
            "DECIMAL(21,4)" => "",
            "BLOB" => "", //".clone()",
            _ => c.type_.as_str(),
        };
        t.to_string()
    }

    /// Lesen der Datei tables.xml.
    fn read_tables(tables: &mut Vec<Table>) {
        let tables_src = include_str!("../res/tables.xml");
        let mut reader = Reader::from_str(tables_src);
        reader.trim_text(true);

        let mut data = false;
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"table" => {
                        let mut a = e.attributes().filter(|a| {
                            std::str::from_utf8(a.as_ref().unwrap().key).unwrap() == "name"
                        });
                        if let Some(Ok(a2)) = a.next() {
                            let key = a2.value;
                            let t = Table {
                                name: std::str::from_utf8(&key).unwrap().to_string(),
                                columns: Vec::new(),
                            };
                            // println!("table {}", t.name);
                            tables.push(t);
                            data = true;
                        }
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name() {
                    b"column" => {
                        let atts = e
                            .attributes()
                            .map(|a| {
                                let at = a.unwrap();
                                (
                                    std::str::from_utf8(at.key).unwrap(),
                                    std::str::from_utf8(&at.value).unwrap().to_string(),
                                )
                            })
                            .collect::<Vec<_>>();
                        let mut c = Column {
                            name: get_attribut_value(&atts, "name"),
                            type_: get_attribut_value(&atts, "type"),
                            length: functions::to_i32(get_attribut_value(&atts, "length").as_str()),
                            nullable: get_attribut_value(&atts, "nullable") == "true",
                            extension: get_attribut_value(&atts, "extension") == "true",
                            revision: false,
                            primary_key: false,
                        };
                        if c.name.to_lowercase().starts_with("angelegt_")
                            || c.name.to_lowercase().starts_with("geaendert_")
                            || c.name.to_lowercase().starts_with("replikation_uid")
                        {
                            c.revision = true;
                        }
                        //     // println!(
                        //     //     "  column {} {} {} {}",
                        //     //     c.name, c.type_, c.nullable, c.primary_key
                        //     // );
                        if let Some(t) = tables.last_mut() {
                            t.columns.push(c);
                        }
                    }
                    b"keycolumn" => {
                        if let Some(Ok(a)) = e.attributes().into_iter().next() {
                            if let Ok(name) = std::str::from_utf8(&a.value) {
                                if let Some(t) = tables.last_mut() {
                                    if let Some(c) =
                                        t.columns.iter_mut().filter(|a| a.name == name).next()
                                    {
                                        // println!("    keycolumn {}", c.name);
                                        c.primary_key = true;
                                    }
                                }
                            }
                        }
                    }
                    _ => (),
                },
                Ok(Event::Text(_e)) => {
                    if data {
                        // let v = e.unescape_and_decode(&reader).unwrap();
                        // sb.push_str(v.as_str());
                        // sb.push_str("\"#,\n");
                        // data = false;
                        // println!("value: {:?}", v);
                    }
                }
                Ok(Event::End(ref e)) => match e.name() {
                    b"table" => {
                        data = false;
                    }
                    _ => (),
                },
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other events we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
    }

    fn get_attribut_value(atts: &Vec<(&str, String)>, key: &str) -> String {
        if let Some(att) = atts.iter().filter(|a| a.0 == key).next() {
            return att.1.to_string();
        }

        // let mut a = atts.filter(|a| std::str::from_utf8(a.as_ref().unwrap().key).unwrap() == key);
        // if let Some(Ok(a2)) = a.next() {
        //     let v = a2.value;
        //     return std::str::from_utf8(&v).unwrap().to_string();
        // }
        "".into()
    }
}
