use super::{tb110_date::Tb110Date, tb210_position::Tb210Position};
use crate::{
    apis::{
        enums::{DialogTypeEnum, SearchDirectionEnum},
        services,
    },
    base::functions,
    config::{self, RsbpConfig, RsbpError},
    forms::{
        bin,
        controls::{self, DateCallback, DateEvent},
        main_window::MainWindow,
        ui_tools,
    },
    res::messages::M,
    services::diary_service,
};
use chrono::NaiveDate;
use gtk::prelude::*;
use rsbp_rep::{models::TbEintrag, models_ext::TbEintragOrtExt};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Tb100Diary {
    config: RsbpConfig,
    parent: gtk::Window,
    pub window: gtk::Grid,
    copy: gtk::Button,
    paste: gtk::Button,
    undo: gtk::Button,
    redo: gtk::Button,
    save: gtk::Button,
    before1: gtk::TextView,
    before2: gtk::TextView,
    before3: gtk::TextView,
    date0: gtk::Label,
    date: gtk::Grid,
    entry0: gtk::Label,
    clear: gtk::Button,
    search1: gtk::Entry,
    search2: gtk::Entry,
    search3: gtk::Entry,
    search4: gtk::Entry,
    search5: gtk::Entry,
    search6: gtk::Entry,
    search7: gtk::Entry,
    search8: gtk::Entry,
    search9: gtk::Entry,
    first: gtk::Button,
    back: gtk::Button,
    forward: gtk::Button,
    last: gtk::Button,
    positions: gtk::TreeView,
    new: gtk::Button,
    remove: gtk::Button,
    position: gtk::ComboBoxText,
    add: gtk::Button,
    angelegt: gtk::Entry,
    geaendert: gtk::Entry,
    posbefore: gtk::Button,
    entry: gtk::TextView,
    position2: gtk::ComboBoxText,
    from: gtk::Grid,
    to: gtk::Grid,
    after1: gtk::TextView,
    after2: gtk::TextView,
    after3: gtk::TextView,
    copy_string: String,
    loaded: bool,
    position_list: Vec<TbEintragOrtExt>,
    entry_old: TbEintrag,
    position_list_old: Vec<TbEintragOrtExt>,
}

impl DateCallback for Tb100Diary {
    fn date_callback(&mut self, event: &DateEvent) {
        match event {
            DateEvent::Date { name, date: _ } => {
                if name == "date" {
                    self.on_date_date();
                }
            }
            DateEvent::Month { name, date: _ } => {
                if name == "date" {
                    self.on_date_month();
                }
            }
            DateEvent::Unchanged => (),
        }
    }
}

impl Tb100Diary {
    pub fn new(parent: &gtk::Window) -> Rc<RefCell<Self>> {
        let wref = Tb100Diary::get_objects(parent);
        Tb100Diary::init_data(&mut wref.borrow_mut(), 0);
        // Events erst nach dem init_data verbinden, damit das Model gespeichert ist und geändert werden kann.
        let w = wref.borrow();
        w.copy.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_copy(&mut wref.borrow_mut()) ),
        );
        w.paste.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_paste(&mut wref.borrow_mut()) ),
        );
        w.undo.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_undo(&mut wref.borrow_mut()) ),
        );
        w.redo.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_redo(&mut wref.borrow_mut()) ),
        );
        w.save.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_save(&mut wref.borrow_mut()) ),
        );
        w.positions.connect_row_activated(
            glib::clone!(@strong wref => move |_,_,_| Self::on_positions_activated(&mut wref.borrow_mut()) ),
        );
        w.new.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_new(&mut wref.borrow_mut()) ),
        );
        w.add.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_add(&mut wref.borrow_mut()) ),
        );
        w.posbefore.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_posbefore(&mut wref.borrow_mut()) ),
        );
        w.remove.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_remove(&mut wref.borrow_mut()) ),
        );
        w.first.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_first(&mut wref.borrow_mut()) ),
        );
        w.back.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_back(&mut wref.borrow_mut()) ),
        );
        w.forward.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_forward(&mut wref.borrow_mut()) ),
        );
        w.last.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_last(&mut wref.borrow_mut()) ),
        );
        w.clear.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_clear(&mut wref.borrow_mut()) ),
        );
        w.entry.grab_focus();
        wref.clone()
    }

    /// Formular aus glade-Datei erstellen.
    fn get_objects(parent: &gtk::Window) -> Rc<RefCell<Self>> {
        let glade_src = include_str!("../../res/gtkgui/tb/TB100Diary.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let w = Tb100Diary {
            config: config::get_config(),
            parent: parent.clone(),
            window: builder.object::<gtk::Grid>("TB100Diary").unwrap(),
            copy: builder.object::<gtk::Button>("copyAction").unwrap(),
            paste: builder.object::<gtk::Button>("pasteAction").unwrap(),
            undo: builder.object::<gtk::Button>("undoAction").unwrap(),
            redo: builder.object::<gtk::Button>("redoAction").unwrap(),
            save: builder.object::<gtk::Button>("saveAction").unwrap(),
            before1: builder.object::<gtk::TextView>("before1").unwrap(),
            before2: builder.object::<gtk::TextView>("before2").unwrap(),
            before3: builder.object::<gtk::TextView>("before3").unwrap(),
            date0: builder.object::<gtk::Label>("date0").unwrap(),
            date: builder.object::<gtk::Grid>("date").unwrap(),
            entry0: builder.object::<gtk::Label>("entry0").unwrap(),
            clear: builder.object::<gtk::Button>("clear").unwrap(),
            search1: builder.object::<gtk::Entry>("search1").unwrap(),
            search2: builder.object::<gtk::Entry>("search2").unwrap(),
            search3: builder.object::<gtk::Entry>("search3").unwrap(),
            search4: builder.object::<gtk::Entry>("search4").unwrap(),
            search5: builder.object::<gtk::Entry>("search5").unwrap(),
            search6: builder.object::<gtk::Entry>("search6").unwrap(),
            search7: builder.object::<gtk::Entry>("search7").unwrap(),
            search8: builder.object::<gtk::Entry>("search8").unwrap(),
            search9: builder.object::<gtk::Entry>("search9").unwrap(),
            first: builder.object::<gtk::Button>("first").unwrap(),
            back: builder.object::<gtk::Button>("back").unwrap(),
            forward: builder.object::<gtk::Button>("forward").unwrap(),
            last: builder.object::<gtk::Button>("last").unwrap(),
            positions: builder.object::<gtk::TreeView>("positions").unwrap(),
            new: builder.object::<gtk::Button>("new").unwrap(),
            remove: builder.object::<gtk::Button>("remove").unwrap(),
            position: builder.object::<gtk::ComboBoxText>("position").unwrap(),
            add: builder.object::<gtk::Button>("add").unwrap(),
            angelegt: builder.object::<gtk::Entry>("angelegt").unwrap(),
            geaendert: builder.object::<gtk::Entry>("geaendert").unwrap(),
            posbefore: builder.object::<gtk::Button>("posbefore").unwrap(),
            entry: builder.object::<gtk::TextView>("entry").unwrap(),
            position2: builder.object::<gtk::ComboBoxText>("position2").unwrap(),
            from: builder.object::<gtk::Grid>("from").unwrap(),
            to: builder.object::<gtk::Grid>("to").unwrap(),
            after1: builder.object::<gtk::TextView>("after1").unwrap(),
            after2: builder.object::<gtk::TextView>("after2").unwrap(),
            after3: builder.object::<gtk::TextView>("after3").unwrap(),
            copy_string: String::new(),
            loaded: false,
            position_list: Vec::new(),
            entry_old: TbEintrag {
                mandant_nr: 0,
                datum: NaiveDate::from_yo(0, 1),
                eintrag: String::new(),
                angelegt_am: None,
                angelegt_von: None,
                geaendert_am: None,
                geaendert_von: None,
                replikation_uid: None,
            },
            position_list_old: Vec::new(),
        };
        w.window.connect_destroy(|_| {
            println!("TB100 Diary destroy");
        });
        bin::make_locale(&builder, &w.config, None, &"".to_string());
        bin::set_bold(&w.date0);
        bin::set_bold(&w.entry0);
        let w2 = Rc::new(RefCell::new(w));
        let g = controls::Date::new(&w2.borrow().date, &w2, "date", false, true, true);
        g.borrow_mut().set_accel("m", "p", Some(&w2.borrow().date0));
        let _ = controls::Date::new(&w2.borrow().from, &w2, "from", true, true, false);
        let _ = controls::Date::new(&w2.borrow().to, &w2, "to", true, true, false);
        //g.borrow().grab_focus();
        w2.borrow().window.show_all();
        w2
    }

    /// Model-Daten initialisieren.
    /// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
    fn init_data(&mut self, step: i32) {
        if step <= 0 {
            let daten = services::get_daten();
            self.fill_lists();
            self.clear_search();
            self.entry_old.datum = daten.get_today();
            // self.load_month(Some(self.entry_old.datum)); // braucht nicht wegen on_last
            bin::set_date_grid(&self.date, &Some(self.entry_old.datum), true);
            self.update_entries(false, true);
            self.on_last();
            self.before1.set_editable(false);
            self.before2.set_editable(false);
            self.before3.set_editable(false);
            self.after1.set_editable(false);
            self.after2.set_editable(false);
            self.after3.set_editable(false);
            self.angelegt.set_editable(false);
            self.geaendert.set_editable(false);
        }
    }

    /// Update parent.
    pub fn update_parent(&self) {
        self.fill_lists();
    }

    /// Behandlung von Refresh.
    pub fn on_refresh(&mut self) {
        self.fill_lists();
        self.update_entries(false, true);
    }

    /// Behandlung von Undo.
    fn on_undo(&mut self) {
        if MainWindow::undo(Some(&self.parent)) {
            self.on_refresh();
        }
    }

    /// Behandlung von Redo.
    fn on_redo(&mut self) {
        if MainWindow::redo(Some(&self.parent)) {
            self.on_refresh();
        }
    }

    /// Behandlung von Copy.
    fn on_copy(&mut self) {
        self.copy_string = bin::get_text_textview(&self.entry);
    }

    /// Behandlung von Paste.
    fn on_paste(&mut self) {
        bin::set_text_textview(&self.entry, &Some(self.copy_string.to_string()));
        self.update_entries(true, false);
    }

    /// Behandlung von Save.
    fn on_save(&mut self) {
        // TODO Generate report
        self.update_entries(true, false);
        // var pfad = Parameter.TempPath;
        // var datei = Functions.GetDateiname(M0(TB005), true, true, "txt");
        // UiTools.SaveFile(Get(FactoryService.DiaryService.GetFile(ServiceDaten, GetSearchArray())), pfad, datei, true);
    }

    /// Behandlung von Date.
    fn on_date_date(&mut self) {
        // println!("on_date_date");
        self.update_entries(true, true);
    }

    /// Behandlung von Date.
    fn on_date_month(&mut self) {
        // println!("on_date_month");
        self.on_date_date();
        let d = bin::get_date_grid(&self.date);
        self.load_month(d);
    }

    /// Handle position.
    fn on_positions_activated(&self) {
        let r = bin::get_text_tv(&self.positions, false, 0);
        if bin::get(&r, Some(&self.parent)) {
            if let Ok(Some(uid)) = r {
                if let Some(p) = self
                    .position_list
                    .iter()
                    .filter(|a| a.ort_uid == uid)
                    .collect::<Vec<&TbEintragOrtExt>>()
                    .first()
                {
                    let s = format!(
                        "https://www.openstreetmap.org/#map=19/{}/{}",
                        p.breite, p.laenge
                    );
                    bin::get(&ui_tools::start_url(s.as_str()), Some(&self.parent));
                }
            }
        }
    }

    /// Handle new.
    fn on_new(&mut self) {
        let _w = Tb210Position::new(DialogTypeEnum::New, Some(self), None, &None);
    }

    /// Handle add.
    fn on_add(&mut self) {
        if let Some(uid) = bin::get_text_cb(&self.position) {
            if let Some(p) = self
                .position_list
                .iter()
                .filter(|a| a.ort_uid == uid)
                .collect::<Vec<&TbEintragOrtExt>>()
                .first()
            {
                // Change date
                let wref = Tb110Date::new(DialogTypeEnum::Edit, p);
                // println!("TB110: {:?}", wref.borrow().result);
                let od = bin::get_date_grid(&self.date);
                if let (Some(to), Some(d)) = (wref.borrow().result, od) {
                    let mut p1 = (*p).clone();
                    if to >= d {
                        p1.datum_bis = to;
                    } else {
                        p1.datum_von = to;
                    }
                    if let Some(pos) = self.position_list.iter().position(|a| a.ort_uid == uid) {
                        self.position_list.remove(pos);
                        self.position_list.insert(pos, p1);
                    }
                    self.init_positions();
                }
                return;
            }
            let daten = services::get_daten();
            let r0 = diary_service::get_position(&daten, &uid);
            if !bin::get(&r0, Some(&self.parent)) {
                return;
            }
            let od = bin::get_date_grid(&self.date);
            if let (Ok(Some(k)), Some(d)) = (r0, od) {
                let p = TbEintragOrtExt {
                    mandant_nr: k.mandant_nr,
                    ort_uid: k.uid,
                    datum_von: d,
                    datum_bis: d,
                    angelegt_von: None,
                    angelegt_am: None,
                    geaendert_von: None,
                    geaendert_am: None,
                    bezeichnung: k.bezeichnung,
                    breite: k.breite,
                    laenge: k.laenge,
                    hoehe: k.hoehe,
                    notiz: String::new(),
                };
                self.position_list.push(p);
                self.init_positions();
            }
        }
    }

    /// Handle posbefore.
    fn on_posbefore(&mut self) {
        if let Some(d) = functions::ond_add_days(&bin::get_date_grid(&self.date), -1) {
            let daten = services::get_daten();
            let r = diary_service::get_entry_position_list(&daten, &d);
            if let (true, Ok(Some(list))) = (bin::get(&r, Some(&self.parent)), r) {
                for p in list {
                    if let None = self
                        .position_list
                        .iter()
                        .position(|a| a.ort_uid == p.ort_uid)
                    {
                        let mut pc = p.clone();
                        if pc.datum_bis == d {
                            if let Some(d1) = functions::ond_add_days(&Some(pc.datum_bis), 1) {
                                pc.datum_bis = d1;
                            }
                        }
                        self.position_list.push(pc);
                    }
                }
                self.init_positions();
            }
        }
    }

    /// Handle remove.
    fn on_remove(&mut self) {
        let r = bin::get_text_tv(&self.positions, false, 0);
        if let (true, Ok(Some(uid))) = (bin::get(&r, Some(&self.parent)), r) {
            if let Some(p) = self.position_list.iter().position(|a| a.ort_uid == uid) {
                self.position_list.remove(p);
                self.init_positions();
            }
        }
    }

    /// Handle first.
    fn on_first(&mut self) {
        self.search_entry(&SearchDirectionEnum::First);
    }

    /// Handle back.
    fn on_back(&mut self) {
        self.search_entry(&SearchDirectionEnum::Back);
    }

    /// Handle forward.
    fn on_forward(&mut self) {
        self.search_entry(&SearchDirectionEnum::Forward);
    }

    /// Handle last.
    fn on_last(&mut self) {
        self.search_entry(&SearchDirectionEnum::Last);
    }

    /// Handle clear.
    fn on_clear(&mut self) {
        self.clear_search();
        self.update_entries(true, false);
    }

    /// Fill lists.
    fn fill_lists(&self) {
        let uid = bin::get_text_cb(&self.position);
        let daten = services::get_daten();
        let rl0 = diary_service::get_position_list(&daten, &None, &None);
        if bin::get(&rl0, Some(&self.parent)) {
            if let Ok(ref rl) = rl0 {
                let mut values = Vec::<Vec<String>>::new();
                values.push(vec![String::new(), String::new()]); // empty entry
                for e in rl {
                    let v: Vec<String> = vec![e.bezeichnung.to_string(), e.uid.to_string()];
                    values.push(v);
                }
                let r = bin::add_string_columns_cb(&self.position, Some(values));
                bin::get(&r, Some(&self.parent));
                let mut values = Vec::<Vec<String>>::new();
                values.push(vec![String::new(), String::new()]); // empty entry
                let de = daten.config.is_de();
                values.push(vec![
                    M::mec(M::TB012, de).to_owned().to_string(),
                    "0".to_string(),
                ]); // without position
                for e in rl {
                    let v: Vec<String> = vec![e.bezeichnung.to_string(), e.uid.to_string()];
                    values.push(v);
                }
                let r = bin::add_string_columns_cb(&self.position2, Some(values));
                bin::get(&r, Some(&self.parent));
            }
        }
        bin::set_text_cb(&self.position, &uid);
    }

    /// Clear search strings.
    fn clear_search(&self) {
        self.search1.set_text("%%");
        self.search2.set_text("%%");
        self.search3.set_text("%%");
        self.search4.set_text("%%");
        self.search5.set_text("%%");
        self.search6.set_text("%%");
        self.search7.set_text("%%");
        self.search8.set_text("%%");
        self.search9.set_text("%%");
        bin::set_text_cb(&self.position2, &None);
        let today = services::get_daten().get_today();
        bin::set_date_grid(&self.from, &None, true);
        bin::set_date_grid(&self.to, &Some(today), true);
    }

    /// Search next entry
    fn search_entry(&mut self, dir: &SearchDirectionEnum) {
        self.update_entries(true, false);
        let daten = services::get_daten();
        let date = bin::get_date_grid(&self.date);
        let search = [
            bin::get_text_entry(&self.search1),
            bin::get_text_entry(&self.search2),
            bin::get_text_entry(&self.search3),
            bin::get_text_entry(&self.search4),
            bin::get_text_entry(&self.search5),
            bin::get_text_entry(&self.search6),
            bin::get_text_entry(&self.search7),
            bin::get_text_entry(&self.search8),
            bin::get_text_entry(&self.search9),
        ];
        let puid = bin::get_text_cb(&self.position2);
        let from = bin::get_date_grid(&self.from);
        let to = bin::get_date_grid(&self.to);
        let d0 = diary_service::search_date(&daten, dir, &date, &search, &puid, &from, &to);
        if bin::get(&d0, Some(&self.parent)) {
            if let Ok(Some(d)) = d0 {
                self.load_month(Some(d));
                bin::set_date_grid(&self.date, &Some(d), true);
                self.update_entries(false, true);
                bin::update_date_grid_month(&self.date);
            }
        }
    }

    /// Load entries.
    fn load_entries(&mut self, date: Option<NaiveDate>) -> crate::Result<()> {
        if let Some(d) = date {
            let daten = services::get_daten();
            let de = daten.config.is_de();
            let mut errors = Vec::<String>::new();
            let mut entry = String::new();
            self.position_list.clear();
            self.position_list_old.clear();
            if let Some(d1) = functions::nd_add_dmy(&d, -1, 0, 0) {
                if let Some(tb) =
                    crate::config::get(&mut errors, &diary_service::get_entry(&daten, &d1))
                {
                    entry = tb.eintrag.to_string();
                }
            }
            bin::set_text_textview(&self.before1, &Some(entry));
            entry = String::new();
            if let Some(d1) = functions::nd_add_dmy(&d, 0, -1, 0) {
                if let Some(tb) =
                    crate::config::get(&mut errors, &diary_service::get_entry(&daten, &d1))
                {
                    entry = tb.eintrag.to_string();
                }
            }
            bin::set_text_textview(&self.before2, &Some(entry));
            entry = String::new();
            if let Some(d1) = functions::nd_add_dmy(&d, 0, 0, -1) {
                if let Some(tb) =
                    crate::config::get(&mut errors, &diary_service::get_entry(&daten, &d1))
                {
                    entry = tb.eintrag.to_string();
                }
            }
            bin::set_text_textview(&self.before3, &Some(entry));
            // Current entry
            if let Some(tb) = crate::config::get(&mut errors, &diary_service::get_entry(&daten, &d))
            {
                self.entry_old.eintrag = tb.eintrag.to_string();
                self.angelegt.set_text(
                    functions::format_date_of(&tb.angelegt_am, &tb.angelegt_von, de).as_str(),
                );
                self.geaendert.set_text(
                    functions::format_date_of(&tb.geaendert_am, &tb.geaendert_von, de).as_str(),
                );
            } else {
                self.entry_old.eintrag = String::new();
                self.angelegt.set_text("");
                self.geaendert.set_text("");
            }
            if let Some(plist) = crate::config::get(
                &mut errors,
                &diary_service::get_entry_position_list(&daten, &d),
            ) {
                for p in plist {
                    self.position_list.push(p.clone());
                    self.position_list_old.push(p.clone());
                }
            }
            self.entry_old.datum = d;
            bin::set_text_textview(&self.entry, &Some(self.entry_old.eintrag.to_string()));
            self.init_positions();
            entry = String::new();
            if let Some(d1) = functions::nd_add_dmy(&d, 1, 0, 0) {
                if let Some(tb) =
                    crate::config::get(&mut errors, &diary_service::get_entry(&daten, &d1))
                {
                    entry = tb.eintrag.to_string();
                }
            }
            bin::set_text_textview(&self.after1, &Some(entry));
            entry = String::new();
            if let Some(d1) = functions::nd_add_dmy(&d, 0, 1, 0) {
                if let Some(tb) =
                    crate::config::get(&mut errors, &diary_service::get_entry(&daten, &d1))
                {
                    entry = tb.eintrag.to_string();
                }
            }
            bin::set_text_textview(&self.after2, &Some(entry));
            entry = String::new();
            if let Some(d1) = functions::nd_add_dmy(&d, 0, 0, 1) {
                if let Some(tb) =
                    crate::config::get(&mut errors, &diary_service::get_entry(&daten, &d1))
                {
                    entry = tb.eintrag.to_string();
                }
            }
            bin::set_text_textview(&self.after3, &Some(entry));
            if !errors.is_empty() {
                return Err(RsbpError::error(&errors));
            }
        }
        Ok(())
    }

    /// Init positions.
    fn init_positions(&mut self) {
        let daten = services::get_daten();
        let de = daten.config.is_de();
        let mut values = Vec::<Vec<String>>::new();
        // Nr.;Bezeichnung;Breite;Länge;Von;Bis;Geändert am;Geändert von;Angelegt am;Angelegt von
        for e in self.position_list.iter() {
            let v: Vec<String> = vec![
                e.ort_uid.to_string(),
                e.bezeichnung.clone(),
                functions::f64_to_str_5(&e.breite, de),
                functions::f64_to_str_5(&e.laenge, de),
                functions::ond_to_str(&Some(e.datum_von)),
                functions::ond_to_str(&Some(e.datum_bis)),
                functions::ondt_to_str(&e.geaendert_am),
                functions::ostr_to_str(&e.geaendert_von),
                functions::ondt_to_str(&e.angelegt_am),
                functions::ostr_to_str(&e.angelegt_von),
            ];
            values.push(v);
        }
        let columns = M::me(M::TB100_positions_columns, de);
        let r = bin::add_string_columns_sort(&self.positions, columns, Some(values));
        bin::get(&r, Some(&self.parent));
    }

    /// Load month.
    fn load_month(&mut self, date: Option<NaiveDate>) {
        let mut m = Vec::<bool>::new();
        if let Some(d) = date {
            let daten = services::get_daten();
            let r = diary_service::get_month(&daten, &d);
            if bin::get(&r, Some(&self.parent)) {
                for b in r.ok().unwrap() {
                    m.push(b);
                }
            }
        }
        // Recursion vermeiden!
        bin::set_month_grid(&self.date, &Some(m), false);
    }

    /// Edit entries.
    fn update_entries(&mut self, save: bool, load: bool) {
        // Rekursion vermeiden
        if save && self.loaded {
            // Alten Eintrag von vorher merken.
            let old = self.entry_old.eintrag.to_string();
            self.position_list.sort_by(|a, b| a.ort_uid.cmp(&b.ort_uid));
            let p = self
                .position_list
                .iter()
                .map(|a| format!("{} {} {}", a.ort_uid, a.datum_von, a.datum_bis))
                .collect::<Vec<String>>()
                .join(",");
            self.position_list_old
                .sort_by(|a, b| a.ort_uid.cmp(&b.ort_uid));
            let p0 = self
                .position_list_old
                .iter()
                .map(|a| format!("{} {} {}", a.ort_uid, a.datum_von, a.datum_bis))
                .collect::<Vec<String>>()
                .join(",");
            // Nur speichern, wenn etwas geändert ist.
            let new = bin::get_text_textview(&self.entry);
            if old == "" || old != new || p != p0 {
                let daten = services::get_daten();
                let r = diary_service::save_entry(
                    &daten,
                    &self.entry_old.datum,
                    &new,
                    &self.position_list,
                );
                bin::get(&r, Some(&self.parent));
            }
        }
        if load {
            let d = bin::get_date_grid(&self.date);
            let r = self.load_entries(d);
            // self.load_month(d); // braucht nicht wegen on_date_month
            self.loaded = true;
            bin::get(&r, Some(&self.parent));
        }
    }
}
