use std::{cell::RefCell, rc::Rc};

use super::ag200_users::Ag200Users;
use crate::{
    apis::{enums::DialogTypeEnum, services},
    base::functions,
    config::{self},
    forms::{
        bin,
        controls::{self, DateCallback, DateEvent},
    },
    res,
    services::client_service,
};
use gtk::prelude::*;
use res::messages::M;
use rsbp_rep::models::Benutzer;

#[derive(Debug, Clone)]
pub struct Ag210User {
    dialog_type: DialogTypeEnum,
    parent: Ag200Users,
    window: gtk::Dialog,
    uid: i32,
    grid: gtk::Grid,
    nr: gtk::Entry,
    benutzer_id0: gtk::Label,
    benutzer_id: gtk::Entry,
    kennwort0: gtk::Label,
    kennwort: gtk::Entry,
    berechtigung0: gtk::Label,
    berechtigung1: gtk::RadioButton,
    berechtigung2: gtk::RadioButton,
    berechtigung3: gtk::RadioButton,
    geburt0: gtk::Label,
    geburt: gtk::Grid,
    angelegt: gtk::Entry,
    geaendert: gtk::Entry,
    ok: gtk::Button,
    abbrechen: gtk::Button,
    model: Option<Benutzer>,
}

impl DateCallback for Ag210User {
    fn date_callback(&mut self, event: &DateEvent) {
        match event {
            DateEvent::Date { name, date: _ } => {
                // println!(
                //     "date_callback date {} {}",
                //     functions::ond_to_str(date),
                //     name,
                // );
                if name == "geburt" {
                    Self::on_geburt_changed(self);
                }
            }
            DateEvent::Month { name, date: _ } => {
                if name == "geburt" {
                    Self::on_geburt_changed(self);
                }
            }
            DateEvent::Unchanged => (),
        }
    }
}

impl<'a> Ag210User {
    /// Erstellen des nicht-modalen Dialogs.
    /// * dialog_type: Betroffener Dialog-Typ.
    /// * parent: Betroffener Eltern-Dialog.
    /// * uid: Betroffene ID.
    pub fn new(dialog_type: DialogTypeEnum, parent: &Ag200Users, uid: i32) -> Rc<RefCell<Self>> {
        let wref = Ag210User::get_objects(dialog_type, parent, uid);
        Ag210User::init_data(&mut wref.borrow_mut(), 0);
        // Events erst nach dem init_data verbinden, damit das Model gespeichert ist.
        let w = wref.borrow();
        w.ok.connect_clicked(glib::clone!(@strong w => move |_| Self::on_ok(&w)));
        w.abbrechen
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_abbrechen(&w)));
        w.benutzer_id.grab_focus();
        wref.clone()
    }

    /// Formular aus glade-Datei erstellen.
    fn get_objects(
        dialog_type: DialogTypeEnum,
        parent: &Ag200Users,
        uid: i32,
    ) -> Rc<RefCell<Self>> {
        let glade_src = include_str!("../../res/gtkgui/ag/AG210User.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let config = config::get_config();
        let w = Ag210User {
            dialog_type: dialog_type.clone(),
            parent: parent.clone(),
            window: gtk::Dialog::new(),
            uid,
            grid: builder.object::<gtk::Grid>("AG210User").unwrap(),
            nr: builder.object::<gtk::Entry>("nr").unwrap(),
            benutzer_id0: builder.object::<gtk::Label>("benutzerId0").unwrap(),
            benutzer_id: builder.object::<gtk::Entry>("benutzerId").unwrap(),
            kennwort0: builder.object::<gtk::Label>("kennwort0").unwrap(),
            kennwort: builder.object::<gtk::Entry>("kennwort").unwrap(),
            berechtigung0: builder.object::<gtk::Label>("berechtigung0").unwrap(),
            berechtigung1: builder.object::<gtk::RadioButton>("berechtigung1").unwrap(),
            berechtigung2: builder.object::<gtk::RadioButton>("berechtigung2").unwrap(),
            berechtigung3: builder.object::<gtk::RadioButton>("berechtigung3").unwrap(),
            geburt0: builder.object::<gtk::Label>("geburt0").unwrap(),
            geburt: builder.object::<gtk::Grid>("geburt").unwrap(),
            angelegt: builder.object::<gtk::Entry>("angelegt").unwrap(),
            geaendert: builder.object::<gtk::Entry>("geaendert").unwrap(),
            ok: builder.object::<gtk::Button>("ok").unwrap(),
            abbrechen: builder.object::<gtk::Button>("abbrechen").unwrap(),
            model: None,
        };
        let de = config.is_de();
        w.window
            .set_title(bin::get_title(M::AG210_title, &dialog_type, de).as_str());
        w.window.set_modal(false);
        let content_area = w.window.content_area();
        content_area.add(&w.grid);
        bin::make_locale(
            &builder,
            &config,
            Some(&w.window),
            &std::any::type_name::<Ag210User>().to_string(),
        );
        bin::set_bold(&w.benutzer_id0);
        bin::set_bold(&w.kennwort0);
        bin::set_bold(&w.berechtigung0);
        bin::set_bold(&w.geburt0);
        w.window.show_all();
        let w2 = Rc::new(RefCell::new(w));
        let g = controls::Date::new(&w2.borrow().geburt, &w2, "geburt", false, true, true);
        g.borrow_mut()
            .set_accel("m", "p", Some(&w2.borrow().geburt0));
        // g.borrow().grab_focus();
        w2
    }

    /// Behandlung von Geburt.
    fn on_geburt_changed(&self) {
        // println!("on_geburt_changed {:?}", bin::get_date_grid(&self.geburt));
    }

    /// Model-Daten initialisieren.
    /// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
    fn init_data(&mut self, step: i32) {
        let config = config::get_config();
        if step <= 0 {
            bin::init_data_rb(vec![
                (&self.berechtigung1, "0"),
                (&self.berechtigung2, "1"),
                (&self.berechtigung3, "2"),
            ]);
            let neu = self.dialog_type == DialogTypeEnum::New;
            let loeschen = self.dialog_type == DialogTypeEnum::Delete;
            if !neu && self.uid > 0 {
                let daten = services::get_daten();
                let rm = client_service::get_user(&daten, self.uid);
                if bin::get(&rm, Some(&self.window)) {
                    if let Ok(Some(k)) = rm {
                        self.nr.set_text(functions::to_str(k.person_nr).as_str());
                        self.benutzer_id.set_text(k.benutzer_id.as_str());
                        let _r = bin::set_text_rb(&self.berechtigung1, &k.berechtigung.to_string());
                        self.kennwort
                            .set_text(k.passwort.clone().unwrap_or("".into()).as_str());
                        bin::set_date_grid(&self.geburt, &k.geburt, true);
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
            self.benutzer_id.set_editable(!loeschen);
            self.kennwort.set_editable(!loeschen);
            for a in self.berechtigung1.group().iter() {
                a.set_sensitive(!loeschen)
            }
            self.geburt.set_sensitive(!loeschen);
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
            let r = client_service::save_user(
                &daten,
                nr,
                &self.benutzer_id.text().to_string(),
                &Some(self.kennwort.text().to_string()),
                &functions::to_i32(bin::get_text_rb(&self.berechtigung1).as_str()),
                &bin::get_date_grid(&self.geburt),
            );
            if bin::get(&r, Some(&self.window)) {
                self.parent.on_refresh();
                self.window.close();
            }
        } else if self.dialog_type == DialogTypeEnum::Delete {
            if let Some(model) = &self.model {
                let r = client_service::delete_user(&daten, model);
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
