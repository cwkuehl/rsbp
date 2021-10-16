use strum_macros::EnumString;

pub type M = Messages;

/// Meldungen und Basisfunktionen zum Übersetzen.
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug)]
pub enum Messages {
    none,
    Enum_permission_no,
    Enum_permission_user,
    Enum_permission_admin,
    Enum_permission_all,
}

impl Messages {
    /// Liefert Meldung zu Messages-Enum in gewünschter Sprache.
    pub fn me<'a>(key: Messages, is_de: bool) -> &'a str {
        if is_de {
            return M::m_de(key);
        }
        return M::m_en(key);
    }

    /// Liefert deutsche Meldung zu Enum.
    pub fn m_de(key: Messages) -> &'static str {
        match key {
            M::none => "",
            M::Enum_permission_no => r#"Keine"#,
            M::Enum_permission_user => r#"Benutzer"#,
            M::Enum_permission_admin => r#"Administrator"#,
            M::Enum_permission_all => r#"Alles"#,
        }
    }

    /// Liefert englische Meldung zu Enum.
    pub fn m_en(key: Messages) -> &'static str {
        match key {
            M::none => "",
            M::Enum_permission_no => r#"No"#,
            M::Enum_permission_user => r#"User"#,
            M::Enum_permission_admin => r#"Administrator"#,
            M::Enum_permission_all => r#"All"#,
        }
    }
}
