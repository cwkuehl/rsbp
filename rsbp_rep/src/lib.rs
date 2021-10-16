pub mod models;
pub mod models_ext;
mod res;
pub mod revision;
pub mod schema;

// diesel: use von diesel-macros funktioniert nicht.
#[macro_use]
extern crate diesel;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn inner_join() {
        use crate::schema::*;
        use diesel::*;
        let x = TB_EINTRAG_ORT::table
            .inner_join(TB_ORT::table.on(TB_ORT::mandant_nr.eq(TB_EINTRAG_ORT::mandant_nr)));
        print!("inner_join {:?}", x);
    }
}
