use super::tb210_position::Tb210Position;
use crate::{
    apis::{enums::DialogTypeEnum, services},
    base::functions,
    config::{self, RsbpConfig},
    forms::{bin, main_window::MainWindow},
    res::messages::M,
    services::diary_service,
};
use gtk::prelude::*;

#[derive(Debug, Clone)]
pub struct Tb200Positions {
    config: RsbpConfig,
    parent: gtk::Window,
    pub window: gtk::Grid,
    //grid: gtk::Grid,
    refresh: gtk::Button,
    undo: gtk::Button,
    redo: gtk::Button,
    new: gtk::Button,
    copy: gtk::Button,
    edit: gtk::Button,
    delete: gtk::Button,
    positions0: gtk::Label,
    positions: gtk::TreeView,
    all: gtk::Button,
    search: gtk::Entry,
}

impl Tb200Positions {
    pub fn new(parent: &gtk::Window) -> Self {
        let w = Tb200Positions::get_objects(parent);
        Tb200Positions::init_data(&w, 0);
        w.positions.grab_focus();
        w
    }

    /// Formular aus glade-Datei erstellen.
    fn get_objects(parent: &gtk::Window) -> Self {
        let glade_src = include_str!("../../res/gtkgui/tb/TB200Positions.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let w = Tb200Positions {
            config: config::get_config(),
            parent: parent.clone(),
            window: builder.object::<gtk::Grid>("TB200Positions").unwrap(),
            //grid: builder.object::<gtk::Grid>("TB200Positions").unwrap(),
            refresh: builder.object::<gtk::Button>("refreshAction").unwrap(),
            undo: builder.object::<gtk::Button>("undoAction").unwrap(),
            redo: builder.object::<gtk::Button>("redoAction").unwrap(),
            new: builder.object::<gtk::Button>("newAction").unwrap(),
            copy: builder.object::<gtk::Button>("copyAction").unwrap(),
            edit: builder.object::<gtk::Button>("editAction").unwrap(),
            delete: builder.object::<gtk::Button>("deleteAction").unwrap(),
            positions0: builder.object::<gtk::Label>("positions0").unwrap(),
            positions: builder.object::<gtk::TreeView>("positions").unwrap(),
            all: builder.object::<gtk::Button>("all").unwrap(),
            search: builder.object::<gtk::Entry>("search").unwrap(),
        };
        w.window.connect_destroy(|_| {
            println!("TB200 Positions destroy");
        });
        bin::make_locale(&builder, &w.config, None, &"".to_string());
        bin::set_bold(&w.positions0);
        w.refresh
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_refresh(&w) ));
        w.undo
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_undo(&w) ));
        w.redo
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_redo(&w) ));
        w.new
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_new(&w) ));
        w.copy
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_copy(&w) ));
        w.edit
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_edit(&w) ));
        w.delete
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_delete(&w) ));
        w.positions
            .connect_row_activated(glib::clone!(@strong w => move |_,_,_| Self::on_position(&w) ));
        w.all
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_all(&w) ));
        w.window.show_all();
        w
    }

    /// Model-Daten initialisieren.
    /// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
    fn init_data(&self, step: i32) {
        if step <= 0 {
            bin::set_text_entry(&self.search, &Some("%%".to_string()));
        }
        if step <= 1 {
            let daten = services::get_daten();
            let l0 = diary_service::get_position_list(
                &daten,
                &None,
                &Some(bin::get_text_entry(&self.search)),
            );
            if bin::get(&l0, Some(&self.parent)) {
                if let Ok(ref l) = l0 {
                    let de = daten.config.is_de();
                    let mut values = Vec::<Vec<String>>::new();
                    // Nr.;Bezeichnung;Breite;Länge;Geändert am;Geändert von;Angelegt am;Angelegt von
                    for e in l {
                        let v: Vec<String> = vec![
                            e.uid.clone(),
                            e.bezeichnung.clone(),
                            functions::f64_to_str_5(&e.breite, de),
                            functions::f64_to_str_5(&e.laenge, de),
                            functions::ondt_to_str(&e.geaendert_am),
                            functions::ostr_to_str(&e.geaendert_von),
                            functions::ondt_to_str(&e.angelegt_am),
                            functions::ostr_to_str(&e.angelegt_von),
                        ];
                        values.push(v);
                    }
                    let columns = M::me(M::TB200_positions_columns, de);
                    let r = bin::add_string_columns_sort(&self.positions, columns, Some(values));
                    bin::get(&r, Some(&self.parent));
                }
            }
        }
    }

    /// Update parent.
    pub fn update_parent(&self) {
        self.on_refresh();
    }

    /// Behandlung von Refresh.
    fn on_refresh(&self) {
        let f = move || self.init_data(1);
        let r = bin::refresh_treeview(&self.positions, f, None);
        bin::get(&r, Some(&self.parent));
    }

    /// Behandlung von Undo.
    fn on_undo(&self) {
        if MainWindow::undo(Some(&self.parent)) {
            self.on_refresh();
        }
    }

    /// Behandlung von Redo.
    fn on_redo(&self) {
        if MainWindow::redo(Some(&self.parent)) {
            self.on_refresh();
        }
    }

    /// Behandlung von New.
    fn on_new(&self) {
        self.start_dialog(DialogTypeEnum::New);
    }

    /// Behandlung von Copy.
    fn on_copy(&self) {
        self.start_dialog(DialogTypeEnum::Copy);
    }

    /// Behandlung von Edit.
    fn on_edit(&self) {
        self.start_dialog(DialogTypeEnum::Edit);
    }

    /// Behandlung von Delete.
    fn on_delete(&self) {
        self.start_dialog(DialogTypeEnum::Delete);
    }

    /// Behandlung von Benutzer.
    fn on_position(&self) {
        self.edit.activate();
    }

    /// Behandlung von All.
    pub fn on_all(&self) {
        let f = move || self.init_data(0);
        let r = bin::refresh_treeview(&self.positions, f, None);
        bin::get(&r, Some(&self.parent));
    }

    /// Starten des Details-Dialogs.
    /// * dt: Betroffener Dialog-Typ.
    fn start_dialog(&self, dt: DialogTypeEnum) {
        let r = bin::get_text_tv(&self.positions, dt != DialogTypeEnum::New, 0);
        if bin::get(&r, Some(&self.parent)) {
            let uid = r.unwrap();
            let _w = Tb210Position::new(dt, None, Some(self), &uid);
        }
    }
}
