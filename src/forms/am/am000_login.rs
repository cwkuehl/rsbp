use crate::{
    apis::services::{self, ServiceDaten},
    base::{functions, parameter},
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
pub struct Am000Login {
    config: RsbpConfig,
    window: gtk::Dialog,
    grid: gtk::Grid,
    client0: gtk::Label,
    client: gtk::Entry,
    user0: gtk::Label,
    user: gtk::Entry,
    password0: gtk::Label,
    password: gtk::Entry,
    save: gtk::CheckButton,
    login: gtk::Button,
    reset: gtk::Button,
    cancel: gtk::Button,
    pub result: bool,
}

impl Form for Am000Login {}

impl Am000Login {
    pub fn new(application: &gtk::Application) -> Self {
        let config = config::get_config();
        let mut w = Am000Login::get_objects(config);
        application.add_window(&w.window);
        Am000Login::init_data(&w, 0);
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
        let glade_src = include_str!("../../res/gtkgui/am/AM000Login.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let w = Am000Login {
            config,
            window: gtk::Dialog::new(),
            grid: builder.object::<gtk::Grid>("AM000Login").unwrap(),
            client0: builder.object::<gtk::Label>("client0").unwrap(),
            client: builder.object::<gtk::Entry>("client").unwrap(),
            user0: builder.object::<gtk::Label>("user0").unwrap(),
            user: builder.object::<gtk::Entry>("user").unwrap(),
            password0: builder.object::<gtk::Label>("password0").unwrap(),
            password: builder.object::<gtk::Entry>("password").unwrap(),
            save: builder.object::<gtk::CheckButton>("save").unwrap(),
            login: builder.object::<gtk::Button>("login").unwrap(),
            reset: builder.object::<gtk::Button>("reset").unwrap(),
            cancel: builder.object::<gtk::Button>("cancel").unwrap(),
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
            &std::any::type_name::<Am000Login>().to_string(),
        );
        bin::set_bold(&w.client0);
        bin::set_bold(&w.user0);
        bin::set_bold(&w.password0);
        w.login
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_login(&w)));
        w.reset
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_reset(&w)));
        w.cancel
            .connect_clicked(glib::clone!(@strong w => move |_| Self::on_cancel(&w)));
        w.window.show_all();
        w
    }

    /// Model-Daten initialisieren.
    /// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
    fn init_data(&self, step: i32) {
        if step == 0 {
            // Werte aus Parametern nehmen.
            self.client
                .set_text(functions::to_str(parameter::get_login_client()).as_ref());
            self.user.set_text(parameter::get_login_user().as_str());
            if self.client.text_length() <= 0 {
                self.client.grab_focus();
            } else if self.user.text_length() <= 0 {
                self.user.grab_focus();
            } else {
                self.password.grab_focus();
            }
        }
    }

    /// Behandlung von Anmelden.
    fn on_login(&self) {
        let daten = ServiceDaten::new0(
            &self.config,
            functions::to_i32(self.client.buffer().text().as_str()),
            self.user.buffer().text().as_str(),
        );
        let r = login_service::login(
            &daten,
            self.password.buffer().text().as_str(),
            self.save.is_active(),
        );
        bin::get(&r, Some(&self.window));
        if let Ok(benutzer_id) = r {
            let daten1 = ServiceDaten::new0(&self.config, daten.mandant_nr, benutzer_id.as_str());
            services::set_daten(&daten1);
            // Parameter setzen.
            parameter::set_login_client(daten1.mandant_nr);
            parameter::set_login_user(daten1.benutzer_id.as_str());
            bin::get(&parameter::save(), Some(&self.window));
            unsafe {
                self.window.set_data("result", true);
            }
            self.window.hide();
        }
    }

    /// Behandlung von Zurücksetzen.
    fn on_reset(&self) {
        parameter::reset_dialog_sizes();
    }

    /// Behandlung von Abbrechen.
    fn on_cancel(&self) {
        self.window.hide();
    }
}
