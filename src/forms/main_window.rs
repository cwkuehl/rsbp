use super::{am::am100_change::Am100Change, bin};
use crate::{
    apis::services::{self, ServiceDaten},
    base::{
        functions,
        parameter::{self, Parameter},
    },
    config::{self, RsbpConfig, RsbpError},
    forms::{
        ad::ad120_birthdays::Ad120Birthdays,
        ag::{ag100_clients::Ag100Clients, ag200_users::Ag200Users},
        am::{am000_login::Am000Login, am510_dialogs::Am510Dialogs},
        tb::{tb100_diary::Tb100Diary, tb200_positions::Tb200Positions},
    },
    res::{self, messages::Messages},
    services::login_service,
};
use gtk::{prelude::*, traits::SettingsExt, IconSize, Orientation, ReliefStyle, Window};
use lazy_static::lazy_static;
use res::messages::M;
use std::sync::Arc;

pub struct MainWindow {
    config: RsbpConfig,
    window: gtk::Window,
    notebook: gtk::Notebook,
    menu_undo: gtk::MenuItem,
    menu_redo: gtk::MenuItem,
    menu_clients: gtk::MenuItem,
    menu_users: gtk::MenuItem,
    menu_backups: gtk::MenuItem,
    menu_quit: gtk::MenuItem,
    menu_login: gtk::MenuItem,
    menu_logout: gtk::MenuItem,
    menu_pwchange: gtk::MenuItem,
    menu_options: gtk::MenuItem,
    menu_dialogs: gtk::MenuItem,
    menu_reset: gtk::MenuItem,
    menu_diary: gtk::MenuItem,
    menu_positions: gtk::MenuItem,
    menu_notes: gtk::MenuItem,
    menu_persons: gtk::MenuItem,
    menu_mileages: gtk::MenuItem,
    menu_bikes: gtk::MenuItem,
    menu_books: gtk::MenuItem,
    menu_authors: gtk::MenuItem,
    menu_series: gtk::MenuItem,
    menu_statistics: gtk::MenuItem,
    menu_sudoku: gtk::MenuItem,
    menu_detective: gtk::MenuItem,
    menu_bookings: gtk::MenuItem,
    menu_events: gtk::MenuItem,
    menu_accounts: gtk::MenuItem,
    menu_periods: gtk::MenuItem,
    menu_finalbalance: gtk::MenuItem,
    menu_plbalance: gtk::MenuItem,
    menu_openingbalance: gtk::MenuItem,
    menu_ancestors: gtk::MenuItem,
    menu_families: gtk::MenuItem,
    menu_sources: gtk::MenuItem,
    menu_stocks: gtk::MenuItem,
    menu_configurations: gtk::MenuItem,
    menu_chart: gtk::MenuItem,
    menu_investments: gtk::MenuItem,
    menu_bookings3: gtk::MenuItem,
    menu_prices: gtk::MenuItem,
    menu_about: gtk::MenuItem,
    menu_help2: gtk::MenuItem,
}

lazy_static! {
    pub static ref TITLE_HEIGHT: i32 = {
        let mut th = 0;
        if std::env::consts::OS == "linux" {
            th = 37;
        }
        th
    };
}

pub fn build_ui(application: &gtk::Application) {
    gtk::Settings::default()
        .unwrap()
        .set_gtk_application_prefer_dark_theme(true);
    let mw = MainWindow::get_objects();
    mw.window.set_application(Some(application));
    let logobuf = res::load_logo().unwrap_or(None);
    mw.window.set_icon(logobuf.as_ref());
    mw.window.set_title(res::APP_NAME);
    let size = parameter::get_dialog_size(std::any::type_name::<MainWindow>());
    // mw.window.set_size_request(size.width, size.height); // Das Fenster kann damit nicht mehr kleiner werden.
    mw.window.set_default_size(size.width, size.height);
    if size.x == -1 && size.y == -1 {
        mw.window.set_position(gtk::WindowPosition::Center);
    } else {
        mw.window.move_(size.x, size.y);
    }
    mw.window.connect_configure_event(|_, e| {
        // println!("connect_configure_event {:?} {:?}", e.position(), e.size());
        let p = e.position();
        let s = e.size();
        let r = parameter::Rectangle {
            x: p.0,
            y: p.1 - *TITLE_HEIGHT,
            width: s.0 as i32,
            height: s.1 as i32,
        };
        parameter::set_dialog_size(std::any::type_name::<MainWindow>(), &r);
        false
    });
    mw.window.connect_delete_event(|w, _| {
        MainWindow::quit("Shutdown", 0, Some(w));
        Inhibit(false)
    });
    mw.window.connect_key_press_event(move |w, key| {
        match key.keyval() {
            gdk::keys::constants::Escape => MainWindow::quit("Escape", 1, Some(w)),
            _ => (),
        }
        Inhibit(false)
    });
    let mw2 = Arc::new(mw);
    MainWindow::set_menu_events(&mw2, application);
    mw2.window.show_all();
    mw2.set_permission(false);
    mw2.start();
}

impl MainWindow {
    /// Erstellt MainWindow aus glade-Datei und benennt die Menüpunkte sprachabhängig.
    /// * config: Konfiguration mit Sprache.
    fn get_objects() -> Self {
        let config = config::get_config();
        let glade_src = include_str!("../res/gtkgui/MainWindow.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let mw = MainWindow {
            config,
            window: builder.object::<gtk::Window>("MainWindow").unwrap(),
            notebook: builder.object::<gtk::Notebook>("Notebook").unwrap(),
            menu_undo: builder.object::<gtk::MenuItem>("MenuUndo").unwrap(),
            menu_redo: builder.object::<gtk::MenuItem>("MenuRedo").unwrap(),
            menu_clients: builder.object::<gtk::MenuItem>("MenuClients").unwrap(),
            menu_users: builder.object::<gtk::MenuItem>("MenuUsers").unwrap(),
            menu_backups: builder.object::<gtk::MenuItem>("MenuBackups").unwrap(),
            menu_quit: builder.object::<gtk::MenuItem>("MenuQuit").unwrap(),
            menu_login: builder.object::<gtk::MenuItem>("MenuLogin").unwrap(),
            menu_logout: builder.object::<gtk::MenuItem>("MenuLogout").unwrap(),
            menu_pwchange: builder.object::<gtk::MenuItem>("MenuPwchange").unwrap(),
            menu_options: builder.object::<gtk::MenuItem>("MenuOptions").unwrap(),
            menu_dialogs: builder.object::<gtk::MenuItem>("MenuDialogs").unwrap(),
            menu_reset: builder.object::<gtk::MenuItem>("MenuReset").unwrap(),
            menu_diary: builder.object::<gtk::MenuItem>("MenuDiary").unwrap(),
            menu_positions: builder.object::<gtk::MenuItem>("MenuPositions").unwrap(),
            menu_notes: builder.object::<gtk::MenuItem>("MenuNotes").unwrap(),
            menu_persons: builder.object::<gtk::MenuItem>("MenuPersons").unwrap(),
            menu_mileages: builder.object::<gtk::MenuItem>("MenuMileages").unwrap(),
            menu_bikes: builder.object::<gtk::MenuItem>("MenuBikes").unwrap(),
            menu_books: builder.object::<gtk::MenuItem>("MenuBooks").unwrap(),
            menu_authors: builder.object::<gtk::MenuItem>("MenuAuthors").unwrap(),
            menu_series: builder.object::<gtk::MenuItem>("MenuSeries").unwrap(),
            menu_statistics: builder.object::<gtk::MenuItem>("MenuStatistics").unwrap(),
            menu_sudoku: builder.object::<gtk::MenuItem>("MenuSudoku").unwrap(),
            menu_detective: builder.object::<gtk::MenuItem>("MenuDetective").unwrap(),
            menu_bookings: builder.object::<gtk::MenuItem>("MenuBookings").unwrap(),
            menu_events: builder.object::<gtk::MenuItem>("MenuEvents").unwrap(),
            menu_accounts: builder.object::<gtk::MenuItem>("MenuAccounts").unwrap(),
            menu_periods: builder.object::<gtk::MenuItem>("MenuPeriods").unwrap(),
            menu_finalbalance: builder.object::<gtk::MenuItem>("MenuFinalbalance").unwrap(),
            menu_plbalance: builder.object::<gtk::MenuItem>("MenuPlbalance").unwrap(),
            menu_openingbalance: builder
                .object::<gtk::MenuItem>("MenuOpeningbalance")
                .unwrap(),
            menu_ancestors: builder.object::<gtk::MenuItem>("MenuAncestors").unwrap(),
            menu_families: builder.object::<gtk::MenuItem>("MenuFamilies").unwrap(),
            menu_sources: builder.object::<gtk::MenuItem>("MenuSources").unwrap(),
            menu_stocks: builder.object::<gtk::MenuItem>("MenuStocks").unwrap(),
            menu_configurations: builder
                .object::<gtk::MenuItem>("MenuConfigurations")
                .unwrap(),
            menu_chart: builder.object::<gtk::MenuItem>("MenuChart").unwrap(),
            menu_investments: builder.object::<gtk::MenuItem>("MenuInvestments").unwrap(),
            menu_bookings3: builder.object::<gtk::MenuItem>("MenuBookings3").unwrap(),
            menu_prices: builder.object::<gtk::MenuItem>("MenuPrices").unwrap(),
            menu_about: builder.object::<gtk::MenuItem>("MenuAbout").unwrap(),
            menu_help2: builder.object::<gtk::MenuItem>("MenuHelp2").unwrap(),
        };
        // Menüpunkte benennen.
        let cl = builder.objects();
        for c in cl.iter() {
            if c.type_().name() == "GtkMenuItem" {
                // println!("{:?}", c);
                if let Some(p) = c.property("label").ok() {
                    if let Some(gstr) = p.get::<glib::GString>().ok() {
                        let s = gstr.as_str();
                        if s.contains(".") {
                            // println!("{:?} {:?}", p, s);
                            let s1 = M::ms(s, mw.config.is_de());
                            // let s2 = s.to_owned() + "<-";
                            let v = glib::Value::from(s1.into_owned().as_str());
                            c.set_property("label", &v).ok();
                        }
                    }
                }
            }
        }
        if false && mw.config.is_de() && cfg!(debug_assertions) {
            println!("Deutsch");
        }
        mw
    }

    fn set_menu_events(mw: &Arc<MainWindow>, application: &gtk::Application) {
        mw.menu_undo
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::undo(Some(&mw.window));
            }));
        mw.menu_redo
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::redo(Some(&mw.window));
            }));
        {
            let gui = Arc::clone(mw);
            mw.menu_clients
                .connect_activate(glib::clone!(@weak application => move |_| {
                    let w = Ag100Clients::new(&gui.window);
                    gui.append_page(w.window.upcast(), M::AG100_title);
                }));
        }
        {
            let gui = Arc::clone(mw);
            mw.menu_users
                .connect_activate(glib::clone!(@weak application => move |_| {
                    let w = Ag200Users::new(&gui.window);
                    gui.append_page(w.window.upcast(), M::AG200_title);
                }));
        }
        mw.menu_backups
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        {
            let gui = Arc::clone(mw);
            mw.menu_quit
                .connect_activate(glib::clone!(@weak application => move |_| {
                    MainWindow::quit("Quit", 0, Some(&gui.window));
                }));
        }
        {
            let gui = Arc::clone(mw);
            mw.menu_login
                .connect_activate(glib::clone!(@weak application => move |_| {
                    let w = Am000Login::new(&application);
                    if w.result {
                        gui.login();
                    }
                    // Once the new window has been created, we put it into our hashmap so we can update its title when needed.
                    // windows.borrow_mut().insert(0, window.downgrade());
                }));
        }
        mw.menu_logout
            .connect_activate(glib::clone!(@strong mw => move |_| {
                mw.logout();
            }));
        mw.menu_pwchange
            .connect_activate(glib::clone!(@weak application => move |_| {
                let _ = Am100Change::new(&application);
            }));
        mw.menu_options
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_dialogs
            .connect_activate(glib::clone!(@strong mw => move |_| {
                let _ = Am510Dialogs::new();
            }));
        mw.menu_reset
            .connect_activate(glib::clone!(@strong mw => move |_| {
                parameter::reset_dialog_sizes();
                mw.window.hide();
                mw.window.set_position(gtk::WindowPosition::Center);
                mw.window.set_size_request(400, 300);
                mw.window.unmaximize(); // Bringt nichts, wenn Window maximal ist!
                mw.window.show_all();
            }));
        {
            let gui = Arc::clone(mw);
            mw.menu_diary
                .connect_activate(glib::clone!(@weak application => move |_| {
                    let w0 = Tb100Diary::new(&gui.window);
                    let w = w0.borrow();
                    gui.append_page(w.window.clone().upcast(), M::TB100_title);
                }));
        }
        {
            let gui = Arc::clone(mw);
            mw.menu_positions
                .connect_activate(glib::clone!(@weak application => move |_| {
                    let w = Tb200Positions::new(&gui.window);
                    gui.append_page(w.window.upcast(), M::TB200_title);
                }));
        }
        mw.menu_notes
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_persons
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_mileages
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_bikes
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_books
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_authors
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_series
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_statistics
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_sudoku
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_detective
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_bookings
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_events
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_accounts
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_periods
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_finalbalance
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_plbalance
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_openingbalance
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_ancestors
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_families
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_sources
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_stocks
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_configurations
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_chart
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_investments
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_bookings3
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_prices
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
        mw.menu_about.connect_activate(|_| {
            let logobuf = res::load_logo().unwrap_or(None);
            let about = MainWindow::new_about_dialog(None::<&gtk::Window>, logobuf);
            //about.grab_focus();
            about.run();
        });
        mw.menu_help2
            .connect_activate(glib::clone!(@strong mw => move |_| {
                MainWindow::not_implemented(Some(&mw.window));
            }));
    }

    /// Aktualisieren des Anwendungstitels.
    fn refresh_title(&self) {
        let test = match parameter::get_test() {
            true => "Test-",
            _ => "",
        };
        let t = parameter::get_title();
        let title = format!("{}RSBP {} W. Kuehl", test, t);
        self.window.set_title(title.as_str());
    }

    /// Setzen der Berechtigung für die Anwendung.
    /// * b: Ist die Anmeldung erfolgt?
    fn set_permission(&self, b: bool) {
        self.refresh_title();

        self.menu_undo.set_visible(true);
        self.menu_redo.set_visible(true);
        self.menu_clients.set_visible(b);
        self.menu_users.set_visible(b);
        self.menu_backups.set_visible(b);

        self.menu_login.set_visible(!b);
        self.menu_logout.set_visible(b);
        self.menu_pwchange.set_visible(b);
        self.menu_options.set_visible(b);
        self.menu_dialogs.set_visible(b);
        self.menu_reset.set_visible(true);

        self.menu_diary.set_visible(b);
        self.menu_positions.set_visible(b);
        self.menu_notes.set_visible(b);
        self.menu_persons.set_visible(b);
        self.menu_mileages.set_visible(b);
        self.menu_bikes.set_visible(b);
        self.menu_books.set_visible(b);
        self.menu_authors.set_visible(b);
        self.menu_series.set_visible(b);
        self.menu_statistics.set_visible(b);
        self.menu_sudoku.set_visible(b);
        self.menu_detective.set_visible(b);

        self.menu_bookings.set_visible(b);
        self.menu_events.set_visible(b);
        self.menu_accounts.set_visible(b);
        self.menu_periods.set_visible(b);
        self.menu_finalbalance.set_visible(b);
        self.menu_plbalance.set_visible(b);
        self.menu_openingbalance.set_visible(b);

        self.menu_ancestors.set_visible(b);
        self.menu_families.set_visible(b);
        self.menu_sources.set_visible(b);

        self.menu_stocks.set_visible(b);
        self.menu_configurations.set_visible(b);
        self.menu_chart.set_visible(b);
        self.menu_investments.set_visible(b);
        self.menu_bookings3.set_visible(b);
        self.menu_prices.set_visible(b);

        self.menu_about.set_visible(true);
        self.menu_help2.set_visible(true);
    }

    fn start(&self) {
        while let Some(_) = self.notebook.nth_page(Some(0)) {
            self.notebook.remove_page(Some(0));
        }
        let daten1 = ServiceDaten::new0(&self.config, 0, "Admin");
        let r = crate::services::client_service::init_db(&daten1);
        if !bin::get(&r, Some(&self.window)) {
            functions::mach_nichts();
        }
        let m_nr = parameter::get_login_client();
        let username = parameter::get_login_user();
        let daten1 = ServiceDaten::new0(&self.config, m_nr, username.as_str());
        if cfg!(debug_assertions) {
            // Automatische Anmeldung mit aktuellem Benutzer.
            // let un = "Wolfgangxxx";
            // let unos = OsString::from(un);
            // let username = functions::to_first_upper(
            //     glib::get_user_name()
            //         .unwrap_or(unos)
            //         .into_string()
            //         .unwrap_or(un.to_string())
            //         .as_str(),
            // );
            services::set_daten(&daten1);
            self.login();
            // MainClass.Login(new ServiceDaten(3, "Wolfgang"));
        } else {
            if let Ok(r) = crate::services::login_service::is_without_password(&daten1) {
                if r {
                    services::set_daten(&daten1);
                    self.login();
                } else {
                    self.menu_login.activate();
                }
                return;
            }
        }
        // self.menu_clients.activate();
        // self.menu_users.activate();
        // self.menu_backups.activate();
        // self.menu_login.activate();
        // self.menu_pwchange.activate();
        // self.menu_options.activate();
        // self.menu_dialogs.activate();
        // self.menu_diary.activate();
        // self.menu_positions.activate();
        // self.menu_notes.activate();
        // self.menu_persons.activate();
        // let _ = Ad120Birthdays::new();
        // self.menu_mileages.activate();
        // self.menu_bikes.activate();
        // self.menu_books.activate();
        // self.menu_authors.activate();
        // self.menu_series.activate();
        // self.menu_periods.activate();
        // self.menu_accounts.activate();
        // self.menu_events.activate();
        // self.menu_bookings3.activate();
        // self.menu_openingbalance.activate();
        // self.menu_plbalance.activate();
        // self.menu_finalbalance.activate();
        // self.menu_ancestors.activate();
        // self.menu_families.activate();
        // self.menu_sources.activate();
        // self.menu_stocks.activate();
        // self.menu_chart.activate();
        // self.menu_configurations.activate();
        // self.menu_investments.activate();
        // self.menu_bookings3.activate();
        // self.menu_prices.activate();
        if functions::mach_nichts() == 0 {
            std::thread::spawn(|| {
                let _ = crate::services::https_server::start();
            });
        }
    }

    /// Hinzufügen einer Seite zum Notebook.
    /// * widget: Window für neue Seite.
    /// * msg: Überschrift der Seite als Message.
    fn append_page(&self, widget: gtk::Widget, msg: Messages) {
        let config = config::get_config();
        let title = M::me(msg, config.is_de())
            .replace("_", "")
            .replace("&", "&amp;");
        let close_image = gtk::Image::from_icon_name(Some("window-close"), IconSize::Button);
        let button = gtk::Button::new();
        let lbl = gtk::Label::new(Some(title.as_str()));
        let tab = gtk::Box::new(Orientation::Horizontal, 0);
        button.set_relief(ReliefStyle::None);
        button.add(&close_image);
        tab.pack_start(&lbl, false, false, 0);
        tab.pack_start(&button, false, false, 0);
        tab.show_all();
        let index = self.notebook.append_page(&widget, Some(&tab));
        self.notebook.set_tab_reorderable(&widget, true);
        button.connect_clicked(glib::clone!(@weak self.notebook as notebook => move |_| {
            let index = notebook
                .page_num(&widget)
                .expect("Couldn't get page_num from notebook");
            notebook.remove_page(Some(index));
        }));
        self.notebook.set_page(index as i32);
        // self.notebook.show_all();
    }

    /// Undo last transaction.
    /// * parent: Parent-Window für MessageBox.
    /// * returns: Is anything changed?
    pub fn undo(parent: Option<&gtk::Window>) -> bool {
        let r = login_service::undo(&ServiceDaten::new());
        bin::get(&r, parent);
        r.is_ok()
    }

    /// Redo last transaction.
    /// * parent: Parent-Window für MessageBox.
    /// * returns: Is anything changed?
    pub fn redo(parent: Option<&gtk::Window>) -> bool {
        let r = login_service::redo(&ServiceDaten::new());
        bin::get(&r, parent);
        r.is_ok()
    }

    /// Show not implemented Messages.
    /// * parent: Parent-Window für MessageBox.
    /// * returns: nothing
    pub fn not_implemented(parent: Option<&gtk::Window>) -> () {
        let r: Result<(), RsbpError> =
            Err(RsbpError::error_msg(M::MIMPL, config::get_config().is_de()));
        bin::get(&r, parent);
    }

    /// Beenden der Anwendung
    fn quit(s: &str, exitcode: i32, mw: Option<&Window>) {
        if let Some(w) = mw {
            unsafe {
                w.destroy();
            }
        }
        parameter::save().unwrap_or_else(|op| print!("Parameter save: {}", op.to_string()));
        if cfg!(debug_assertions) {
            println!("Quit: {}", s);
        }
        // gtk::main_quit(); // Fehler: Attempted to quit a GTK main loop when none is running.
        std::process::exit(exitcode);
    }

    /// Anmelden
    fn login(&self) {
        // Berechtigungen nach erfolgreicher Anmeldung setzen.
        let config = config::get_config();
        let daten = services::get_daten();
        let r = Parameter::init(&config, daten.mandant_nr);
        bin::get(&r, Some(&self.window));
        self.set_permission(true);

        // TODO Start-Dialoge starten
        let sd = parameter::get_start_dialogs();
        let arr = sd.split("|");
        for d in arr {
            match &d[1..] {
                "AG100" => {
                    self.menu_clients.activate();
                }
                "AG200" => {
                    self.menu_users.activate();
                }
                "AG400" => {
                    //self.menu_backups.activate();
                }
                "AM500" => {
                    //self.menu_options.activate();
                }
                "AM510" => {
                    self.menu_dialogs.activate();
                }
                "FZ100" => {
                    //self.menu_statistics.activate();
                }
                "FZ200" => {
                    //self.menu_bikes.activate();
                }
                "FZ250" => {
                    //self.menu_mileages.activate();
                }
                "FZ300" => {
                    //self.menu_authors.activate();
                }
                "FZ320" => {
                    //self.menu_series.activate();
                }
                "FZ340" => {
                    //self.menu_books.activate();
                }
                "FZ700" => {
                    //self.menu_notes.activate();
                }
                "HH100" => {
                    //self.menu_periods.activate();
                }
                "HH200" => {
                    //self.menu_accounts.activate();
                }
                "HH300" => {
                    //self.menu_events.activate();
                }
                "HH400" => {
                    //self.menu_bookings.activate();
                }
                "HH500;EB" => {
                    //self.menu_openingbalance.activate();
                }
                "HH500;GV" => {
                    //self.menu_plbalance.activate();
                }
                "HH500;SB" => {
                    //self.menu_finalbalance.activate();
                }
                "SB200" => {
                    //self.menu_ancestors.activate();
                }
                "SB300" => {
                    //self.menu_families.activate();
                }
                "SB400" => {
                    //self.menu_sources.activate();
                }
                "TB100" => {
                    self.menu_diary.activate();
                }
                "TB200" => {
                    self.menu_positions.activate();
                }
                "WP200" => {
                    //self.menu_stocks.activate();
                }
                "WP250" => {
                    //self.menu_investments.activate();
                }
                "WP300" => {
                    //self.menu_configurations.activate();
                }
                "WP400" => {
                    //self.menu_bookings3.activate();
                }
                "WP500" => {
                    //self.menu_prices.activate();
                }
                _ => {
                    functions::mach_nichts();
                }
            }
        }
        if parameter::get_ad120_start() {
            // Geburtstagsliste starten
            let _ = Ad120Birthdays::new();
        }
    }

    /// Abmelden.
    fn logout(&self) {
        // Schließen aller offenen Tabs.
        while self.notebook.n_pages() > 0 {
            if let Some(p) = self.notebook.nth_page(Some(0)) {
                self.notebook.remove(&p);
            }
        }
        // Berechtigungen beim Abmelden zurücksetzen.
        services::set_daten(&ServiceDaten::init());
        self.set_permission(false);
    }

    /// Create an about dialog specific to this application that should be re-used
    /// each time the user asks to open it.
    /// If this is a single window app, set parent to the main window, if not, use `None`.
    fn new_about_dialog<'b, P: glib::object::IsA<gtk::Window> + 'b, Q: Into<Option<&'b P>>>(
        parent: Q,
        logo: Option<gdk_pixbuf::Pixbuf>,
    ) -> gtk::AboutDialog {
        let p = gtk::AboutDialog::new();
        p.set_authors(&["Wolfgang Kuehl"]);
        p.set_copyright(Some(
            String::from("Copyright © 2021 Wolfgang Kuehl.\nAll rights reserved.").as_str(),
        ));
        p.set_modal(false);
        p.set_keep_above(true);
        p.set_destroy_with_parent(true);
        p.set_license_type(gtk::License::MitX11);
        p.set_program_name(res::APP_NAME);
        p.set_skip_pager_hint(true);
        p.set_skip_taskbar_hint(true);
        // I sometimes use a macro to determine language and perform i18n; maybe that's a topic for a future post.
        let title = "Hallo"; // translate!("About");
        p.set_title(&title);
        p.set_transient_for(parent.into());
        // Make sure tiling window managers know that this window should probably be floating.
        //p.set_type_hint(gdk::WindowTypeHint::Splashscreen); // Bekommt keinen Fokus!
        p.set_version(Some(res::VERSION));
        p.set_website(Some("https://cwkuehl.de"));
        p.set_website_label(Some("cwkuehl.de"));
        p.add_credit_section(
            "Open Source",
            &[
                "Gtk-rs http://gtk-rs.org/ (MIT)",
                "Rust https://www.rust-lang.org/ (MIT/Apache-2.0)",
            ],
        );
        let daten = services::get_daten();
        p.set_comments(Some(
            format!(
                "RSBP is a simple budget program.\n Client: {} User: {}",
                daten.mandant_nr, daten.benutzer_id
            )
            .as_str(),
        ));

        // We expect to reuse this dialog across multiple windows, so don't destroy
        // it when the window is closed.
        p.connect_delete_event(|_, _| Inhibit(true)); //p.hide_on_delete()));
        p.connect_response(|p, _| p.hide());

        // If the window manager doesn't use a header bar, slap one on there.
        // I just think this makes the about dialog look nicer on eg. MATE.
        if p.header_bar().is_none() {
            let hbar = gtk::HeaderBar::new();
            hbar.set_title(Some(
                p.title().unwrap_or_else(move || title.into()).as_str(),
            ));
            hbar.show_all();
            p.set_titlebar(Some(&hbar));
        }

        // This isn't strictly necessary, but on about dialogs I like to swap the
        // logo back and forth if the theme changes. This isn't guaranteed to work.
        if let Some(lg) = logo {
            p.set_logo(Some(&lg));
        }
        p
    }
}
