use super::ag100_clients::Ag100Clients;
use crate::{
    apis::{enums::DialogTypeEnum, services},
    base::functions,
    config::{self},
    forms::bin,
    res,
    services::client_service,
};
use gtk::prelude::*;
use res::messages::M;
use rsbp_rep::models::MaMandant;

#[derive(Debug, Clone)]
pub struct Ag110Client {
    dialog_type: DialogTypeEnum,
    parent: Ag100Clients,
    uid: i32,
    window: gtk::Dialog,
    grid: gtk::Grid,
    nr: gtk::Entry,
    beschreibung0: gtk::Label,
    beschreibung: gtk::Entry,
    angelegt: gtk::Entry,
    geaendert: gtk::Entry,
    ok: gtk::Button,
    abbrechen: gtk::Button,
    model: Option<MaMandant>,
}

impl<'a> Ag110Client {
    /// Erstellen des nicht-modalen Dialogs.
    /// * dialog_type: Betroffener Dialog-Typ.
    /// * parent: Betroffener Eltern-Dialog.
    /// * uid: Betroffene ID.
    pub fn new(dialog_type: DialogTypeEnum, parent: &Ag100Clients, uid: i32) -> Self {
        let mut w = Ag110Client::get_objects(dialog_type, parent, uid);
        Ag110Client::init_data(&mut w, 0);
        // Events erst nach dem init_data verbinden, damit das Model gespeichert ist.
        w.ok.connect_clicked(glib::clone!(@strong w => move |_| Self::on_ok(&w)));
        w.abbrechen
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_abbrechen(&w)));
        w.beschreibung.grab_focus();
        w
    }

    /// Formular aus glade-Datei erstellen.
    fn get_objects(dialog_type: DialogTypeEnum, parent: &Ag100Clients, uid: i32) -> Self {
        let glade_src = include_str!("../../res/gtkgui/ag/AG110Client.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let config = config::get_config();
        let w = Ag110Client {
            dialog_type: dialog_type.clone(),
            parent: parent.clone(),
            uid,
            window: gtk::Dialog::new(),
            grid: builder.object::<gtk::Grid>("AG110Client").unwrap(),
            nr: builder.object::<gtk::Entry>("nr").unwrap(),
            beschreibung0: builder.object::<gtk::Label>("beschreibung0").unwrap(),
            beschreibung: builder.object::<gtk::Entry>("beschreibung").unwrap(),
            angelegt: builder.object::<gtk::Entry>("angelegt").unwrap(),
            geaendert: builder.object::<gtk::Entry>("geaendert").unwrap(),
            ok: builder.object::<gtk::Button>("ok").unwrap(),
            abbrechen: builder.object::<gtk::Button>("abbrechen").unwrap(),
            model: None,
        };
        let de = config.is_de();
        w.window
            .set_title(bin::get_title(M::AG110_title, &dialog_type, de).as_str());
        w.window.set_modal(false);
        let content_area = w.window.content_area();
        content_area.add(&w.grid);
        bin::make_locale(
            &builder,
            &config,
            Some(&w.window),
            &std::any::type_name::<Ag110Client>().to_string(),
        );
        bin::set_bold(&w.beschreibung0);
        w.window.show_all();
        w
    }

    /// Model-Daten initialisieren.
    /// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
    fn init_data(&mut self, step: i32) {
        let config = config::get_config();
        if step <= 0 {
            let neu = self.dialog_type == DialogTypeEnum::New;
            let loeschen = self.dialog_type == DialogTypeEnum::Delete;
            if !neu && self.uid > 0 {
                let daten = services::get_daten();
                let rm = client_service::get_client(&daten, self.uid);
                if bin::get(&rm, Some(&self.window)) {
                    if let Ok(Some(k)) = rm {
                        self.nr.set_text(functions::to_str(k.nr).as_str());
                        self.beschreibung.set_text(k.beschreibung.as_str());
                        self.angelegt.set_text(
                            functions::format_date_of(
                                &k.angelegt_am,
                                &k.angelegt_von,
                                config.is_de(),
                            )
                            .as_str(),
                        );
                        self.geaendert.set_text(
                            functions::format_date_of(
                                &k.geaendert_am,
                                &k.geaendert_von,
                                config.is_de(),
                            )
                            .as_str(),
                        );
                        self.model = Some(k);
                    }
                }
            }
            self.nr.set_editable(false);
            self.beschreibung.set_editable(!loeschen);
            self.angelegt.set_editable(false);
            self.geaendert.set_editable(false);
            if loeschen {
                self.ok.set_label(M::me(M::Forms_delete, config.is_de()));
            }
        }
    }

    /// Behandlung von OK.
    fn on_ok(&self) {
        let daten = services::get_daten();
        if self.dialog_type == DialogTypeEnum::New
            || self.dialog_type == DialogTypeEnum::Copy
            || self.dialog_type == DialogTypeEnum::Edit
        {
            let mut nr = 0;
            if self.dialog_type == DialogTypeEnum::Edit {
                nr = functions::to_i32(self.nr.text().as_str());
            }
            let r = client_service::save_client(&daten, nr, self.beschreibung.text().as_str());
            if bin::get(&r, Some(&self.window)) {
                self.parent.on_refresh();
                self.window.close();
            }
        } else if self.dialog_type == DialogTypeEnum::Delete {
            if let Some(model) = &self.model {
                let r = client_service::delete_client(&daten, model);
                if bin::get(&r, Some(&self.window)) {
                    self.parent.on_refresh();
                    self.window.close();
                }
            }
        }
    }

    /// Behandlung von Abbrechen.
    fn on_abbrechen(&self) {
        self.window.close();
    }
}
