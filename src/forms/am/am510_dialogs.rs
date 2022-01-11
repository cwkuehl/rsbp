use crate::{
    apis::{enums::DialogTypeEnum, services},
    base::parameter,
    config::{self},
    forms::bin,
    res,
};
use gtk::{prelude::*, SelectionMode};
use lazy_static::lazy_static;
use res::messages::M;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct Am510Dialogs {
    window: gtk::Dialog,
    grid: gtk::Grid,
    dialoge: gtk::TreeView,
    zuordnen: gtk::Button,
    entfernen: gtk::Button,
    zudialoge: gtk::TreeView,
    oben: gtk::Button,
    unten: gtk::Button,
    ok: gtk::Button,
    abbrechen: gtk::Button,
    model: Vec<String>,
}

impl<'a> Am510Dialogs {
    /// Erstellen des nicht-modalen Dialogs.
    /// * dialog_type: Betroffener Dialog-Typ.
    /// * parent: Betroffener Eltern-Dialog.
    /// * uid: Betroffene ID.
    pub fn new() -> Rc<RefCell<Self>> {
        let wref = Am510Dialogs::get_objects();
        Am510Dialogs::init_data(&mut wref.borrow_mut(), 0);
        // Events erst nach dem init_data verbinden, damit das Model gespeichert ist.
        let w = wref.borrow();
        w.dialoge.connect_row_activated(
            glib::clone!(@strong wref => move |_,_,_| Self::on_zuordnen(&mut wref.borrow_mut())),
        );
        w.zuordnen.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_zuordnen(&mut wref.borrow_mut())),
        );
        w.entfernen.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_entfernen(&mut wref.borrow_mut())),
        );
        w.zudialoge.connect_row_activated(
            glib::clone!(@strong wref => move |_,_,_| Self::on_entfernen(&mut wref.borrow_mut())),
        );
        w.oben.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_oben(&mut wref.borrow_mut())),
        );
        w.unten.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_unten(&mut wref.borrow_mut())),
        );
        w.ok.connect_clicked(glib::clone!(@strong wref => move |_| Self::on_ok(&wref.borrow())));
        w.abbrechen.connect_clicked(
            glib::clone!(@strong wref => move |_| Self::on_abbrechen(&wref.borrow())),
        );
        w.dialoge.grab_focus();
        wref.clone()
    }

    /// Formular aus glade-Datei erstellen.
    fn get_objects() -> Rc<RefCell<Self>> {
        let glade_src = include_str!("../../res/gtkgui/am/AM510Dialogs.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let config = config::get_config();
        let w = Am510Dialogs {
            window: gtk::Dialog::new(),
            grid: builder.object::<gtk::Grid>("AM510Dialogs").unwrap(),
            dialoge: builder.object::<gtk::TreeView>("dialoge").unwrap(),
            zuordnen: builder.object::<gtk::Button>("zuordnen").unwrap(),
            entfernen: builder.object::<gtk::Button>("entfernen").unwrap(),
            zudialoge: builder.object::<gtk::TreeView>("zudialoge").unwrap(),
            oben: builder.object::<gtk::Button>("oben").unwrap(),
            unten: builder.object::<gtk::Button>("unten").unwrap(),
            ok: builder.object::<gtk::Button>("ok").unwrap(),
            abbrechen: builder.object::<gtk::Button>("abbrechen").unwrap(),
            model: vec![],
        };
        let de = config.is_de();
        w.window
            .set_title(bin::get_title(M::AM510_title, &DialogTypeEnum::Without, de).as_str());
        w.window.set_modal(false);
        let content_area = w.window.content_area();
        content_area.add(&w.grid);
        bin::make_locale(
            &builder,
            &config,
            Some(&w.window),
            &std::any::type_name::<Am510Dialogs>().to_string(),
        );
        // bin::set_bold(&w.datum0);
        w.window.show_all();
        let w2 = Rc::new(RefCell::new(w));
        w2
    }

    /// Model-Daten initialisieren.
    /// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
    fn init_data(&mut self, step: i32) {
        let _daten = services::get_daten();
        if step <= 0 {
            self.dialoge.selection().set_mode(SelectionMode::Multiple);
            self.zudialoge.selection().set_mode(SelectionMode::Multiple);
            let sd = parameter::get_start_dialogs();
            let arr = sd.split("|");
            self.model.clear();
            for d in arr {
                self.model.push(d.to_string());
            }
            self.fill_lists();
        }
    }

    /// Fill lists.
    fn fill_lists(&self) {
        let _sel = bin::get_selected_tv(&self.dialoge, 0);
        let _sel2 = bin::get_selected_tv(&self.zudialoge, 0);
        let guard = match START_DIALOGS.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let daten = services::get_daten();
        let de = daten.config.is_de();
        let mut values = Vec::<Vec<String>>::new();
        // Nr.;Kürzel;Titel
        for e in (*guard).values() {
            let key = format!("#{}", e.key);
            // println!("Keys {} {}", key, e.key);
            if !self.model.contains(&key) {
                let v: Vec<String> = vec![key, e.key.to_string(), M::mss(e.title.as_str(), de)];
                values.push(v);
            }
        }
        let columns = M::me(M::AM510_dialoge_columns, de);
        let r = bin::add_string_columns_sort(&self.dialoge, columns, Some(values));
        bin::get2(&r);
        let mut zuvalues = Vec::<Vec<String>>::new();
        // Nr.;Kürzel;Titel
        for e in self.model.iter() {
            if let Some(sd) = (*guard).get(e.as_str()) {
                let v: Vec<String> = vec![
                    e.to_string(),
                    sd.key.to_string(),
                    M::mss(sd.title.as_str(), de),
                ];
                zuvalues.push(v);
            }
        }
        let columns = M::me(M::AM510_zudialoge_columns, de);
        let r = bin::add_string_columns_sort(&self.zudialoge, columns, Some(zuvalues));
        bin::get2(&r);
    }

    /// Behandlung von Zuordnen.
    fn on_zuordnen(&mut self) {
        let tv = &self.dialoge;
        let sel = bin::get_selected_tv(tv, 0);
        for s in sel {
            self.model.push(s);
        }
        tv.selection().unselect_all();
        self.fill_lists();
    }

    /// Behandlung von Entfernen.
    fn on_entfernen(&mut self) {
        let tv = &self.zudialoge;
        let sel = bin::get_selected_tv(tv, 0);
        for s in sel {
            self.model.retain(|x| *x != s);
        }
        tv.selection().unselect_all();
        self.fill_lists();
    }

    /// Behandlung von Oben.
    fn on_oben(&mut self) {
        let tv = &self.zudialoge;
        let column = 0;
        let s = tv.selection().selected_rows();
        if s.0.len() < 1 {
            return;
        }
        for sel in s.0 {
            if let Some(iter) = s.1.iter(&sel) {
                let val = s.1.value(&iter, column);
                if let Some(v2) = val.get::<String>().ok() {
                    if let Some(p) = self.model.iter().position(|r| *r == v2) {
                        if p > 0 {
                            self.model.remove(p);
                            self.model.insert(p - 1, v2);
                        }
                    }
                }
            }
        }
        self.fill_lists();
    }

    /// Behandlung von Unten.
    fn on_unten(&mut self) {
        let tv = &self.zudialoge;
        let column = 0;
        let s = tv.selection().selected_rows();
        if s.0.len() < 1 {
            return;
        }
        for sel in s.0 {
            if let Some(iter) = s.1.iter(&sel) {
                let val = s.1.value(&iter, column);
                if let Some(v2) = val.get::<String>().ok() {
                    if let Some(p) = self.model.iter().position(|r| *r == v2) {
                        if p < self.model.len() {
                            self.model.remove(p);
                            self.model.insert(p + 1, v2);
                        }
                    }
                }
            }
        }
        self.fill_lists();
    }

    /// Behandlung von OK.
    fn on_ok(&self) {
        let sd = self.model.join("|");
        // println!("sd {}", sd);
        parameter::set_start_dialogs(&sd);
        let r = parameter::save();
        bin::get(&r, Some(&self.window));
        if let Ok(_) = r {
            self.window.close();
        }
    }

    /// Behandlung von Abbrechen.
    fn on_abbrechen(&self) {
        self.window.close();
    }
}

struct StartDialog {
    key: String,
    title: String,
}

lazy_static! {
    /// Hash of dialog titles.
    static ref START_DIALOGS: Arc<RwLock<HashMap<&'static str, StartDialog>>> = {
        let mut map = HashMap::new();
        map.insert(
            "#AG100",
            StartDialog {
                key: "AG100".to_string(),
                title: M::AG100_title.to_string(),
            },
        );
        map.insert(
            "#AG200",
            StartDialog {
                key: "AG200".to_string(),
                title: M::AG200_title.to_string(),
            },
        );
        map.insert(
            "#AG400",
            StartDialog {
                key: "AG400".to_string(),
                title: M::AG400_title.to_string(),
            },
        );
        map.insert(
            "#AM500",
            StartDialog {
                key: "AM500".to_string(),
                title: M::AM500_title.to_string(),
            },
        );
        map.insert(
            "#AM510",
            StartDialog {
                key: "AM510".to_string(),
                title: M::AM510_title.to_string(),
            },
        );
        map.insert(
            "#FZ100",
            StartDialog {
                key: "FZ100".to_string(),
                title: M::FZ100_title.to_string(),
            },
        );
        map.insert(
            "#FZ200",
            StartDialog {
                key: "FZ200".to_string(),
                title: M::FZ200_title.to_string(),
            },
        );
        map.insert(
            "#FZ250",
            StartDialog {
                key: "FZ250".to_string(),
                title: M::FZ250_title.to_string(),
            },
        );
        map.insert(
            "#FZ300",
            StartDialog {
                key: "FZ300".to_string(),
                title: M::FZ300_title.to_string(),
            },
        );
        map.insert(
            "#FZ320",
            StartDialog {
                key: "FZ320".to_string(),
                title: M::FZ320_title.to_string(),
            },
        );
        map.insert(
            "#FZ340",
            StartDialog {
                key: "FZ340".to_string(),
                title: M::FZ340_title.to_string(),
            },
        );
        map.insert(
            "#FZ700",
            StartDialog {
                key: "FZ700".to_string(),
                title: M::FZ700_title.to_string(),
            },
        );
        map.insert(
            "#HH100",
            StartDialog {
                key: "HH100".to_string(),
                title: M::HH100_title.to_string(),
            },
        );
        map.insert(
            "#HH200",
            StartDialog {
                key: "HH200".to_string(),
                title: M::HH200_title.to_string(),
            },
        );
        map.insert(
            "#HH300",
            StartDialog {
                key: "HH300".to_string(),
                title: M::HH300_title.to_string(),
            },
        );
        map.insert(
            "#HH400",
            StartDialog {
                key: "HH400".to_string(),
                title: M::HH400_title.to_string(),
            },
        );
        map.insert(
            "#HH500;EB",
            StartDialog {
                key: "HH500;EB".to_string(),
                title: M::HH500_title_EB.to_string(),
            },
        );
        map.insert(
            "#HH500;GV",
            StartDialog {
                key: "HH500;GV".to_string(),
                title: M::HH500_title_GV.to_string(),
            },
        );
        map.insert(
            "#HH500;SB",
            StartDialog {
                key: "HH500;SB".to_string(),
                title: M::HH500_title_SB.to_string(),
            },
        );
        map.insert(
            "#SB200",
            StartDialog {
                key: "SB200".to_string(),
                title: M::SB200_title.to_string(),
            },
        );
        map.insert(
            "#SB300",
            StartDialog {
                key: "SB300".to_string(),
                title: M::SB300_title.to_string(),
            },
        );
        map.insert(
            "#SB400",
            StartDialog {
                key: "SB400".to_string(),
                title: M::SB400_title.to_string(),
            },
        );
        map.insert(
            "#TB100",
            StartDialog {
                key: "TB100".to_string(),
                title: M::TB100_title.to_string(),
            },
        );
        map.insert(
            "#WP200",
            StartDialog {
                key: "WP200".to_string(),
                title: M::WP200_title.to_string(),
            },
        );
        map.insert(
            "#WP250",
            StartDialog {
                key: "WP250".to_string(),
                title: M::WP250_title.to_string(),
            },
        );
        map.insert(
            "#WP300",
            StartDialog {
                key: "WP300".to_string(),
                title: M::WP300_title.to_string(),
            },
        );
        map.insert(
            "#WP400",
            StartDialog {
                key: "WP400".to_string(),
                title: M::WP400_title.to_string(),
            },
        );
        map.insert(
            "#WP500",
            StartDialog {
                key: "WP500".to_string(),
                title: M::WP500_title.to_string(),
            },
        );
        let m = Arc::new(RwLock::new(map));
        m
    };
    /// Sammlung von allen Parametern in der Setting-Datei.
    static ref PARAMS2: Arc<RwLock<HashMap<String, Option<String>>>> = {
      let map = HashMap::new();
      let m = Arc::new(RwLock::new(map));
      m
    };
}
