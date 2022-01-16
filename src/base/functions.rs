use crate::res::messages::M;
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, Weekday};
use lazy_static::lazy_static;
use rand::Rng;
use regex::{Regex, RegexBuilder};
use std::borrow::Cow;
use thousands::Separable;

/// Die Funktion macht nichts und liefert immer 0.
pub fn mach_nichts() -> i32 {
    0
}

/// Wandelt einen String in einen Integer um.
/// * s: Zu konvertierender String.
pub fn to_i32(s: &str) -> i32 {
    let x = s.parse::<i32>();
    if let Ok(i) = x {
        return i;
    }
    0
}

/// Wandelt einen String in einen Float um.
/// * s: Zu konvertierender String.
pub fn to_f32(s: &str) -> f32 {
    let x = s.parse::<f32>();
    if let Ok(i) = x {
        return i;
    }
    0_f32
}

/// Wandelt einen String in einen Float um.
/// * s: Zu konvertierender String.
/// * is_de: German format?
pub fn to_f64(s: &str, is_de: bool) -> f64 {
    let mut s1 = s.to_string();
    if is_de {
        s1 = switch_en_de(&s1);
    }
    let x = s1.parse::<f64>();
    if let Ok(i) = x {
        return i;
    }
    0_f64
}

/// Convert string to bool.
/// * s: Affected string.
pub fn to_bool(s: &str) -> bool {
    if let Ok(x) = s.parse::<bool>() {
        return x;
    }
    false
}

/// Convert bool to string.
/// * v: Affected bool.
pub fn bool_to_str(v: bool) -> String {
    v.to_string()
}

/// Wandelt einen Integer in einen String um.
/// * i: Zu konvertierender Integer.
pub fn to_cowstr<'a>(i: i32) -> Cow<'a, str> {
    let s = i.to_string();
    Cow::Owned(s)
}

/// Convert integer to string.
/// * i: Affected integer.
pub fn to_str<'a>(i: i32) -> String {
    let s = i.to_string();
    s
}

/// Convert float to string.
/// * i: Affected float.
pub fn f32_to_str<'a>(i: f32) -> String {
    let s = i.to_string();
    s
}

/// Convert float to string.
/// * i: Affected float.
pub fn f64_to_str<'a>(i: f64) -> String {
    let s = i.to_string();
    s
}

/// Convert float to string with 2 digits.
/// * f: Affected float.
/// * is_de: German format?
/// return: Formatted amount.
pub fn f64_to_str_2<'a>(f: &f64, is_de: bool) -> String {
    let s = ((f * 100.0).round() / 100.0).separate_with_commas();
    if is_de {
        return switch_en_de(&s);
    }
    s
}

/// Convert float to optional string with 2 digits.
/// * f: Affected float.
/// * is_de: German format?
/// return: Formatted amount.
pub fn f64_to_ostr_2<'a>(f: &f64, is_de: bool) -> Option<String> {
    if *f == 0.0 {
        return None;
    }
    let s = ((f * 100.0).round() / 100.0).separate_with_commas();
    if is_de {
        return Some(switch_en_de(&s));
    }
    Some(s)
}

/// Convert float to string with 4 digits.
/// * f: Affected float.
/// * is_de: German format?
/// return: Formatted amount.
pub fn f64_to_str_4<'a>(f: &f64, is_de: bool) -> String {
    let s = ((f * 10000.0).round() / 10000.0).separate_with_commas();
    if is_de {
        return switch_en_de(&s);
    }
    // let s = format!("{:.4}", i);
    s
}

/// Convert float to string with 5 digits.
/// * f: Affected float.
/// * is_de: German format?
/// return: Formatted amount.
pub fn f64_to_str_5<'a>(f: &f64, is_de: bool) -> String {
    let s = ((f * 100000.0).round() / 100000.0).separate_with_commas();
    if is_de {
        return switch_en_de(&s);
    }
    s
}

/// Convert English formatted amount to German format or vice versa.
/// * s: Affected formatted amount.
/// return: Converted formatted amount.
pub fn switch_en_de(s: &String) -> String {
    let mut sd = String::new();
    for c in s.chars() {
        if c == ',' {
            sd.push('.');
        } else if c == '.' {
            sd.push(',');
        } else {
            sd.push(c);
        }
    }
    return sd;
}

/// ===================== String =====================

/// Wandelt optionalen String in String.
/// * s: Betroffener String.
pub fn ostr_to_str(s: &Option<String>) -> String {
    if let Some(i) = s {
        return i.clone();
    }
    "".into()
}

/// Ist der String None oder leer.
/// * s: Betroffener String.
pub fn is_empty(s: &Option<String>) -> bool {
    if let Some(i) = s {
        return i.is_empty();
    }
    true
}

/// Vergleich zweier Strings ohne Unterscheidung von Groß-/Kleinschreibung.
/// * s: 1. String.
/// * s2: 2. String.
pub fn cmp(s: &str, s2: &str) -> bool {
    let l1 = s.to_lowercase();
    let l2 = s2.to_lowercase();
    l1 == l2
}

/// Vergleich zweier Strings ohne Unterscheidung von Groß-/Kleinschreibung, None=Leer.
/// * s: 1. String.
/// * s2: 2. String.
pub fn cmpo(s: &Option<String>, s2: &str) -> bool {
    let l1 = match s {
        Some(s1) => s1.clone(),
        _ => "".into(),
    }
    .to_lowercase();
    let l2 = s2.to_lowercase();
    l1 == l2
}

// /// Abschneiden der ersten 5 Zeichen, falls möglich.
// fn m0(s: &str, cut: bool) -> Option<&str> {
//     if !cut || s.is_empty() || s.len() < 5 {
//         Some(s)
//     } else {
//         Some(&s[5..])
//     }
// }

/// Abschneiden der ersten 5 Zeichen, falls möglich.
pub fn m5(s: &str, cut: bool) -> &str {
    if !cut || s.is_empty() || s.len() < 5 {
        s
    } else {
        &s[5..]
    }
}

/// Erstes Zeichen groß.
pub fn to_first_upper(s: &str) -> String {
    if s.is_empty() {
        s.to_string()
    } else {
        let mut x1 = s.get(..1).unwrap_or_default().to_uppercase();
        x1.push_str(s.get(1..).unwrap_or_default());
        x1
    }
}

/// Must String be taken for a compare?
pub fn is_like(s: &str) -> bool {
    !(s.is_empty() || s == "%" || s == "%%")
}

/// ===================== Date =====================

/// Convert optional date to string.
/// * ond: Affected date.
pub fn ond_to_str(ond: &Option<NaiveDate>) -> String {
    if let Some(d) = ond {
        return d.format("%Y-%m-%d").to_string();
    }
    "".into()
}

/// Get year of optional date.
/// * ond: Affected date.
pub fn ond_year(ond: &Option<NaiveDate>) -> i32 {
    if let Some(d) = ond {
        return d.year();
    }
    0
}

/// Get year of optional datetime.
/// * ond: Affected date.
pub fn ondt_year(ond: &Option<NaiveDateTime>) -> i32 {
    if let Some(d) = ond {
        return d.year();
    }
    0
}

/// Wandelt optionales Datum in einen String mit Wochentag um.
/// * ond: Zu konvertierendes Datum.
pub fn ond_to_weekday(ond: &Option<NaiveDate>, is_de: bool) -> &str {
    if let Some(d) = ond {
        if is_de {
            return match d.weekday() {
                Weekday::Mon => "Montag",
                Weekday::Tue => "Dienstag",
                Weekday::Wed => "Mittwoch",
                Weekday::Thu => "Donnerstag",
                Weekday::Fri => "Freitag",
                Weekday::Sat => "Samstag",
                Weekday::Sun => "Sonntag",
            };
        }
        return match d.weekday() {
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday",
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturday",
            Weekday::Sun => "Sunday",
        };
    }
    ""
}

/// Wandelt einen String in optionales Datum um.
/// * s: Zu konvertierender String.
pub fn to_ond(s: &str) -> Option<NaiveDate> {
    if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(d);
    }
    None
}

/// Wandelt einen optionalen String in optionales Datum um.
/// * s: Zu konvertierender String.
pub fn ostr_to_ond(s: Option<&str>) -> Option<NaiveDate> {
    if let Some(str) = s {
        if let Ok(d) = NaiveDate::parse_from_str(str, "%Y-%m-%d") {
            return Some(d);
        }
    }
    None
}

/// Wandelt einen optionalen String in optionales Datum mit Uhrzeit um.
/// * s: Zu konvertierender String.
pub fn ostr_to_ondt(s: Option<&str>) -> Option<NaiveDateTime> {
    if let Some(str) = s {
        if let Ok(d) = NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M:%S") {
            return Some(d);
        }
    }
    None
}

/// Add days to optional date.
/// * ond: Affected date.
pub fn ond_add_days(ond: &Option<NaiveDate>, days: i32) -> Option<NaiveDate> {
    if let Some(d) = ond {
        let d2 = NaiveDate::from_num_days_from_ce_opt(d.num_days_from_ce() + days);
        return d2;
    }
    None
}

/// Get last day of month.
fn last_day_of_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1))
        .pred()
        .day()
}

/// Add days, month and years to date.
/// * nd: Affected date.
/// * returns: Added date.
pub fn nd_add_dmy(nd: &NaiveDate, days: i32, months: i32, years: i32) -> Option<NaiveDate> {
    if let Some(d2) = NaiveDate::from_num_days_from_ce_opt(nd.num_days_from_ce() + days) {
        let mut d = d2.day();
        let mut m = d2.month() as i32 + months;
        let mut y = d2.year() + years;
        while m > 12 {
            m -= 12;
            y += 1;
        }
        while m < 1 {
            m += 12;
            y -= 1;
        }
        let ml = last_day_of_month(y, m as u32);
        if d > ml {
            m += 1;
            if m > 12 {
                y += 1;
            }
            d -= ml;
        }
        let d3 = NaiveDate::from_ymd_opt(y, m as u32, d as u32);
        return d3;
    }
    None
}

/// Get minimum of two dates.
pub fn min_date(d1: &NaiveDate, d2: &NaiveDate) -> NaiveDate {
    let m = match d1 < d2 {
        true => d1,
        _ => d2,
    };
    m.clone()
}

/// Get maximum of two dates.
pub fn max_date(d1: &NaiveDate, d2: &NaiveDate) -> NaiveDate {
    let m = match d1 > d2 {
        true => d1,
        _ => d2,
    };
    m.clone()
}

/// Convert optional date to string.
/// * ondt: Affected date.
pub fn ondt_to_str<'a>(ondt: &Option<NaiveDateTime>) -> String {
    if let Some(d) = ondt {
        return d.format("%Y-%m-%d %H:%M:%S").to_string();
    }
    "".into()
}

/// Return a GUID.
pub fn get_uid() -> String {
    let guid = uuid::Uuid::new_v4();
    guid.to_string()
    //format!("{}", guid)
}

/// Wandelt optionales Datum und optionale Benutzer-ID in String.
/// * date: Optionales Datum.
/// * s: Optionale Benutzer-ID.
/// * returns: Zusammengesetzter String.
pub fn format_date_of(date: &Option<NaiveDateTime>, id: &Option<String>, is_de: bool) -> String {
    if let (Some(d), Some(b)) = (date, id) {
        return M::m1011(d, b.as_str(), is_de);
    }
    "".into()
}

/// Liefert String in Abhängigkeit eines Boolean-Wertes.
pub fn iif<'a>(b: bool, strue: &'a str, sfalse: &'a str) -> &'a str {
    if b {
        return strue;
    }
    sfalse
}

/// Liefert i32 in Abhängigkeit eines Boolean-Wertes.
pub fn iif_i32(b: bool, itrue: i32, ifalse: i32) -> i32 {
    if b {
        return itrue;
    }
    ifalse
}

/// Liefert i64 in Abhängigkeit eines Boolean-Wertes.
pub fn iif_i64(b: bool, itrue: i64, ifalse: i64) -> i64 {
    if b {
        return itrue;
    }
    ifalse
}

/// Convert string with graphhopper.com coordinates to tuple
pub fn to_coordinates(s: &str) -> Option<(f64, f64, f64)> {
    lazy_static! {
        static ref RE_COORD: Regex =
            RegexBuilder::new(r#"^(-?\d+(\.\d+)),\s*(-?\d+(\.\d+))(,\s*(-?\d+(\.\d+))z?)?$"#)
                .case_insensitive(true)
                .build()
                .unwrap();
    }
    if let Some(c) = RE_COORD.captures(s) {
        return Some((
            to_f64(&c[1], false),
            to_f64(&c[3], false),
            to_f64(&c.get(6).map_or("", |a| a.as_str()), false),
        ));
    }
    None
}

/// Get a file name with optional date and random number.
pub fn get_file_name(name: &str, date: bool, random: bool, ext: &str) -> String {
    let mut s = String::new();
    s.push_str(name);
    if date {
        let now: DateTime<Local> = Local::now();
        s.push('_');
        s.push_str(now.format("%Y%m%d").to_string().as_str());
    }
    if random {
        let mut rng = rand::thread_rng();
        let n: u32 = rng.gen_range(1000..10000);
        s.push('_');
        s.push_str(n.to_string().as_str());
    }
    if !ext.is_empty() {
        s.push('.');
        s.push_str(ext);
    }
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_uid() {
        assert_eq!(36, super::get_uid().len());
    }
    #[test]
    fn mach_nichts() {
        assert_eq!(0, super::mach_nichts());
    }
    #[test]
    fn to_i32() {
        assert_eq!(0, super::to_i32(""));
        assert_eq!(0, super::to_i32("x"));
        assert_eq!(1, super::to_i32("1"));
        assert_eq!(-1, super::to_i32("-1"));
    }

    #[test]
    fn to_f32() {
        assert_eq!(0_f32, super::to_f32(""));
        assert_eq!(0_f32, super::to_f32("x"));
        assert_eq!(1_f32, super::to_f32("1"));
        assert_eq!(-1_f32, super::to_f32("-1"));
        assert_eq!(1.1_f32, super::to_f32("1.1"));
        assert_eq!(1.01_f32, super::to_f32("1.01"));
    }

    // #[test]
    // fn m0() {
    //     assert_eq!(Some(""), super::m0("", true));
    //     assert_eq!(Some("1"), super::m0("1", true));
    //     assert_eq!(Some("12"), super::m0("12", true));
    //     assert_eq!(Some("123"), super::m0("123", true));
    //     assert_eq!(Some("1234"), super::m0("1234", true));
    //     assert_eq!(Some(""), super::m0("12345", true));
    //     assert_eq!(Some("6"), super::m0("123456", true));
    //     assert_eq!(Some("67"), super::m0("1234567", true));
    //     assert_eq!(Some(""), super::m0("", false));
    //     assert_eq!(Some("1"), super::m0("1", false));
    //     assert_eq!(Some("12"), super::m0("12", false));
    //     assert_eq!(Some("123"), super::m0("123", false));
    //     assert_eq!(Some("1234"), super::m0("1234", false));
    //     assert_eq!(Some("12345"), super::m0("12345", false));
    //     assert_eq!(Some("123456"), super::m0("123456", false));
    //     assert_eq!(Some("1234567"), super::m0("1234567", false));
    // }

    #[test]
    fn m5() {
        assert_eq!("", super::m5("", true));
        assert_eq!("1", super::m5("1", true));
        assert_eq!("12", super::m5("12", true));
        assert_eq!("123", super::m5("123", true));
        assert_eq!("1234", super::m5("1234", true));
        assert_eq!("", super::m5("12345", true));
        assert_eq!("6", super::m5("123456", true));
        assert_eq!("67", super::m5("1234567", true));
        assert_eq!("", super::m5("", false));
        assert_eq!("1", super::m5("1", false));
        assert_eq!("12", super::m5("12", false));
        assert_eq!("123", super::m5("123", false));
        assert_eq!("1234", super::m5("1234", false));
        assert_eq!("12345", super::m5("12345", false));
        assert_eq!("123456", super::m5("123456", false));
        assert_eq!("1234567", super::m5("1234567", false));
    }
}
