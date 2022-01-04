pub mod messages;

use gdk_pixbuf::{self, traits::PixbufLoaderExt};

/// The name of the application.
pub const APP_NAME: &str = "RSBP Rust-Haushalts-Programm W. Kuehl";

/// The GTK application ID.
pub const APP_ID: &str = "de.cwkuehl.rsbp";

/// The version of the app.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The logo to be shown in the about dialog.
const IMG_LOGO: &[u8] = include_bytes!("res/icons/WKHH.gif");

/// Loads the logo included in the binary into a pixbuf.
pub fn load_logo() -> Result<Option<gdk_pixbuf::Pixbuf>, glib::Error> {
    Ok({
        let logoloader = gdk_pixbuf::PixbufLoader::new();
        logoloader.write(IMG_LOGO)?;
        logoloader.close()?;
        logoloader.pixbuf()
    })
}

/// Benutzer-ID für Initialisierung.
pub const USER_ID: &str = "Benutzer-ID";

/// Mandant-Einstellung: OHNE_ANMELDUNG.
pub const EINST_MA_OHNE_ANMELDUNG: &str = "OHNE_ANMELDUNG";

/// Mandant-Einstellung: REPLIKATION_UID.
pub const EINST_MA_REPLIKATION_UID: &str = "REPLIKATION_UID";

/// Zeit in Millisekunden für Änderungsintervall.
pub const AEND_ZEIT: i64 = 60_000;

#[cfg(test)]
mod tests {
    use heck::ToSnakeCase;
    use quick_xml::{events::Event, Reader};

    /// Generieren einer GUI-Struktur.
    #[test]
    fn generate_gui() {
        let glade_src = include_str!("res/gtkgui/tb/TB200Positions.glade");
        let mut reader = Reader::from_str(glade_src);
        reader.trim_text(true);

        let mut data = true;
        let mut buf = Vec::new();
        let mut sb = "pub struct xxx {
    dialog_type: DialogTypeEnum,
    parent: yyy,
    window: gtk::Dialog,
    uid: String,"
            .to_string();
        let mut sb2 = "        let w = Ag210User {
            dialog_type: dialog_type.clone(),
            parent: parent.clone(),
            window: gtk::Dialog::new(),
            uid,"
            .to_string();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"object" => {
                        let atts = e
                            .attributes()
                            .map(|a| {
                                let at = a.unwrap();
                                (
                                    std::str::from_utf8(at.key).unwrap(),
                                    std::str::from_utf8(&at.value).unwrap().to_string(),
                                )
                            })
                            .collect::<Vec<_>>();
                        let class = get_attribut_value(&atts, "class");
                        let id = get_attribut_value(&atts, "id");
                        let mut name = match data == true && class == "GtkGrid" {
                            true => {
                                data = false;
                                "grid".to_snake_case().to_string()
                            }
                            false => id.to_snake_case(),
                        };
                        if name.ends_with("_action") {
                            name = name.replace("_action", "");
                        }
                        if !(name == ""
                            || class == "GtkImage"
                            || class == "GtkActionBar"
                            || class == "GtkScrolledWindow")
                        {
                            sb.push_str(
                                format!(
                                    "
    {}: {},",
                                    name,
                                    class.replace("Gtk", "gtk::")
                                )
                                .as_str(),
                            );
                            sb2.push_str(
                                format!(
                                    r#"
            {}: builder.object::<{}>("{}").unwrap(),"#,
                                    name,
                                    class.replace("Gtk", "gtk::"),
                                    id,
                                )
                                .as_str(),
                            );
                            //println!("class {} id {}", class, id);
                        }
                    }
                    _ => (),
                },
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other events we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        sb.push_str(
            "
}
",
        );
        sb2.push_str(
            "
        }
",
        );
        println!("{}{}", sb, sb2);
    }

    fn get_attribut_value(atts: &Vec<(&str, String)>, key: &str) -> String {
        if let Some(att) = atts.iter().filter(|a| a.0 == key).next() {
            return att.1.to_string();
        }
        "".into()
    }
}
