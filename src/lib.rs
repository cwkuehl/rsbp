//! # rsbp Crate
//!
//! `rsbp` ist das beliebte Haushalts-Programm, das in der Sprache Rust implementiert ist.

pub mod apis;
pub mod base;
pub mod config;
pub mod forms;
pub mod res;
pub mod services;

use config::RsbpError;

/// A typedef of the result returned by many methods.
pub type Result<T> = core::result::Result<T, RsbpError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn inner_join() {
        use diesel::*;
        use rsbp_rep::schema::*;
        let x = TB_EINTRAG_ORT::table
            .inner_join(TB_ORT::table.on(TB_ORT::mandant_nr.eq(TB_EINTRAG_ORT::mandant_nr)));
        print!("inner_join {:?}", x);
    }
}
