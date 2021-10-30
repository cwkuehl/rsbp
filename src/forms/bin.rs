use crate::{
    apis::enums::DialogTypeEnum,
    base::{functions, parameter},
    config::{self, RsbpConfig, RsbpError},
    res::messages::M,
};
use chrono::NaiveDate;
use glib::object::Cast;
use glib::{GString, Object, Value};
use gtk::{
    prelude::*, Builder, ButtonsType, ComboBoxText, Container, Dialog, DialogFlags, Entry, Grid,
    Label, MessageDialog, MessageType, RadioButton, SortColumn, SortType, TextView, TreeModelSort,
    TreeStore, TreeView, TreeViewColumn, WidgetHelpType, Window,
};
use std::{borrow::Cow, cmp::Ordering};

pub trait Form {
    /// Anzeige von Fehlern in einer Messagebox, falls vorhanden.
    /// * r: Betroffenes Ergebnis mit evtl. Fehlermeldungen.
    /// * returns: Ergebnis oder None im Fehlerfalle.
    fn get2<'a, T>(&self, r: &'a Result<T, RsbpError>) -> Option<&'a T> {
        return match r {
            Err(err) => {
                let s = err.to_string();
                println!("get: {}", s);
                None
            }
            Ok(ok) => Some(ok),
        };
    }
}

/// Formular aus glade-Datei erstellen.
pub fn get_title(title_msg: M, dialog_type: &DialogTypeEnum, is_de: bool) -> String {
    let dt_msg = match dialog_type {
        DialogTypeEnum::Without => M::none,
        DialogTypeEnum::New => M::Enum_dialog_new,
        DialogTypeEnum::Copy => M::Enum_dialog_copy,
        DialogTypeEnum::Copy2 => M::Enum_dialog_copy2,
        DialogTypeEnum::Edit => M::Enum_dialog_edit,
        DialogTypeEnum::Delete => M::Enum_dialog_delete,
        DialogTypeEnum::Reverse => M::Enum_dialog_reverse,
    };
    let title = M::me(title_msg, is_de);
    let dt = M::me(dt_msg, is_de);
    if dt.is_empty() {
        return title.to_string();
    }
    format!("{} - {}", title, dt)
}

/// ComboBox mit String-Spalten und Zeilen füllen.
/// * cb: Betroffene ComboBox.
/// * values: Betroffene Werte.
/// * returns: evtl. Fehlermeldungen.
pub fn add_string_columns_cb<'a>(
    cb: &'a ComboBoxText,
    values: Option<Vec<Vec<String>>>,
) -> Result<(), RsbpError> {
    let vtitles: Vec<String> = "Text;ID".split(";").map(String::from).collect();
    if vtitles.len() <= 1 {
        return Err(RsbpError::error_string("Too few column headers."));
    }
    let titles = &vtitles[..];
    let vtypes: Vec<glib::Type> = titles.into_iter().map(|_| glib::Type::STRING).collect();
    let types = &vtypes[..];
    for _ in types {
        let cell = gtk::CellRendererText::new();
        cb.pack_start(&cell, true);
    }
    let store = TreeStore::new(types);
    if let Some(vlist) = values {
        for v in vlist {
            let l = v.len() as u32;
            let vv: Vec<_> = (0..l).zip(v.iter().map(|a| a as &dyn ToValue)).collect();
            store.insert_with_values(None, None, &vv[..]);
        }
    }
    let sortable = TreeModelSort::new(&store);
    cb.set_model(Some(&sortable));
    Ok(())
    //return add_columns_sort(tv, titles, types, false, values);
}

/// Sortierbaren TreeView mit String-Spalten und Zeilen füllen.
/// * tv: Betroffener TreeView.
/// * headers: Spaltenüberschriften mit Semikolon getrennt.
/// * values: Betroffene Werte.
/// * returns: evtl. Fehlermeldungen.
pub fn add_string_columns_sort<'a>(
    tv: &'a TreeView,
    headers: &'a str,
    values: Option<Vec<Vec<String>>>,
) -> Result<(), RsbpError> {
    let vtitles: Vec<String> = headers.split(";").map(String::from).collect();
    if vtitles.len() <= 1 {
        return Err(RsbpError::error_string("Too few column headers."));
    }
    let titles = &vtitles[..];
    let vtypes: Vec<glib::Type> = titles.into_iter().map(|_| glib::Type::STRING).collect();
    let types = &vtypes[..];
    return add_columns_sort(tv, titles, types, false, values);
}

/// Setzen des Default-Buttons für einen Dialog.
fn set_default_button(con: &Container, dlg: &Dialog) {
    let cl = con.children();
    for c in cl.iter() {
        if c.is::<Container>() {
            let w = c.clone().downcast::<Container>().unwrap();
            set_default_button(&w, dlg);
        }
        if c.type_().name() == "GtkButton" && c.can_default() {
            dlg.set_default(Some(c));
        }
    }
}

/// Liefert optionalen Wert einer Eigenschaft.
fn get_property_value<'a>(obj: &'a Object, name: &'a str) -> Option<Cow<'a, str>> {
    if name == "tooltip_text" {
        if let Some(p) = obj.property("has_tooltip").ok() {
            if let Some(b) = p.get::<bool>().ok() {
                if !b {
                    return None;
                }
            }
        }
    }
    if let Some(p) = obj.property(name).ok() {
        if let Some(gstr) = p.get::<GString>().ok() {
            let s = gstr.as_str();
            return Some(Cow::Owned(s.to_string().clone()));
        }
    }
    None
}

/// Label fett darstellen.
pub fn set_bold(lbl: &Label) {
    lbl.set_use_markup(true);
    let l = lbl.label();
    let l = format!("<b>{}</b>", l.as_str());
    lbl.set_label(l.as_str());
}

/// Texte in Entry, Label und Button lokalisieren und Default-Button setzen.
pub fn make_locale(
    builder: &Builder,
    config: &RsbpConfig,
    dialog: Option<&Dialog>,
    type_name: &String,
) {
    let cl = builder.objects();
    for c in cl.iter() {
        if c.type_().name() == "GtkLabel" {
            if let Some(s) = get_property_value(&c, "label") {
                let s = s.into_owned();
                if s.contains(".") {
                    let s1 = M::ms(&s, config.is_de()).into_owned();
                    set_property_value(c, "label", s1.as_str());
                }
            }
        } else if c.type_().name() == "GtkEntry" {
            if let Some(s) = get_property_value(&c, "placeholder_text") {
                let s = s.into_owned();
                if s.ends_with(".tt") {
                    let s1 = M::ms(&s, config.is_de()).into_owned();
                    set_property_value(c, "placeholder_text", s1.as_str());
                }
            }
        } else if c.type_().name() == "GtkButton"
            || c.type_().name() == "GtkCheckButton"
            || c.type_().name() == "GtkRadioButton"
        {
            if let Some(s) = get_property_value(&c, "tooltip_text") {
                let s = s.into_owned();
                if s.starts_with("Action.") {
                    let s1 = M::ms(&s, config.is_de()).into_owned();
                    set_property_value(c, "tooltip_text", s1.as_str());
                }
            }
            if let Some(s) = get_property_value(&c, "label") {
                let s = s.into_owned();
                if s.contains(".") {
                    let s1 = M::ms(&s, config.is_de()).into_owned();
                    set_property_value(c, "label", s1.as_str());
                }
            }
        }
        if let Some(s) = get_property_value(&c, "tooltip_text") {
            let s = s.into_owned();
            if s.ends_with(".tt") {
                let s1 = M::ms(&s, config.is_de()).into_owned();
                set_property_value(c, "tooltip_text", s1.as_str());
            }
        }
    }
    if let Some(dlg) = dialog {
        let con = dlg
            .clone()
            .upcast::<gtk::Widget>()
            .downcast::<gtk::Container>()
            .unwrap();
        set_default_button(&con, dlg);
        if !type_name.is_empty() {
            let size = parameter::get_dialog_size(type_name.as_str());
            dlg.set_default_size(size.width, size.height);
            if size.x == -1 && size.y == -1 {
                dlg.set_position(gtk::WindowPosition::Center);
            } else {
                dlg.move_(size.x, size.y);
            }
            dlg.connect_configure_event(glib::clone!(@strong type_name => move |_, e| {
                // println!("connect_configure_event {:?} {:?}", e.position(), e.size());
                let p = e.position();
                let s = e.size();
                let r = parameter::Rectangle {
                    x: p.0,
                    y: p.1 - *super::main_window::TITLE_HEIGHT,
                    width: s.0 as i32,
                    height: s.1 as i32,
                };
                parameter::set_dialog_size(&type_name, &r);
                false
            }));
        }
    }
}

/// Anzeige von Fehlern in einer Messagebox, falls vorhanden.
/// * r: Betroffenes Ergebnis mit evtl. Fehlermeldungen.
/// * returns: Ist alles OK, d.h. keine Fehler?
pub fn get2<T>(r: &Result<T, RsbpError>) -> bool {
    if let Err(err) = r {
        let s = err.to_string();
        let md = MessageDialog::new::<gtk::Window>(
            None,
            DialogFlags::DESTROY_WITH_PARENT,
            MessageType::Error,
            ButtonsType::Close,
            s.as_str(),
        );
        md.run();
        unsafe {
            md.destroy();
        }
        return false;
    }
    true
}

/// Anzeige von Fehlern in einer Messagebox, falls vorhanden.
/// * r: Betroffenes Ergebnis mit evtl. Fehlermeldungen.
/// * parent: Betroffener Eltern-Dialog kann None sein.
/// * returns: Ist alles OK, d.h. keine Fehler?
pub fn get<T, S: IsA<Window>>(r: &Result<T, RsbpError>, parent: Option<&S>) -> bool {
    if let Err(err) = r {
        let s = err.to_string();
        let md = MessageDialog::new(
            parent,
            DialogFlags::DESTROY_WITH_PARENT,
            MessageType::Error,
            ButtonsType::Close,
            s.as_str(),
        );
        md.run();
        unsafe {
            md.destroy();
        }
        return false;
    }
    true
}

/// Setzen einer Eigenschaft.
fn set_property_value<'a>(obj: &'a Object, name: &'a str, value: &'a str) {
    let v = Value::from(value);
    obj.set_property(name, &v).ok();
}

/// Sortierbaren TreeView mit Spalten und Zeilen füllen.
/// * tv: Betroffener TreeView.
/// * headers: Spaltenüberschriften mit Semikolon getrennt.
/// * values: Betroffene Werte.
/// * returns: evtl. Fehlermeldungen.
fn add_columns_sort<'a>(
    tv: &'a TreeView,
    titles: &'a [String],
    types: &'a [glib::Type],
    editable: bool,
    values: Option<Vec<Vec<String>>>,
) -> Result<(), RsbpError> {
    for c in tv.columns() {
        tv.remove_column(&c);
    }
    let store = TreeStore::new(types);
    for (i, title) in titles.iter().enumerate() {
        let col = get_column(title, i as i32, editable, &store);
        tv.append_column(&col);
    }
    if let Some(vlist) = values {
        for v in vlist {
            let l = v.len() as u32;
            let vv: Vec<_> = (0..l).zip(v.iter().map(|a| a as &dyn ToValue)).collect();
            store.insert_with_values(None, None, &vv[..]);
        }
    }
    //tv.set_model(Some(&store));
    let sortable = TreeModelSort::new(&store);
    for (i, title) in titles.iter().enumerate() {
        if title.ends_with("_r") {
            let j = i as i32;
            sortable.set_sort_func(SortColumn::Index(i as u32), move |model, a, b| {
                let a1 = model.value(a, j);
                let b1 = model.value(b, j);
                let mut s1 = 0_f32;
                let mut s2 = 0_f32;
                if let Some(a2) = a1.get::<String>().ok() {
                    s1 = functions::to_f32(a2.as_str());
                }
                if let Some(b2) = b1.get::<String>().ok() {
                    s2 = functions::to_f32(b2.as_str());
                }
                if s1 < s2 {
                    return Ordering::Less;
                } else if s1 > s2 {
                    return Ordering::Greater;
                }
                return Ordering::Equal;
            });
        }
    }
    tv.set_model(Some(&sortable));
    if titles.len() == 2 {
        // tv.selection().set_mode(gtk::SelectionMode::Single);
        tv.set_enable_search(true);
        tv.set_search_column(1);
    } else {
        tv.set_enable_search(false);
    }
    Ok(())
}

/// Get the value of a TreeView.
/// * tv: Affected treeview.
/// * mandatory: Is the value mandatory?
/// * column: Affected column number.
/// * returns: Value or error.
pub fn get_text_tv(
    tv: &TreeView,
    mandatory: bool,
    column: i32,
) -> Result<Option<String>, RsbpError> {
    let mut v: Option<String> = None;
    let s = tv.selection().selected_rows();
    if s.0.len() > 0 {
        if let Some(tp1) = s.0.first() {
            if let Some(iter) = s.1.iter(tp1) {
                let val = s.1.value(&iter, column);
                if let Some(v2) = val.get::<String>().ok() {
                    v = Some(v2);
                }
            }
        }
    } else if s.1.iter_n_children(None) == 1 {
        // Wenn nur 1 Zeile da ist, wird diese genommen.
        if let Some(iter) = s.1.iter_first() {
            let val = s.1.value(&iter, column);
            if let Some(v2) = val.get::<String>().ok() {
                v = Some(v2);
            }
        }
    }
    if mandatory && v.is_none() {
        let config = config::get_config();
        return Err(RsbpError::error_msg(M::M1013, config.is_de()));
    }
    Ok(v)
}

/// Get the selected values of a TreeView.
/// * tv: Affected treeview.
/// * column: Affected column number.
/// * returns: Selected values.
pub fn get_selected_tv(tv: &gtk::TreeView, column: i32) -> Vec<String> {
    let mut list: Vec<String> = vec![];
    if tv.model().is_none() {
        return list;
    }
    let s = tv.selection().selected_rows();
    for sel in s.0 {
        if let Some(iter) = s.1.iter(&sel) {
            let val = s.1.value(&iter, column);
            if let Some(v2) = val.get::<String>().ok() {
                list.push(v2);
            }
        }
    }
    list
}

/// Set the value of a TreeView.
/// * tv: Affected treeview.
/// * v: Value to set.
/// * returns: Is value set? or possibly errors.
pub fn set_text_tv(tv: &TreeView, v: Option<String>) -> Result<bool, RsbpError> {
    tv.selection().unselect_all();
    if let (Some(store), Some(value), Some(c)) = (tv.model(), v, &tv.column(0)) {
        if let Some(i1) = &store.iter_first() {
            let i = i1;
            loop {
                let v0 = store.value(i, 0);
                if let Some(val) = v0.get::<String>().ok() {
                    if val == value {
                        tv.selection().select_iter(i);
                        if let Some(path) = &store.path(i) {
                            tv.scroll_to_cell(Some(path), Some(c), false, 0_f32, 0_f32);
                            return Ok(true);
                        }
                    }
                }
                if !store.iter_next(i) {
                    break;
                }
            }
        }
    }
    Ok(false)
}

/// Get the value of a TextView.
/// * tv: Affected textview.
/// * returns: Value or error.
pub fn _get_otext_textview(tv: &TextView) -> Option<String> {
    if let Some(buffer) = tv.buffer() {
        let (s, e) = buffer.bounds();
        if let Some(v) = buffer.text(&s, &e, true) {
            return Some(v.to_string());
        }
    }
    return None;
}

/// Get the value of a TextView.
/// * tv: Affected textview.
/// * returns: Value or error.
pub fn get_text_textview(tv: &TextView) -> String {
    if let Some(buffer) = tv.buffer() {
        let (s, e) = buffer.bounds();
        if let Some(v) = buffer.text(&s, &e, true) {
            return v.to_string();
        }
    }
    String::new()
}

/// Set the value of a TextView.
/// * tv: Affected textview.
/// * s: Affected string.
pub fn set_text_textview(tv: &TextView, s: &Option<String>) {
    if let Some(buffer) = tv.buffer() {
        if let Some(str) = s {
            buffer.set_text(str);
        } else {
            buffer.set_text("");
        };
    }
}

/// Get the value of a Entry.
/// * tv: Affected entry.
/// * returns: Value as string.
pub fn get_text_entry(tv: &Entry) -> String {
    tv.text().as_str().to_string()
}

/// Set the value of an Entry.
/// * tv: Affected entry.
/// * s: Affected string.
pub fn set_text_entry(tv: &Entry, s: &Option<String>) {
    if let Some(str) = s {
        tv.set_text(str.as_str());
    } else {
        tv.set_text("");
    }
}

/// Set the data of a RadioButton group.
/// * rbd: Affected radiobutton group with data.
pub fn init_data_rb(rbd: Vec<(&RadioButton, &str)>) {
    unsafe {
        for r in rbd.iter() {
            r.0.set_data::<String>("v", r.1.to_string());
        }
    }
}

/// Set the value of a RadioButton.
/// * rb: Affected radiobutton.
/// * v: Value to set.
/// * returns: Is value set?.
pub fn set_text_rb(rb: &RadioButton, v: &String) -> bool {
    unsafe {
        for r in rb.group().iter() {
            if let Some(valnn) = r.data::<String>("v") {
                let val = valnn.as_ref();
                if v == val {
                    r.set_active(true);
                    return true;
                }
            }
        }
    }
    rb.set_active(true);
    false
}

/// Get the value of a RadioButton.
/// * rb: Affected radiobutton.
/// * returns: Is the value set?
pub fn get_text_rb(rb: &RadioButton) -> String {
    unsafe {
        for r in rb.group().iter() {
            if r.is_active() {
                if let Some(valnn) = r.data::<String>("v") {
                    let val = valnn.as_ref();
                    return val.to_string();
                }
            }
        }
    }
    "".into()
}

/// Set the value of a ComboBox.
/// * cb: Affected combobox.
/// * value: Value to set.
/// * returns: Is the value set?
pub fn set_text_cb(cb: &ComboBoxText, value: &Option<String>) -> bool {
    cb.set_active(None);
    if let (Some(store), Some(va)) = (cb.model(), value) {
        let first = gtk::TreePath::new_first();
        for i in store.iter(&first) {
            let v = store.value(&i, 1);
            if let Some(val) = v.get::<String>().ok() {
                if val == *va {
                    cb.set_active_iter(Some(&i));
                    return true;
                }
            }
        }
    }
    false
}

/// Get the value of a ComboBox.
/// * rb: Affected combobox.
/// * returns: Value as String.
pub fn get_text_cb(cb: &ComboBoxText) -> Option<String> {
    if let (Some(store), Some(r)) = (cb.model(), cb.active_iter()) {
        let v = store.value(&r, 1);
        if let Some(val) = v.get::<String>().ok() {
            return Some(val.to_string());
        }
    }
    None
}

/// Get the date value of a Grid.
/// * grid: Affected grid.
/// * returns: Value or error.
pub fn get_date_grid(grid: &Grid) -> Option<NaiveDate> {
    unsafe {
        if let Some(valnn) = grid.data::<Option<NaiveDate>>("date") {
            let val = valnn.as_ref().clone();
            return val;
        }
    }
    None
}

/// Sets the date value of a Grid.
/// * grid: Affected grid.
/// * v: Value to set.
/// * emit_signal: Emit signal?
pub fn set_date_grid(grid: &Grid, v: &Option<NaiveDate>, emit_signal: bool) {
    unsafe {
        grid.set_data::<Option<NaiveDate>>("date", v.clone());
    }
    if emit_signal {
        // Trick: Signal by non existent popup menu.
        grid.emit_popup_menu();
    }
}

/// Get the month value of a Grid.
/// * grid: Affected grid.
/// * returns: Value or error.
pub fn get_month_grid(grid: &Grid) -> Option<Vec<bool>> {
    unsafe {
        if let Some(valnn) = grid.data::<Option<Vec<bool>>>("month") {
            let val = valnn.as_ref().clone();
            return val;
        }
    }
    None
}

/// Sets the month value of a Grid.
/// * grid: Affected grid.
/// * v: Value to set.
/// * emit_signal: Emit signal?
pub fn set_month_grid(grid: &Grid, v: &Option<Vec<bool>>, emit_signal: bool) {
    unsafe {
        grid.set_data::<Option<Vec<bool>>>("month", v.clone());
    }
    if emit_signal {
        // Trick: Signal by non existent popup menu.
        // grid.emit_popup_menu();
        grid.emit_show_help(WidgetHelpType::__Unknown(1));
    }
}

/// Refreshes a TreeView.
/// * tv: Affected treeview.
/// * step: Betroffener Schritt: 0 erstmalig, 1 aktualisieren.
/// * value: Affected value to select.
/// * returns: Value or error
pub fn refresh_treeview<F>(
    tv: &TreeView,
    init_data: F,
    value: Option<String>,
) -> Result<(), RsbpError>
where
    F: FnOnce() -> (),
{
    let mut v = value;
    if v.is_none() {
        v = get_text_tv(tv, false, 0)?;
    }
    let s = tv.selection().selected_rows();
    let mut si = -1;
    let mut so = SortType::Ascending;
    for i in 1..tv.n_columns() as i32 {
        if let Some(c) = tv.column(i) {
            if c.is_sort_indicator() {
                so = c.sort_order();
                si = i;
                break;
            }
        }
    }
    init_data();
    if si > 0 {
        // Sortierung wiederherstellen
        if let Some(c) = tv.column(si) {
            if let Some(b) = c.button() {
                b.activate();
                if so == SortType::Descending {
                    b.activate();
                }
            }
        }
    } else if v.is_none() {
        for p in s.0 {
            tv.selection().select_path(&p);
        }
    } else {
        let _r = set_text_tv(tv, v);
    }
    Ok(())
}

/// Liefert eine TreeView-Spalte.
/// * title: Betroffene Spaltenüberschrift.
/// * i: Betroffene Spaltennummer.
/// * editable: Spalte editierbar?
/// * store: Betroffenes Model.
/// * returns: Eine TreeView-Spalte.
fn get_column<'a>(title: &'a str, i: i32, editable: bool, _store: &'a TreeStore) -> TreeViewColumn {
    let mut align = 0_f32; // left
    let mut t = title.to_string();
    let mut edit = editable;
    if title.ends_with("_r") {
        t = title[..title.len() - 2].to_string();
        align = 1.0; // right
    } else if title.ends_with("_e") {
        t = title[..title.len() - 2].to_string();
        edit = true;
    }
    let cell = gtk::CellRendererText::new();
    cell.set_xalign(align);
    cell.set_editable(edit);
    if edit {
        // TODO cell.set_data("store", store);
        //cell.set_data("cnr", i);
        //cell.Edited += TableCell_Edited;
    }
    let column = gtk::TreeViewColumn::new();
    column.set_sort_column_id(i);
    column.set_resizable(true);
    column.set_alignment(align);
    if edit {
        // TODO column.set_data("cnr", i);
    }
    t = format!("<b>{}</b>", t);
    let lbl = gtk::Label::new(Some(t.as_str()));
    lbl.set_use_markup(true);
    lbl.show();
    column.set_widget(Some(&lbl));
    //column.set_title(title);
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", i);
    if i == 0 {
        // Nicht 0 wegen: Negative content width -12 (allocation 1, extents 6x7) while allocating gadget (node button, owner GtkButton)
        column.set_max_width(13);
    }
    column
}
