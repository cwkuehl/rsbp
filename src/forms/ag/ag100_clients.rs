use super::ag110_client::Ag110Client;
use crate::{
    apis::{enums::DialogTypeEnum, services},
    base::functions,
    config::{self, RsbpConfig},
    forms::{bin, main_window::MainWindow},
    res::messages::M,
    services::client_service,
};
use gtk::prelude::*;

#[derive(Debug, Clone)]
pub struct Ag100Clients {
    config: RsbpConfig,
    parent: gtk::Window,
    pub window: gtk::Grid,
    refresh: gtk::Button,
    undo: gtk::Button,
    redo: gtk::Button,
    new: gtk::Button,
    copy: gtk::Button,
    edit: gtk::Button,
    delete: gtk::Button,
    mandanten0: gtk::Label,
    mandanten: gtk::TreeView,
}

impl Ag100Clients {
    pub fn new(parent: &gtk::Window) -> Self {
        let w = Ag100Clients::get_objects(parent);
        Ag100Clients::init_data(&w, 0);
        w.mandanten.grab_focus();
        w
    }

    /// Formular aus glade-Datei erstellen.
    fn get_objects(parent: &gtk::Window) -> Self {
        let glade_src = include_str!("../../res/gtkgui/ag/AG100Clients.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let w = Ag100Clients {
            config: config::get_config(),
            parent: parent.clone(),
            window: builder.object::<gtk::Grid>("AG100Clients").unwrap(),
            refresh: builder.object::<gtk::Button>("refreshAction").unwrap(),
            undo: builder.object::<gtk::Button>("undoAction").unwrap(),
            redo: builder.object::<gtk::Button>("redoAction").unwrap(),
            new: builder.object::<gtk::Button>("newAction").unwrap(),
            copy: builder.object::<gtk::Button>("copyAction").unwrap(),
            edit: builder.object::<gtk::Button>("editAction").unwrap(),
            delete: builder.object::<gtk::Button>("deleteAction").unwrap(),
            mandanten0: builder.object::<gtk::Label>("mandanten0").unwrap(),
            mandanten: builder.object::<gtk::TreeView>("mandanten").unwrap(),
        };
        w.window.connect_destroy(|_| {
            println!("AG100 Clients destroy");
        });
        bin::make_locale(&builder, &w.config, None, &"".to_string());
        bin::set_bold(&w.mandanten0);
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
        w.mandanten
            .connect_row_activated(glib::clone!(@strong w => move |_,_,_| Self::on_mandanten(&w) ));
        w.window.show_all();
        w
    }

    /// Model-Daten initialisieren.
    /// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
    fn init_data(&self, step: i32) {
        if step <= 1 {
            let daten = services::get_daten();
            let l0 = client_service::get_client_list(&daten);
            if bin::get(&l0, Some(&self.parent)) {
                if let Ok(ref l) = l0 {
                    let mut values = Vec::<Vec<String>>::new();
                    // Nr.;Nr.;Bezeichnung;Geändert am;Geändert von;Angelegt am;Angelegt von
                    for e in l {
                        let v: Vec<String> = vec![
                            e.nr.to_string(),
                            e.nr.to_string(),
                            e.beschreibung.clone(),
                            functions::ondt_to_str(&e.geaendert_am),
                            functions::ostr_to_str(&e.geaendert_von),
                            functions::ondt_to_str(&e.angelegt_am),
                            functions::ostr_to_str(&e.angelegt_von),
                        ];
                        values.push(v);
                    }
                    let config = config::get_config();
                    let columns = M::me(M::AG100_mandanten_columns, config.is_de());
                    let r = bin::add_string_columns_sort(&self.mandanten, columns, Some(values));
                    bin::get(&r, Some(&self.parent));
                    // bin::set_text(&self.mandanten, Some("2".into()));
                }
            }
        }
    }

    /// Behandlung von Refresh.
    pub fn on_refresh(&self) {
        let f = move || self.init_data(0);
        let r = bin::refresh_treeview(&self.mandanten, f, None);
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

    /// Behandlung von Mandanten.
    fn on_mandanten(&self) {
        self.edit.activate();
    }

    /// Starten des Details-Dialogs.
    /// * dt: Betroffener Dialog-Typ.
    fn start_dialog(&self, dt: DialogTypeEnum) {
        let r = bin::get_text_tv(&self.mandanten, dt != DialogTypeEnum::New, 0);
        if bin::get(&r, Some(&self.parent)) {
            let uid = functions::to_i32(r.unwrap().unwrap_or("".into()).as_str());
            let _w = Ag110Client::new(dt, &self, uid);
        }
    }
}
