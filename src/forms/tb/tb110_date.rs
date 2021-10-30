use crate::{
    apis::enums::DialogTypeEnum,
    config::{self},
    forms::{bin, controls},
    res,
};
use chrono::NaiveDate;
use glib::prelude::*;
use gtk::prelude::*;
use res::messages::M;
use rsbp_rep::models_ext::TbEintragOrtExt;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Tb110Date {
    dialog_type: DialogTypeEnum,
    window: gtk::Dialog,
    grid: gtk::Grid,
    number: gtk::Entry,
    description: gtk::Entry,
    date0: gtk::Label,
    date: gtk::Grid,
    calendar: gtk::Calendar,
    ok: gtk::Button,
    cancel: gtk::Button,
    param: TbEintragOrtExt,
    pub result: Option<NaiveDate>,
}

impl Tb110Date {
    pub fn new(dialog_type: DialogTypeEnum, p: &TbEintragOrtExt) -> Rc<RefCell<Self>> {
        let wref = Tb110Date::get_objects(dialog_type, p);
        Tb110Date::init_data(&mut wref.borrow_mut(), 0);
        // w.ok.connect_clicked(
        //     glib::clone!(@strong wref => move |_| Self::on_ok(&mut wref.borrow_mut()) ),
        // );
        // w.cancel.connect_clicked(
        //     glib::clone!(@strong wref => move |_| Self::on_cancel(&mut wref.borrow()) ),
        // );
        let mut w = wref.borrow_mut();
        if w.window.is_modal() {
            w.window.run();
            unsafe {
                if w.window.data::<bool>("result").is_some() {
                    let date = w.calendar.date();
                    // println!("{:?}", date);
                    w.result = Some(NaiveDate::from_ymd(date.0 as i32, date.1 + 1, date.2));
                }
            }
        }
        wref.clone()
    }

    /// Formular aus glade-Datei erstellen.
    fn get_objects(dialog_type: DialogTypeEnum, p: &TbEintragOrtExt) -> Rc<RefCell<Self>> {
        let glade_src = include_str!("../../res/gtkgui/tb/TB110Date.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let date0 = builder.object::<gtk::Label>("datum0").unwrap();
        let date = builder.object::<gtk::Grid>("datum").unwrap();
        let g = controls::Date::new2(&date, "date", false, true, true);
        g.borrow_mut().set_accel("m", "p", Some(&date0));
        let w = Tb110Date {
            dialog_type,
            window: gtk::Dialog::new(),
            grid: builder.object::<gtk::Grid>("TB110Date").unwrap(),
            number: builder.object::<gtk::Entry>("nr").unwrap(),
            description: builder.object::<gtk::Entry>("bezeichnung").unwrap(),
            date0: builder.object::<gtk::Label>("datum0").unwrap(),
            date: builder.object::<gtk::Grid>("datum").unwrap(),
            calendar: g.borrow().calendar.clone(),
            ok: builder.object::<gtk::Button>("ok").unwrap(),
            cancel: builder.object::<gtk::Button>("abbrechen").unwrap(),
            param: p.clone(),
            result: None,
        };
        let config = config::get_config();
        w.window.set_title(M::me(M::TB110_title, config.is_de()));
        w.window.set_modal(true);
        let content_area = w.window.content_area();
        content_area.add(&w.grid);
        bin::make_locale(
            &builder,
            &config,
            Some(&w.window),
            &std::any::type_name::<Tb110Date>().to_string(),
        );
        bin::set_bold(&w.date0);
        w.ok.connect_clicked(glib::clone!(@strong w => move |_| Self::on_ok(&w)));
        w.cancel
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_cancel(&w)));
        w.window.show_all();
        let w2 = Rc::new(RefCell::new(w));
        w2
    }

    /// Model-Daten initialisieren.
    /// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
    fn init_data(&self, step: i32) {
        if step == 0 {
            // Werte aus Parametern nehmen.
            let loeschen = self.dialog_type == DialogTypeEnum::Delete;
            bin::set_text_entry(&self.number, &Some(self.param.ort_uid.to_string()));
            bin::set_text_entry(&self.description, &Some(self.param.bezeichnung.to_string()));
            bin::set_date_grid(&self.date, &Some(self.param.datum_bis), true);
            self.number.set_editable(false);
            self.description.set_editable(false);
            if loeschen {
                let config = config::get_config();
                self.ok.set_label(M::me(M::Forms_delete, config.is_de()));
            }
        }
    }

    /// Behandlung von OK.
    fn on_ok(&self) {
        unsafe {
            self.window.set_data("result", true);
        }
        self.window.hide();
    }

    /// Behandlung von Abbrechen.
    fn on_cancel(&self) {
        self.window.hide();
    }
}
