use chrono::NaiveDateTime;

pub trait Revision {
    fn get_angelegt_von(&self) -> Option<String>;
    fn set_angelegt_von(&mut self, von: &Option<String>);
    fn get_angelegt_am(&self) -> Option<NaiveDateTime>;
    fn set_angelegt_am(&mut self, am: &Option<NaiveDateTime>);
    fn get_geaendert_von(&self) -> Option<String>;
    fn set_geaendert_von(&mut self, von: &Option<String>);
    fn get_geaendert_am(&self) -> Option<NaiveDateTime>;
    fn set_geaendert_am(&mut self, am: &Option<NaiveDateTime>);
}
