use crate::{
    apis::services::{self},
    base::functions,
    config::{self, RsbpConfig},
    forms::bin,
    res,
    services::login_service,
};
use bin::Form;
use glib::prelude::*;
use gtk::prelude::*;
use res::messages::M;

#[derive(Debug, Clone)]
pub struct Am100Change {
    config: RsbpConfig,
    window: gtk::Dialog,
    grid: gtk::Grid,
    client: gtk::Entry,
    user: gtk::Entry,
    passwordold0: gtk::Label,
    passwordold: gtk::Entry,
    passwordnew0: gtk::Label,
    passwordnew: gtk::Entry,
    passwordnew20: gtk::Label,
    passwordnew2: gtk::Entry,
    save: gtk::CheckButton,
    ok: gtk::Button,
    cancel: gtk::Button,
    pub result: bool,
}

impl Form for Am100Change {}

impl Am100Change {
    pub fn new(application: &gtk::Application) -> Self {
        let config = config::get_config();
        let mut w = Am100Change::get_objects(config);
        application.add_window(&w.window);
        Am100Change::init_data(&w, 0);
        if w.window.is_modal() {
            w.window.run();
            unsafe {
                w.result = w.window.data::<bool>("result").is_some();
            }
        }
        w
    }

    /// Formular aus glade-Datei erstellen.
    fn get_objects(config: RsbpConfig) -> Self {
        let glade_src = include_str!("../../res/gtkgui/am/AM100Change.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let w = Am100Change {
            config,
            window: gtk::Dialog::new(),
            grid: builder.object::<gtk::Grid>("AM100Change").unwrap(),
            client: builder.object::<gtk::Entry>("mandant").unwrap(),
            user: builder.object::<gtk::Entry>("benutzer").unwrap(),
            passwordold0: builder.object::<gtk::Label>("kennwortAlt0").unwrap(),
            passwordold: builder.object::<gtk::Entry>("kennwortAlt").unwrap(),
            passwordnew0: builder.object::<gtk::Label>("kennwortNeu0").unwrap(),
            passwordnew: builder.object::<gtk::Entry>("kennwortNeu").unwrap(),
            passwordnew20: builder.object::<gtk::Label>("kennwortNeu20").unwrap(),
            passwordnew2: builder.object::<gtk::Entry>("kennwortNeu2").unwrap(),
            save: builder.object::<gtk::CheckButton>("speichern").unwrap(),
            ok: builder.object::<gtk::Button>("ok").unwrap(),
            cancel: builder.object::<gtk::Button>("abbrechen").unwrap(),
            result: false,
        };
        w.window.set_title(M::me(M::AM000_title, w.config.is_de()));
        w.window.set_modal(true);
        let content_area = w.window.content_area();
        content_area.add(&w.grid);
        bin::make_locale(
            &builder,
            &w.config,
            Some(&w.window),
            &std::any::type_name::<Am100Change>().to_string(),
        );
        bin::set_bold(&w.passwordold0);
        bin::set_bold(&w.passwordnew0);
        bin::set_bold(&w.passwordnew20);
        w.passwordnew
            .connect_key_release_event(glib::clone!(@strong w => move |_,_| {
                w.on_password()
            }));
        w.passwordnew2
            .connect_key_release_event(glib::clone!(@strong w => move |_,_| {
                w.on_password()
            }));
        w.ok.connect_clicked(glib::clone!(@strong w => move |_| Self::on_ok(&w)));
        w.cancel
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_cancel(&w)));
        w.window.show_all();
        w
    }

    /// Model-Daten initialisieren.
    /// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
    fn init_data(&self, step: i32) {
        if step == 0 {
            let daten = services::get_daten();
            bin::set_text_entry(&self.client, &Some(functions::to_str(daten.mandant_nr)));
            bin::set_text_entry(&self.user, &Some(daten.benutzer_id));
            self.ok.set_sensitive(false);
            self.passwordold.grab_focus();
        }
    }

    /// Handle Password.
    fn on_password(&self) -> gtk::Inhibit {
        let new = bin::get_text_entry(&self.passwordnew);
        let new2 = bin::get_text_entry(&self.passwordnew2);
        self.ok.set_sensitive(new == new2);
        gtk::Inhibit(false)
    }

    /// Handle OK.
    fn on_ok(&self) {
        let daten = services::get_daten();
        let r = login_service::change_password(
            &daten,
            daten.mandant_nr,
            daten.benutzer_id.as_str(),
            &bin::get_text_entry(&self.passwordold),
            &bin::get_text_entry(&self.passwordnew),
            self.save.is_active(),
        );
        if bin::get(&r, Some(&self.window)) {
            self.window.hide();
        }
    }

    /// Handle Cancel.
    fn on_cancel(&self) {
        self.window.hide();
    }
}
