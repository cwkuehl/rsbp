use super::{
    reps::{self, DbContext},
    undo::UndoRedoStack,
};
use crate::{
    apis::{enums::PermissionEnum, services::ServiceDaten},
    base::functions,
    config::RsbpError,
    res::{self, messages::M},
    Result,
};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{sql_query, Connection, RunQueryDsl};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use rsbp_rep::models::{Benutzer, MaMandant, TbEintrag};
use std::cmp;

/// Initialize the database.
/// * daten: Service data for database access.
/// * returns: Possibly errors.
pub fn init_db<'a>(daten: &'a ServiceDaten) -> Result<()> {
    let c = reps::establish_connection(daten);
    // let db = DbContext::new(daten, &c);
    // diesel error (trailing input)
    // let _q = sql_query("update ad_person set geburt=date(geburt) where not geburt is null")
    //     .execute(&c)?;
    let _q = sql_query("update benutzer set geburt=date(geburt) where not geburt is null")
        .execute(&c)?;
    // let _q =
    //     sql_query("update tb_eintrag set datum=date(datum) where not datum is null").execute(&c)?;
    // let _q = sql_query(
    //     "update tb_eintrag_ort set datum_von=date(datum_von) where not datum_von is null",
    // )
    // .execute(&c)?;
    // let _q = sql_query(
    //     "update tb_eintrag_ort set datum_bis=date(datum_bis) where not datum_bis is null",
    // )
    // .execute(&c)?;
    // TODO init_db
    Ok(())
}

/// Get list with clients.
/// * daten: Service data for database access.
/// * returns: List with clients.
pub fn get_client_list<'a>(daten: &'a ServiceDaten) -> Result<Vec<MaMandant>> {
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    if functions::mach_nichts() != 0 {
        return Err(RsbpError::NotFound);
    }
    let l = reps::ma_mandant::get_list(&db)?;
    Ok(l)
}

/// Get client by number.
/// * daten: Service data for database access.
/// * nr: Affected client number.
/// * returns: client or none.
pub fn get_client<'a>(daten: &'a ServiceDaten, nr: i32) -> Result<Option<MaMandant>> {
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    let l = reps::ma_mandant::get(&db, &nr)?;
    Ok(l)
}

/// Save a client.
/// * daten: Service data for database access.
/// * nr: Affected client number.
/// * desc: Affected description.
/// * returns: Saved client.
pub fn save_client<'a>(daten: &'a ServiceDaten, nr: i32, desc: &str) -> Result<MaMandant> {
    let d = desc.trim().to_string();
    if d.is_empty() {
        return Err(RsbpError::error_msg(M::AM008, daten.config.is_de()));
    }
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<MaMandant, RsbpError, _>(|| {
        let mut m_nr = nr;
        if nr <= 0 {
            let l = reps::ma_mandant::get_list(&db)?;
            if let Some(last) = l.last() {
                m_nr = last.nr + 1;
            } else {
                m_nr = 1;
            }
            // Einen Benutzer anlegen.
            let benutzer_id_ = &res::USER_ID.to_string();
            let ob = reps::benutzer::get(&db, &m_nr, benutzer_id_)?;
            if ob.is_none() {
                reps::benutzer::save(
                    &mut db,
                    &m_nr,
                    benutzer_id_,
                    &None,
                    &PermissionEnum::to_i32(PermissionEnum::Admin),
                    &0,
                    &0,
                    &None,
                )?;
            }
        }
        let e = reps::ma_mandant::save(&mut db, &m_nr, &d)?;
        Ok(e)
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }
    tr
}

/// Delete a client.
/// * daten: Service data for database access.
/// * e: Affected Entity.
/// * returns: Possibly errors.
pub fn delete_client<'a>(daten: &'a ServiceDaten, e: &MaMandant) -> Result<()> {
    if e.nr == daten.mandant_nr {
        return Err(RsbpError::error_msg(M::AM004, daten.config.is_de()));
    }
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<(), RsbpError, _>(|| {
        // Delete client in all tables.
        for t in get_all_tables() {
            if !(t.name == "Benutzer" || t.name == "MA_Mandant") && t.delete {
                let s = format!("DELETE FROM {} WHERE {}={}", t.name, t.client_number, e.nr);
                c.execute(s.as_str())
                    .map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
            }
        }
        let blist = reps::benutzer::get_list(&db, e.nr)?;
        for b in blist {
            reps::benutzer::delete(&mut db, &b)?;
        }
        reps::ma_mandant::delete(&mut db, e)?;
        Ok(())
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }
    tr
}

/// Get list with users.
/// * daten: Service data for database access.
/// * returns: List with users.
pub fn get_user_list<'a>(daten: &'a ServiceDaten) -> Result<Vec<Benutzer>> {
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    let l = reps::benutzer::get_list(&db, daten.mandant_nr)?;
    Ok(l)
}

/// Get user by number.
/// * daten: Service data for database access.
/// * nr: Affected client number.
/// * returns: user or none.
pub fn get_user<'a>(daten: &'a ServiceDaten, nr: i32) -> Result<Option<Benutzer>> {
    let c = reps::establish_connection(daten);
    let db = DbContext::new(daten, &c);
    let l = reps::benutzer::get_ext(&db, &daten.mandant_nr, &nr)?;
    Ok(l)
}

/// Save a user.
/// * daten: Service data for database access.
/// * nr: Affected user number.
/// * id: Affected user id.
/// * password: Affected password.
/// * permission: Affected permission.
/// * birthday: Affected birthday.
/// * returns: Saved user.
pub fn save_user<'a>(
    daten: &'a ServiceDaten,
    nr: i32,
    userid: &String,
    password: &Option<String>,
    permission: &i32,
    birthday: &Option<NaiveDate>,
) -> Result<Benutzer> {
    let de = daten.config.is_de();
    let id = userid.trim().to_string();
    if id.is_empty() {
        return Err(RsbpError::error_msg(M::AM009, de));
    }
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<Benutzer, RsbpError, _>(|| {
        let p = get_permission(&db, &daten.mandant_nr, &daten.benutzer_id)?;
        if p < *permission {
            return Err(RsbpError::error_msg(M::AM010, de));
        }
        let mut periode = 0;
        if let Some(e) = reps::benutzer::get(&db, &daten.mandant_nr, &id)? {
            periode = e.akt_periode;
        }
        let mut unr = nr;
        let list = reps::benutzer::get_list_ext(&db, &daten.mandant_nr, &0, Some(&id), &unr)?;
        if list.len() > 0 {
            return Err(RsbpError::error_msg(M::AM011, de));
        }
        if unr <= 0 {
            unr = 1;
            let list = reps::benutzer::get_list_ext(&db, &daten.mandant_nr, &0, None, &0)?;
            for u in list.iter() {
                if u.person_nr >= unr {
                    unr = u.person_nr + 1;
                }
            }
        }
        let e = reps::benutzer::save(
            &mut db,
            &daten.mandant_nr,
            &id,
            password,
            permission,
            &periode,
            &unr,
            &birthday,
        )?;
        Ok(e)
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }
    tr
}

/// Read permission of a user.
/// * db: Context for database access.
/// * mandant_nr_: Affected client number.
/// * benutzer_id_: Affected user id.
/// * returns: user permission.
fn get_permission(db: &DbContext, mandant_nr_: &i32, benutzer_id_: &String) -> Result<i32> {
    if let Some(b) = reps::benutzer::get(db, mandant_nr_, benutzer_id_)? {
        return Ok(b.berechtigung);
    }
    Ok(-1)
}

/// Delete a user.
/// * daten: Service data for database access.
/// * e: Affected Entity.
/// * returns: Possibly errors.
pub fn delete_user<'a>(daten: &'a ServiceDaten, e: &Benutzer) -> Result<()> {
    if e.mandant_nr == daten.mandant_nr && daten.benutzer_id == e.benutzer_id {
        return Err(RsbpError::error_msg(M::AM012, daten.config.is_de()));
    }
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<(), RsbpError, _>(|| {
        reps::benutzer::delete(&mut db, e)?;
        Ok(())
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }
    tr
}

/// Replicate a table.
/// * daten: Service data for database access.
/// * table: Affected table.
/// * mode: Affected mode.
/// * json: Affected data as json string.
/// * returns: new table data as json string.
pub fn replicate_table<'a>(
    daten: &'a ServiceDaten,
    table: &'a String,
    mode: &'a String,
    json: &'a String,
) -> Result<String> {
    lazy_static! {
        static ref RE_READ: Regex = RegexBuilder::new("^([a-z]+)(_([0-9]+)d?)?$")
            .case_insensitive(true)
            .build()
            .unwrap();
    }
    if mode.is_empty() {
        return Err(RsbpError::error_string("Missing mode"));
    }
    if table.is_empty() {
        return Err(RsbpError::error_string("Missing table"));
    }
    // if json.is_empty() {
    //     return Err(RsbpError::error_string("Missing json data"));
    // }
    let mut days = 1;
    if let Some(c) = RE_READ.captures(mode.as_str()) {
        days = cmp::max(functions::to_i32(c[3].to_string().as_str()), 1);
    }
    if days == 0 {
        functions::mach_nichts();
    }
    let c = reps::establish_connection(daten);
    let mut db = DbContext::new(daten, &c);
    let tr = c.transaction::<String, RsbpError, _>(|| {
        if table == "TB_Eintrag" {
            let jo: serde_json::Value = serde_json::from_str(json.as_str())
                .map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
            if let Some(jarr) = jo[table].as_array() {
                for a in jarr {
                    let e = TbEintrag {
                        mandant_nr: daten.mandant_nr,
                        datum: json_nd(a, "datum")?,
                        eintrag: json_str(a, "eintrag")?,
                        angelegt_von: json_ostr(a, "angelegtVon")?,
                        angelegt_am: json_ondt(a, "angelegtAm")?,
                        geaendert_von: json_ostr(a, "geaendertVon")?,
                        geaendert_am: json_ondt(a, "geaendertAm")?,
                        replikation_uid: None,
                    };
                    if let Some(mut es) = reps::tb_eintrag::get(&db, &e.mandant_nr, &e.datum)? {
                        if es.eintrag != e.eintrag {
                            // Wenn es.angelegtAm != e.angelegtAm, Einträge zusammenkopieren
                            // Wenn es.angelegtAm == e.angelegtAm und (e.geaendertAm == null oder es.geaendertAm > e.geaendertAm), Eintrag lassen
                            // Wenn es.angelegtAm == e.angelegtAm und es.geaendertAm <= e.geaendertAm, Eintrag überschreiben
                            let mut zusammenkopieren = false;
                            let mut lassen = false;
                            // if (e.Angelegt_Am.HasValue
                            //     && (!es.Angelegt_Am.HasValue || es.Angelegt_Am != e.Angelegt_Am))
                            if let Some(eaa) = e.angelegt_am {
                                if let Some(esaa) = es.angelegt_am {
                                    if eaa != esaa {
                                        zusammenkopieren = true;
                                    }
                                } else {
                                    zusammenkopieren = true;
                                }
                            }
                            if !zusammenkopieren {
                                // else if (es.Angelegt_Am.HasValue && e.Angelegt_Am.HasValue && es.Angelegt_Am == e.Angelegt_Am
                                //   && es.Geaendert_Am.HasValue && (!e.Geaendert_Am.HasValue || es.Geaendert_Am > e.Geaendert_Am))
                                if let Some(esaa) = es.angelegt_am {
                                    if let Some(eaa) = e.angelegt_am {
                                        if esaa == eaa {
                                            if let Some(esga) = es.geaendert_am {
                                                if let Some(ega) = e.geaendert_am {
                                                    if esga > ega {
                                                        lassen = true;
                                                    }
                                                } else {
                                                    lassen = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if zusammenkopieren {
                                // Zusammenkopieren
                                es.eintrag = format!(
                                    "Server: {}
Lokal: {}",
                                    es.eintrag, e.eintrag
                                )
                                .to_string();
                                es.angelegt_am = e.angelegt_am;
                                es.angelegt_von = e.angelegt_von;
                                es.geaendert_am = e.geaendert_am;
                                es.geaendert_von = e.geaendert_von;
                                println!("Zk {:?}", es);
                                reps::tb_eintrag::save0(
                                    &mut db,
                                    &es.mandant_nr,
                                    &es.datum,
                                    &es.eintrag,
                                    &es.angelegt_von,
                                    &es.angelegt_am,
                                    &es.geaendert_von,
                                    &es.geaendert_am,
                                    &es.replikation_uid,
                                )?;
                            } else if !lassen {
                                // Überschreiben
                                es.eintrag = e.eintrag;
                                es.angelegt_am = e.angelegt_am;
                                es.angelegt_von = e.angelegt_von;
                                es.geaendert_am = e.geaendert_am;
                                es.geaendert_von = e.geaendert_von;
                                println!("Üs {:?}", es);
                                reps::tb_eintrag::save0(
                                    &mut db,
                                    &es.mandant_nr,
                                    &es.datum,
                                    &es.eintrag,
                                    &es.angelegt_von,
                                    &es.angelegt_am,
                                    &es.geaendert_von,
                                    &es.geaendert_am,
                                    &es.replikation_uid,
                                )?;
                            }
                        }
                    } else {
                        println!("Neu {:?}", e);
                        reps::tb_eintrag::save0(
                            &mut db,
                            &e.mandant_nr,
                            &e.datum,
                            &e.eintrag,
                            &e.angelegt_von,
                            &e.angelegt_am,
                            &e.geaendert_von,
                            &e.geaendert_am,
                            &e.replikation_uid,
                        )?;
                    }
                }
            }
        } else if table == "FZ_Notiz" {
            // TODO Replikationen ergänzen
        } else if table == "HH_Buchung" {
            //
        } else if table == "HH_Ereignis" {
            //
        } else if table == "HH_Konto" {
            //
        } else if table == "FZ_Fahrrad" {
            //
        } else if table == "FZ_Fahrradstand" {
            //
        } else {
            return Err(RsbpError::error_string(
                format!("Unknown table {}", table).as_str(),
            ));
        }
        Ok("".into())
    });
    if tr.is_ok() {
        UndoRedoStack::add_undo(&mut db.ul);
    }
    tr
}

fn json_ostr(v: &serde_json::Value, key: &str) -> Result<Option<String>> {
    let data = v[key].as_str();
    if let Some(s) = data {
        return Ok(Some(s.to_string()));
    }
    Ok(None)
}

fn json_str(v: &serde_json::Value, key: &str) -> Result<String> {
    let data = v[key].as_str();
    if let Some(s) = data {
        return Ok(s.to_string());
    }
    Err(RsbpError::error_string(
        format!(
            "Missing or wrong json String value for key {}: '{:?}'",
            key, data
        )
        .as_str(),
    ))
}

fn json_nd(v: &serde_json::Value, key: &str) -> Result<NaiveDate> {
    let data = v[key].as_str();
    let ond = functions::ostr_to_ond(data);
    if let Some(nd) = ond {
        return Ok(nd);
    }
    Err(RsbpError::error_string(
        format!(
            "Missing or wrong json NaiveDate value for key {}: '{:?}'",
            key, data
        )
        .as_str(),
    ))
}

// fn json_ond(v: &serde_json::Value, key: &str) -> Result<Option<NaiveDate>> {
//     let data = v[key].as_str();
//     let ond = functions::ostr_to_ond(data);
//     Ok(ond)
// }

fn json_ondt(v: &serde_json::Value, key: &str) -> Result<Option<NaiveDateTime>> {
    let data = v[key].as_str();
    let ondt = json_ostr_to_ondt(data);
    Ok(ondt)
}

/// Wandelt einen optionalen String in optionales Datum mit Uhrzeit um.
/// * s: Zu konvertierender String.
pub fn json_ostr_to_ondt(s: Option<&str>) -> Option<NaiveDateTime> {
    if let Some(str) = s {
        if let Ok(d) = NaiveDateTime::parse_from_str(str, "%Y-%m-%dT%H:%M:%S.000Z") {
            // NaiveDateTime -> UTC...Z -> Local...+02:00 -> NaiveDateTime
            use chrono::{offset::TimeZone, DateTime, Local, LocalResult::Single, Utc};
            if let Single(utc) = Utc.from_local_datetime(&d) {
                // println!("utc {:?}", utc);
                let dl: DateTime<Local> = DateTime::from(utc);
                // println!("dl  {:?}", dl);
                let nl = dl.naive_local();
                // println!("nl {:?}", nl);
                return Some(nl);
            }
        }
    }
    None
}

/// Get all tables.
fn get_all_tables<'a>() -> Vec<ReplicationTable<'a>> {
    let v: Vec<ReplicationTable> = vec![
        ReplicationTable {
            name: "AD_Adresse",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "AD_Person",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "AD_Sitz",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "Benutzer",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Benutzer_Id",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "Byte_Daten",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Typ, Uid, Lfd_Nr",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "FZ_Buch",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "FZ_Buchautor",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "FZ_Buchserie",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "FZ_Buchstatus",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Buch_Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "FZ_Fahrrad",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "FZ_Fahrradstand",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Fahrrad_Uid, Datum, Nr",
            delete: true,
            copy: true,
        },
        // ReplicationTable {
        //     name: "FZ_Lektion",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "FZ_Lektioninhalt",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Lektion_Uid, Lfd_Nr",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "FZ_Lektionstand",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Lektion_Uid",
        //     delete: true,
        //     copy: true,
        // },
        ReplicationTable {
            name: "FZ_Notiz",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "HH_Bilanz",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Periode, Kz, Konto_Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "HH_Buchung",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "HH_Ereignis",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "HH_Konto",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "HH_Periode",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Nr",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "MA_Mandant",
            client_number: "Nr",
            primary_key: "Nr",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "MA_Parameter",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Schluessel",
            delete: true,
            copy: true,
        },
        // ReplicationTable {
        //     name: "MA_Replikation",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Tabellen_Nr, Replikation_Uid",
        //     delete: true,
        //     copy: false,
        // },
        // ReplicationTable {
        //     name: "MO_Einteilung",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "MO_Gottesdienst",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "MO_Messdiener",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "MO_Profil",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        ReplicationTable {
            name: "SB_Ereignis",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Person_Uid, Familie_Uid, Typ",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "SB_Familie",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "SB_Kind",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Familie_Uid, Kind_Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "SB_Person",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "SB_Quelle",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "TB_Eintrag",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Datum",
            delete: true,
            copy: true,
        },
        // ReplicationTable {
        //     name: "VM_Abrechnung",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "VM_Buchung",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "VM_Ereignis",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "VM_Haus",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "VM_Konto",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "VM_Miete",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "VM_Mieter",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        // ReplicationTable {
        //     name: "VM_Wohnung",
        //     client_number: "Mandant_Nr",
        //     primary_key: "Mandant_Nr, Uid",
        //     delete: true,
        //     copy: true,
        // },
        ReplicationTable {
            name: "WP_Anlage",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "WP_Buchung",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "WP_Konfiguration",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "WP_Stand",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Wertpapier_Uid, Datum",
            delete: true,
            copy: true,
        },
        ReplicationTable {
            name: "WP_Wertpapier",
            client_number: "Mandant_Nr",
            primary_key: "Mandant_Nr, Uid",
            delete: true,
            copy: true,
        },
    ];
    v
}

struct ReplicationTable<'a> {
    pub name: &'a str,
    pub client_number: &'a str,
    #[allow(dead_code)]
    pub primary_key: &'a str,
    pub delete: bool,
    #[allow(dead_code)]
    pub copy: bool,
}
