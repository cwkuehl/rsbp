use crate::{
    apis::{enums::DialogTypeEnum, services},
    base::functions,
    config::{self},
    forms::bin,
    res,
    services::diary_service,
};
use gtk::prelude::*;
use res::messages::M;
use rsbp_rep::models::TbOrt;
use std::{cell::RefCell, rc::Rc};

use super::tb200_positions::Tb200Positions;

#[derive(Debug, Clone)]
pub struct Tb210Position {
    dialog_type: DialogTypeEnum,
    parent: Tb200Positions,
    window: gtk::Dialog,
    uid: Option<String>,
    grid: gtk::Grid,
    nr: gtk::Entry,
    bezeichnung0: gtk::Label,
    bezeichnung: gtk::Entry,
    breite0: gtk::Label,
    breite: gtk::Entry,
    laenge0: gtk::Label,
    laenge: gtk::Entry,
    hoehe: gtk::Entry,
    notiz: gtk::TextView,
    angelegt: gtk::Entry,
    geaendert: gtk::Entry,
    ok: gtk::Button,
    cancel: gtk::Button,
    model: Option<TbOrt>,
}

impl Tb210Position {
    pub fn new(
        dialog_type: DialogTypeEnum,
        parent: &Tb200Positions,
        uid: &Option<String>,
    ) -> Rc<RefCell<Self>> {
        let wref = Tb210Position::get_objects(dialog_type, parent, uid);
        Tb210Position::init_data(&mut wref.borrow_mut(), 0);
        // Events erst nach dem init_data verbinden, damit das Model gespeichert ist.
        let w = wref.borrow();
        w.ok.connect_clicked(glib::clone!(@strong w => move |_| Self::on_ok(&w)));
        w.cancel
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_cancel(&w)));
        w.bezeichnung.grab_focus();
        wref.clone()
    }

    /// Formular aus glade-Datei erstellen.
    fn get_objects(
        dialog_type: DialogTypeEnum,
        parent: &Tb200Positions,
        uid: &Option<String>,
    ) -> Rc<RefCell<Self>> {
        let glade_src = include_str!("../../res/gtkgui/tb/TB210Position.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let w = Tb210Position {
            dialog_type: dialog_type.clone(),
            parent: parent.clone(),
            window: gtk::Dialog::new(),
            uid: uid.clone(),
            grid: builder.object::<gtk::Grid>("TB210Position").unwrap(),
            nr: builder.object::<gtk::Entry>("nr").unwrap(),
            bezeichnung0: builder.object::<gtk::Label>("bezeichnung0").unwrap(),
            bezeichnung: builder.object::<gtk::Entry>("bezeichnung").unwrap(),
            breite0: builder.object::<gtk::Label>("breite0").unwrap(),
            breite: builder.object::<gtk::Entry>("breite").unwrap(),
            laenge0: builder.object::<gtk::Label>("laenge0").unwrap(),
            laenge: builder.object::<gtk::Entry>("laenge").unwrap(),
            hoehe: builder.object::<gtk::Entry>("hoehe").unwrap(),
            notiz: builder.object::<gtk::TextView>("notiz").unwrap(),
            angelegt: builder.object::<gtk::Entry>("angelegt").unwrap(),
            geaendert: builder.object::<gtk::Entry>("geaendert").unwrap(),
            ok: builder.object::<gtk::Button>("ok").unwrap(),
            cancel: builder.object::<gtk::Button>("abbrechen").unwrap(),
            model: None,
        };
        let config = config::get_config();
        w.window
            .set_title(bin::get_title(M::TB210_title, &dialog_type, config.is_de()).as_str());
        // w.window.set_modal(true);
        let content_area = w.window.content_area();
        content_area.add(&w.grid);
        bin::make_locale(
            &builder,
            &config,
            Some(&w.window),
            &std::any::type_name::<Tb210Position>().to_string(),
        );
        bin::set_bold(&w.bezeichnung0);
        bin::set_bold(&w.breite0);
        bin::set_bold(&w.laenge0);
        w.window.show_all();
        let w2 = Rc::new(RefCell::new(w));
        w2
    }

    /// Model-Daten initialisieren.
    /// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
    fn init_data(&mut self, step: i32) {
        let config = config::get_config();
        let is_de = config.is_de();
        if step == 0 {
            // Werte aus Parametern nehmen.
            let neu = self.dialog_type == DialogTypeEnum::New;
            let loeschen = self.dialog_type == DialogTypeEnum::Delete;
            if let (false, Some(uid)) = (neu, self.uid.clone()) {
                let daten = services::get_daten();
                let rm = diary_service::get_position(&daten, &uid);
                if let (true, Ok(Some(k))) = (bin::get(&rm, Some(&self.window)), rm) {
                    bin::set_text_entry(&self.nr, &Some(k.uid.clone()));
                    bin::set_text_entry(&self.bezeichnung, &Some(k.bezeichnung.clone()));
                    bin::set_text_entry(
                        &self.breite,
                        &Some(functions::f64_to_str_5(&k.breite, is_de)),
                    );
                    bin::set_text_entry(
                        &self.laenge,
                        &Some(functions::f64_to_str_5(&k.laenge, is_de)),
                    );
                    bin::set_text_entry(&self.hoehe, &functions::f64_to_ostr_2(&k.hoehe, is_de));
                    bin::set_text_textview(&self.notiz, &Some(k.notiz.clone()));
                    self.angelegt.set_text(
                        functions::format_date_of(&k.angelegt_am, &k.angelegt_von, is_de).as_str(),
                    );
                    self.geaendert.set_text(
                        functions::format_date_of(&k.geaendert_am, &k.geaendert_von, is_de)
                            .as_str(),
                    );
                    self.model = Some(k);
                }
            }
            self.nr.set_editable(false);
            self.bezeichnung.set_editable(!loeschen);
            self.breite.set_editable(!loeschen);
            self.laenge.set_editable(!loeschen);
            self.hoehe.set_editable(!loeschen);
            self.notiz.set_editable(!loeschen);
            self.angelegt.set_editable(false);
            self.geaendert.set_editable(false);
            if loeschen {
                self.ok.set_label(M::me(M::Forms_delete, is_de));
            }
        }
    }

    // TODO /// <summary>Behandlung von Breite.</summary>
    // /// <param name="sender">Betroffener Sender.</param>
    // /// <param name="e">Betroffenes Ereignis.</param>
    // protected void OnBreiteFocusOutEvent(object sender, Gtk.FocusOutEventArgs e)
    // {
    //   var c = Functions.ToCoordinates(breite.Text);
    //   if (c != null)
    //   {
    //     breite.Text = Functions.ToString(c.Item1, 5);
    //     laenge.Text = Functions.ToString(c.Item2, 5);
    //     hoehe.Text = Functions.ToString(c.Item3, 2);
    //   }
    // }

    /// Behandlung von OK.
    fn on_ok(&self) {
        let daten = services::get_daten();
        if self.dialog_type == DialogTypeEnum::New
            || self.dialog_type == DialogTypeEnum::Copy
            || self.dialog_type == DialogTypeEnum::Edit
        {
            let r = diary_service::save_position(
                &daten,
                &bin::get_text_entry(&self.nr),
                &bin::get_text_entry(&self.bezeichnung),
                &bin::get_text_entry(&self.breite),
                &bin::get_text_entry(&self.laenge),
                &bin::get_text_entry(&self.hoehe),
                &bin::get_text_textview(&self.notiz),
            );
            if bin::get(&r, Some(&self.window)) {
                self.parent.on_refresh();
                self.window.close();
            }
        } else if self.dialog_type == DialogTypeEnum::Delete {
            if let Some(model) = &self.model {
                let r = diary_service::delete_position(&daten, model);
                if bin::get(&r, Some(&self.window)) {
                    self.parent.on_refresh();
                    self.window.close();
                }
            }
        }
    }

    /// Behandlung von Abbrechen.
    fn on_cancel(&self) {
        self.window.close();
    }
}
