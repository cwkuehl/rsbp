use crate::{apis::services, base::functions, config, forms::bin, res::messages::M};
use chrono::{Datelike, NaiveDate};
use glib::ObjectExt;
use gtk::prelude::{
    ButtonExt, CalendarExt, EditableExt, EntryExt, GridExt, LabelExt, ToggleButtonExt, WidgetExt,
};
use std::{cell::RefCell, rc::Rc};

pub trait DateCallback: Clone {
    fn date_callback(&mut self, event: &DateEvent);
}

pub enum DateEvent {
    /// Date unchanged
    Unchanged,
    /// Date changed
    Date {
        name: String,
        date: Option<NaiveDate>,
    },
    /// Month changed
    Month {
        name: String,
        date: Option<NaiveDate>,
    },
}

/// Date control.
#[derive(Clone)]
pub struct Date {
    grid: gtk::Grid,
    name: String,
    date: gtk::Entry,
    down: gtk::Button,
    daytext: gtk::Label,
    yesterday: gtk::Button,
    tomorrow: gtk::Button,
    pub calendar: gtk::Calendar,
    nullable: bool,
    with_calendar: bool,
    open: bool,
    unknown: gtk::CheckButton,
    value: Option<NaiveDate>,
}

impl Date {
    /// Create a new date control with callback.
    pub fn new<'a, T>(
        g: &gtk::Grid,
        callback: &'a Rc<RefCell<T>>,
        name: &'a str,
        nullable: bool,
        with_calendar: bool,
        open: bool,
    ) -> Rc<RefCell<Self>>
    where
        T: 'static + DateCallback,
    {
        let de = config::get_config().is_de();
        let unknown = gtk::CheckButton::new();
        unknown.set_label(M::me(M::Date_unknown, de));
        unknown.set_visible(true);
        unknown.set_can_focus(true);
        unknown.set_tooltip_text(Some(M::me(M::Date_unknown_tt, de)));
        unknown.set_no_show_all(true);
        g.attach(&unknown, 0, 0, 1, 1);
        let date = gtk::Entry::new();
        date.set_visible(true);
        date.set_can_focus(true);
        date.set_hexpand(false);
        date.set_width_chars(10);
        g.attach(&date, 1, 0, 1, 1);
        let down = gtk::Button::new();
        down.set_label("v");
        down.set_visible(true);
        down.set_can_focus(true);
        down.set_receives_default(true);
        down.set_no_show_all(true);
        g.attach(&down, 2, 0, 1, 1);
        let daytext = gtk::Label::new(Some(""));
        daytext.set_visible(true);
        daytext.set_hexpand(false);
        daytext.set_margin_start(5);
        daytext.set_width_chars(10);
        g.attach(&daytext, 3, 0, 1, 1);
        let yesterday = gtk::Button::new();
        yesterday.set_label(M::me(M::Date_yesterday, de));
        yesterday.set_visible(true);
        yesterday.set_can_focus(true);
        yesterday.set_hexpand(true);
        yesterday.set_focus_on_click(false);
        yesterday.set_tooltip_text(Some(M::me(M::Date_yesterday_tt, de)));
        yesterday.set_use_underline(true);
        yesterday.set_margin_start(5);
        g.attach(&yesterday, 4, 0, 1, 1);
        let today = gtk::Button::new();
        today.set_label(M::me(M::Date_today, de));
        today.set_visible(true);
        today.set_can_focus(true);
        today.set_hexpand(true);
        today.set_focus_on_click(false);
        today.set_tooltip_text(Some(M::me(M::Date_today_tt, de)));
        today.set_use_underline(true);
        g.attach(&today, 5, 0, 1, 1);
        let tomorrow = gtk::Button::new();
        tomorrow.set_label(M::me(M::Date_tomorrow, de));
        tomorrow.set_visible(true);
        tomorrow.set_can_focus(true);
        tomorrow.set_hexpand(true);
        tomorrow.set_focus_on_click(false);
        tomorrow.set_tooltip_text(Some(M::me(M::Date_tomorrow_tt, de)));
        tomorrow.set_use_underline(true);
        g.attach(&tomorrow, 6, 0, 1, 1);
        let calendar = gtk::Calendar::new();
        calendar.set_visible(true);
        calendar.set_can_focus(true);
        calendar.set_hexpand(true);
        calendar.set_vexpand(false);
        calendar.set_no_show_all(true);
        g.attach(&calendar, 0, 1, 7, 1);
        let d = Self {
            grid: g.clone(),
            name: name.to_string(),
            date: date.clone(),
            down: down.clone(),
            daytext: daytext.clone(),
            yesterday: yesterday.clone(),
            tomorrow: tomorrow.clone(),
            calendar: calendar.clone(),
            nullable,
            with_calendar,
            open,
            unknown,
            value: None,
        };
        let cb = callback.clone();
        let d2 = Rc::new(RefCell::new(d));
        date.connect_key_release_event(glib::clone!(@strong cb, @strong d2 => move |_,_| {
            cb.borrow_mut().date_callback(&d2.borrow_mut().on_date_key());
            d2.borrow_mut().on_refresh();
            return gtk::Inhibit(false);
        }));
        down.connect_clicked(
            glib::clone!(@strong cb, @strong d2 => move |_| { d2.borrow_mut().on_down(); }),
        );
        yesterday.connect_clicked(glib::clone!(@strong cb, @strong d2 => move |_| {
            cb.borrow_mut().date_callback(&d2.borrow_mut().on_yesterday());
            if let Some(month) = bin::get_month_grid(&d2.borrow().grid) {
                bin::set_month_grid(&d2.borrow().grid, &None, false);
                d2.borrow().mark_month(&month);
            }
        }));
        today.connect_clicked(glib::clone!(@strong cb, @strong d2 => move |_| {
            cb.borrow_mut().date_callback(&d2.borrow_mut().on_today());
            if let Some(month) = bin::get_month_grid(&d2.borrow().grid) {
                bin::set_month_grid(&d2.borrow().grid, &None, false);
                d2.borrow().mark_month(&month);
            }
        }));
        tomorrow.connect_clicked(glib::clone!(@strong cb, @strong d2 => move |_| {
            cb.borrow_mut().date_callback(&d2.borrow_mut().on_tomorrow());
            if let Some(month) = bin::get_month_grid(&d2.borrow().grid) {
                bin::set_month_grid(&d2.borrow().grid, &None, false);
                d2.borrow().mark_month(&month);
            }
        }));
        calendar.connect_day_selected(
            glib::clone!(@strong cb, @strong d2 => move |cal: &gtk::Calendar| {
                unsafe {
                    if let Some(valnn) = cal.data::<&str>("recursion") {
                        let val = valnn.as_ref().clone();
                        if val == "1" {
                            return;
                        }
                    }
                }
                cb.borrow_mut().date_callback(&d2.borrow_mut().on_calendar());
                if let Some(month) = bin::get_month_grid(&d2.borrow().grid) {
                    bin::set_month_grid(&d2.borrow().grid, &None, false);
                    d2.borrow().mark_month(&month);
                }
            }),
        );
        g.connect_show(glib::clone!(
            @strong d2 => move |_| {
            d2.borrow_mut().on_show();
        }));
        g.connect_popup_menu(glib::clone!(
            @strong cb, @strong d2 => move |x| {
            let date = bin::get_date_grid(x);
            d2.borrow_mut().set_value(date, 1);
            // if let Some(month) = bin::get_month_grid(x) {
            //     bin::set_month_grid(x, &None, false);
            //     d2.borrow().mark_month(&month);
            // }
            false
        }));
        // g.connect_show_help(glib::clone!(
        //     @strong cb, @strong d2 => move |x,_type| {
        //     if let Some(month) = bin::get_month_grid(x) {
        //         bin::set_month_grid(x, &None, false);
        //         d2.borrow().mark_month(&month);
        //     }
        //     false
        // }));
        let u = d2.borrow().unknown.clone();
        u.connect_popup_menu(glib::clone!(@strong d2 => move |_| {
            d2.borrow_mut().on_refresh();
            false
        }));
        g.hide();
        // Show doch nicht beim Aufrufer:
        g.show();
        d2
    }

    /// Create a new date control without callback.
    pub fn new2<'a>(
        g: &gtk::Grid,
        name: &'a str,
        nullable: bool,
        with_calendar: bool,
        open: bool,
    ) -> Rc<RefCell<Self>> {
        let de = config::get_config().is_de();
        let unknown = gtk::CheckButton::new();
        unknown.set_label(M::me(M::Date_unknown, de));
        unknown.set_visible(true);
        unknown.set_can_focus(true);
        unknown.set_tooltip_text(Some(M::me(M::Date_unknown_tt, de)));
        unknown.set_no_show_all(true);
        g.attach(&unknown, 0, 0, 1, 1);
        let date = gtk::Entry::new();
        date.set_visible(true);
        date.set_can_focus(true);
        date.set_hexpand(false);
        date.set_width_chars(10);
        g.attach(&date, 1, 0, 1, 1);
        let down = gtk::Button::new();
        down.set_label("v");
        down.set_visible(true);
        down.set_can_focus(true);
        down.set_receives_default(true);
        down.set_no_show_all(true);
        g.attach(&down, 2, 0, 1, 1);
        let daytext = gtk::Label::new(Some(""));
        daytext.set_visible(true);
        daytext.set_hexpand(false);
        daytext.set_margin_start(5);
        daytext.set_width_chars(10);
        g.attach(&daytext, 3, 0, 1, 1);
        let yesterday = gtk::Button::new();
        yesterday.set_label(M::me(M::Date_yesterday, de));
        yesterday.set_visible(true);
        yesterday.set_can_focus(true);
        yesterday.set_hexpand(true);
        yesterday.set_focus_on_click(false);
        yesterday.set_tooltip_text(Some(M::me(M::Date_yesterday_tt, de)));
        yesterday.set_use_underline(true);
        yesterday.set_margin_start(5);
        g.attach(&yesterday, 4, 0, 1, 1);
        let today = gtk::Button::new();
        today.set_label(M::me(M::Date_today, de));
        today.set_visible(true);
        today.set_can_focus(true);
        today.set_hexpand(true);
        today.set_focus_on_click(false);
        today.set_tooltip_text(Some(M::me(M::Date_today_tt, de)));
        today.set_use_underline(true);
        g.attach(&today, 5, 0, 1, 1);
        let tomorrow = gtk::Button::new();
        tomorrow.set_label(M::me(M::Date_tomorrow, de));
        tomorrow.set_visible(true);
        tomorrow.set_can_focus(true);
        tomorrow.set_hexpand(true);
        tomorrow.set_focus_on_click(false);
        tomorrow.set_tooltip_text(Some(M::me(M::Date_tomorrow_tt, de)));
        tomorrow.set_use_underline(true);
        g.attach(&tomorrow, 6, 0, 1, 1);
        let calendar = gtk::Calendar::new();
        calendar.set_visible(true);
        calendar.set_can_focus(true);
        calendar.set_hexpand(true);
        calendar.set_vexpand(false);
        calendar.set_no_show_all(true);
        g.attach(&calendar, 0, 1, 7, 1);
        let d = Self {
            grid: g.clone(),
            name: name.to_string(),
            date: date.clone(),
            down: down.clone(),
            daytext: daytext.clone(),
            yesterday: yesterday.clone(),
            tomorrow: tomorrow.clone(),
            calendar: calendar.clone(),
            nullable,
            with_calendar,
            open,
            unknown,
            value: None,
        };
        let d2 = Rc::new(RefCell::new(d));
        date.connect_key_release_event(glib::clone!(@strong d2 => move |_,_| {
            d2.borrow_mut().on_date_key();
            return gtk::Inhibit(false);
        }));
        down.connect_clicked(glib::clone!(@strong d2 => move |_| { d2.borrow_mut().on_down(); }));
        yesterday.connect_clicked(glib::clone!(@strong d2 => move |_| {
            d2.borrow_mut().on_yesterday();
            if let Some(month) = bin::get_month_grid(&d2.borrow().grid) {
                bin::set_month_grid(&d2.borrow().grid, &None, false);
                d2.borrow().mark_month(&month);
            }
        }));
        today.connect_clicked(glib::clone!(@strong d2 => move |_| {
            d2.borrow_mut().on_today();
            if let Some(month) = bin::get_month_grid(&d2.borrow().grid) {
                bin::set_month_grid(&d2.borrow().grid, &None, false);
                d2.borrow().mark_month(&month);
            }
        }));
        tomorrow.connect_clicked(glib::clone!(@strong d2 => move |_| {
            d2.borrow_mut().on_tomorrow();
            if let Some(month) = bin::get_month_grid(&d2.borrow().grid) {
                bin::set_month_grid(&d2.borrow().grid, &None, false);
                d2.borrow().mark_month(&month);
            }
        }));
        calendar.connect_day_selected(glib::clone!(@strong d2 => move |cal: &gtk::Calendar| {
            unsafe {
                if let Some(valnn) = cal.data::<&str>("recursion") {
                    let val = valnn.as_ref().clone();
                    if val == "1" {
                        return;
                    }
                }
            }
            d2.borrow_mut().on_calendar();
            if let Some(month) = bin::get_month_grid(&d2.borrow().grid) {
                bin::set_month_grid(&d2.borrow().grid, &None, false);
                d2.borrow().mark_month(&month);
            }
        }));
        g.connect_show(glib::clone!(
            @strong d2 => move |_| {
            d2.borrow_mut().on_show();
        }));
        g.connect_popup_menu(glib::clone!(@strong d2 => move |x| {
            let date = bin::get_date_grid(x);
            d2.borrow_mut().set_value(date, 1);
            false
        }));
        let u = d2.borrow().unknown.clone();
        u.connect_popup_menu(glib::clone!(@strong d2 => move |_| {
            // Mark month in calender.
            d2.borrow_mut().on_refresh();
            false
        }));
        g.hide();
        // Show doch nicht beim Aufrufer:
        g.show();
        d2
    }

    pub fn set_accel(&mut self, yesterday: &str, tomorrow: &str, label: Option<&gtk::Label>) {
        if !yesterday.is_empty() {
            self.yesterday
                .set_label(format!("-_{0}", yesterday).as_str());
        }
        if !tomorrow.is_empty() {
            self.tomorrow.set_label(format!("+_{0}", tomorrow).as_str());
        }
        if let Some(lbl) = label {
            lbl.set_mnemonic_widget(Some(&self.calendar));
        }
    }

    fn on_refresh(&mut self) {
        if let Some(month) = bin::get_month_grid(&self.grid) {
            bin::set_month_grid(&self.grid, &None, false);
            self.mark_month(&month);
        }
    }

    fn on_show(&mut self) {
        // println!("show");
        if self.nullable {
            self.unknown.show();
        } else {
            self.unknown.hide();
        }
        if self.with_calendar {
            self.down.show();
        } else {
            self.down.hide();
            self.open = false;
        }
        if self.open {
            self.calendar.show();
        } else {
            self.calendar.hide();
        }
    }

    /// Handle date key.
    fn on_date_key(&mut self) -> DateEvent {
        let text = bin::get_text_entry(&self.date);
        if let (10, Some(d)) = (text.len(), functions::ostr_to_ond(Some(text.as_str()))) {
            // println!("{} {}", text, d);
            return self.set_value(Some(d), 2);
        }
        DateEvent::Unchanged
    }

    fn on_down(&mut self) {
        // println!("down");
        if self.with_calendar {
            self.open = !self.open;
            // Darf keinen Show-Event auslösen!
            // self.grid.hide();
            // self.grid.show_all();
            if self.open {
                self.calendar.show();
            } else {
                self.calendar.hide();
            }
        }
    }

    /// Get date value.
    pub fn _get_value(&self) -> Option<NaiveDate> {
        if self.unknown.is_active() {
            return None;
        }
        self.value
    }

    /// Get not none date value.
    pub fn _get_value_nn(&self) -> NaiveDate {
        if let Some(nd) = self._get_value() {
            return nd;
        }
        services::get_daten().get_today()
    }

    /// Set date value.
    /// * ond: Affected date.
    /// * set_in_grid: 0: all 1: not grid 2: not date 3: not calendar
    /// * returns: DateEvent
    pub fn set_value(&mut self, ond: Option<NaiveDate>, set_in_grid: i32) -> DateEvent {
        // println!("set_value {:?}", ond);
        let de = config::get_config().is_de();
        let mut has_value = false;
        let mut value = NaiveDate::from_ymd(1, 1, 1);
        if let Some(v) = ond {
            has_value = true;
            value = v;
            self.date.set_editable(true);
            self.daytext.set_text(functions::ond_to_weekday(&ond, de));
        } else {
            self.date.set_editable(false);
            self.daytext.set_text("");
        }
        let mut old_value = NaiveDate::from_ymd(1, 1, 1);
        if let Some(v) = self.value {
            old_value = v;
        }
        let mut datechanged = false;
        let mut monthchanged = false;
        self.unknown.set_active(!has_value);
        if self.value.is_none() == has_value || value != old_value {
            datechanged = true;
            if has_value {
                unsafe {
                    self.calendar.set_data::<&str>("recursion", "1");
                }
                self.calendar.set_day(1); // Falschen Tag vermeiden.
                self.calendar.set_year(value.year());
                self.calendar.set_month(value.month0() as i32);
                self.calendar.set_day(value.day() as i32);
                unsafe {
                    self.calendar.set_data::<&str>("recursion", "0");
                }
            }
            monthchanged = self.value.is_none() == has_value
                || self.value.is_none()
                || old_value.month() != value.month()
                || old_value.year() != value.year();
            self.value = ond;
        }
        if set_in_grid != 2 {
            self.date
                .set_text(functions::ond_to_str(&self.value).as_str());
        }
        if set_in_grid != 1 {
            bin::set_date_grid(&self.grid, &self.value, false);
        }
        if monthchanged {
            return DateEvent::Month {
                name: self.name.clone(),
                date: self.value,
            };
        } else if datechanged {
            return DateEvent::Date {
                name: self.name.clone(),
                date: self.value,
            };
        }
        DateEvent::Unchanged
    }

    /// Mark month.
    fn mark_month(&self, marks: &Vec<bool>) {
        for i in 0..31 {
            let mut b = false;
            if let Some(d) = marks.get(i) {
                b = *d;
            }
            if b {
                self.calendar.mark_day(i as u32 + 1);
            } else {
                self.calendar.unmark_day(i as u32 + 1);
            }
        }
    }

    fn on_yesterday(&mut self) -> DateEvent {
        // println!("yesterday");
        if let Some(d) = functions::to_ond(self.date.text().as_str()) {
            return self.set_value(functions::nd_add_dmy(&d, -1, 0, 0), 0);
        }
        DateEvent::Unchanged
    }

    fn on_today(&mut self) -> DateEvent {
        // println!("today");
        return self.set_value(Some(services::get_daten().get_today()), 0);
    }

    fn on_tomorrow(&mut self) -> DateEvent {
        // println!("tomorrow");
        if let Some(d) = functions::to_ond(self.date.text().as_str()) {
            return self.set_value(functions::nd_add_dmy(&d, 1, 0, 0), 0);
        }
        DateEvent::Unchanged
    }

    // Handle changed calendar.
    fn on_calendar(&mut self) -> DateEvent {
        // println!("calendar {:?}", self.calendar.date());
        let (y, m, d) = self.calendar.date();
        return self.set_value(Some(NaiveDate::from_ymd(y as i32, m + 1, d)), 3);
    }

    /// Set focus to calendar.
    pub fn _grab_focus(&self) {
        self.calendar.grab_focus();
    }
}
