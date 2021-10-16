use crate::{
    apis::{enums::DialogTypeEnum, services},
    base::{functions, parameter},
    config::{self},
    forms::{
        bin,
        controls::{self, DateCallback, DateEvent},
    },
    res,
    services::address_service,
};
use gtk::prelude::*;
use res::messages::M;
use rsbp_rep::models::MaMandant;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Ad120Birthdays {
    window: gtk::Dialog,
    grid: gtk::Grid,
    datum0: gtk::Label,
    datum: gtk::Grid,
    tage0: gtk::Label,
    tage: gtk::Entry,
    geburtstage0: gtk::Label,
    geburtstagesw: gtk::ScrolledWindow,
    geburtstage: gtk::TextView,
    id6: gtk::Box,
    starten: gtk::CheckButton,
    ok: gtk::Button,
    model: Option<MaMandant>,
}

impl DateCallback for Ad120Birthdays {
    fn date_callback(&mut self, event: &DateEvent) {
        match event {
            DateEvent::Date { name: _, date: _ } => {
                Self::init_data(self, 1);
            }
            DateEvent::Month { name: _, date: _ } => {
                Self::init_data(self, 1);
            }
            DateEvent::Unchanged => (),
        }
    }
}

impl<'a> Ad120Birthdays {
    /// Erstellen des nicht-modalen Dialogs.
    /// * dialog_type: Betroffener Dialog-Typ.
    /// * parent: Betroffener Eltern-Dialog.
    /// * uid: Betroffene ID.
    pub fn new() -> Rc<RefCell<Self>> {
        let wref = Ad120Birthdays::get_objects();
        Ad120Birthdays::init_data(&mut wref.borrow_mut(), 0);
        // Events erst nach dem init_data verbinden, damit das Model gespeichert ist.
        let w = wref.borrow();
        w.ok.connect_clicked(glib::clone!(@strong w => move |_| Self::on_ok(&w)));
        w.ok.grab_focus();
        wref.clone()
    }

    /// Formular aus glade-Datei erstellen.
    fn get_objects() -> Rc<RefCell<Self>> {
        let glade_src = include_str!("../../res/gtkgui/ad/AD120Birthdays.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let config = config::get_config();
        let w = Ad120Birthdays {
            window: gtk::Dialog::new(),
            grid: builder.object::<gtk::Grid>("AD120Birthdays").unwrap(),
            datum0: builder.object::<gtk::Label>("datum0").unwrap(),
            datum: builder.object::<gtk::Grid>("datum").unwrap(),
            tage0: builder.object::<gtk::Label>("tage0").unwrap(),
            tage: builder.object::<gtk::Entry>("tage").unwrap(),
            geburtstage0: builder.object::<gtk::Label>("geburtstage0").unwrap(),
            geburtstagesw: builder
                .object::<gtk::ScrolledWindow>("geburtstagesw")
                .unwrap(),
            geburtstage: builder.object::<gtk::TextView>("geburtstage").unwrap(),
            id6: builder.object::<gtk::Box>("id6").unwrap(),
            starten: builder.object::<gtk::CheckButton>("starten").unwrap(),
            ok: builder.object::<gtk::Button>("ok").unwrap(),
            model: None,
        };
        let de = config.is_de();
        w.window
            .set_title(bin::get_title(M::AD120_title, &DialogTypeEnum::Without, de).as_str());
        w.window.set_modal(false);
        let content_area = w.window.content_area();
        content_area.add(&w.grid);
        bin::make_locale(
            &builder,
            &config,
            Some(&w.window),
            &std::any::type_name::<Ad120Birthdays>().to_string(),
        );
        bin::set_bold(&w.datum0);
        bin::set_bold(&w.tage0);
        w.window.show_all();
        let w2 = Rc::new(RefCell::new(w));
        let g = controls::Date::new(&w2.borrow().datum, &w2, "datum", false, true, false);
        g.borrow_mut()
            .set_accel("m", "p", Some(&w2.borrow().datum0));
        w2
    }

    /// Model-Daten initialisieren.
    /// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
    fn init_data(&mut self, step: i32) {
        let daten = services::get_daten();
        if step <= 0 {
            bin::set_date_grid(&self.datum, &Some(daten.get_today()), true);
            self.tage
                .set_text(functions::to_str(parameter::get_ad120_days()).as_str());
            self.starten.set_active(parameter::get_ad120_start());
        }
        if step <= 1 {
            let mut s = String::new();
            let date = bin::get_date_grid(&self.datum).unwrap_or(daten.get_today());
            let days = functions::to_i32(self.tage.text().as_str());
            let l0 = address_service::get_birthday_list(&daten, &date, days);
            if bin::get2(&l0) {
                if let Ok(ref l) = l0 {
                    if l.len() <= 1 {
                        self.window.close();
                        return;
                    }
                    s.push_str(l.join("\n").as_str());
                }
            }
            if let Some(buffer) = self.geburtstage.buffer() {
                buffer.set_text(s.as_str());
            }
        }
    }

    /// Behandlung von OK.
    fn on_ok(&self) {
        parameter::set_ad120_days(self.tage.text().to_string().as_str());
        parameter::set_ad120_start(self.starten.is_active());
        self.window.close();
    }
}
