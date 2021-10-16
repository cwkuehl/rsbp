use diesel::table;

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    AD_ADRESSE (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        staat -> Nullable<Text>,
        plz -> Nullable<Text>,
        ort -> Text,
        strasse -> Nullable<Text>,
        hausnr -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    AD_PERSON (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        typ -> Integer,
        geschlecht -> Text,
        geburt -> Nullable<Date>,
        geburtk -> Integer,
        anrede -> Integer,
        fanrede -> Integer,
        name1 -> Text,
        name2 -> Nullable<Text>,
        praedikat -> Nullable<Text>,
        vorname -> Nullable<Text>,
        titel -> Nullable<Text>,
        person_status -> Integer,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    AD_SITZ (mandant_nr, person_uid, reihenfolge, uid) {
        mandant_nr -> Integer,
        person_uid -> Text,
        reihenfolge -> Integer,
        uid -> Text,
        typ -> Integer,
        name -> Text,
        adresse_uid -> Nullable<Text>,
        telefon -> Nullable<Text>,
        fax -> Nullable<Text>,
        mobil -> Nullable<Text>,
        email -> Nullable<Text>,
        homepage -> Nullable<Text>,
        postfach -> Nullable<Text>,
        bemerkung -> Nullable<Text>,
        sitz_status -> Integer,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    BENUTZER (mandant_nr, benutzer_id) {
        mandant_nr -> Integer,
        benutzer_id -> Text,
        passwort -> Nullable<Text>,
        berechtigung -> Integer,
        akt_periode -> Integer,
        person_nr -> Integer,
        geburt -> Nullable<Date>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    BYTE_DATEN (mandant_nr, typ, uid, lfd_nr) {
        mandant_nr -> Integer,
        typ -> Text,
        uid -> Text,
        lfd_nr -> Integer,
        metadaten -> Nullable<Text>,
        bytes -> Nullable<Binary>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    FZ_BUCH (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        autor_uid -> Text,
        serie_uid -> Text,
        seriennummer -> Integer,
        titel -> Text,
        untertitel -> Nullable<Text>,
        seiten -> Integer,
        sprache_nr -> Integer,
        notiz -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    FZ_BUCHAUTOR (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        name -> Text,
        vorname -> Nullable<Text>,
        notiz -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    FZ_BUCHSERIE (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        name -> Text,
        notiz -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    FZ_BUCHSTATUS (mandant_nr, buch_uid) {
        mandant_nr -> Integer,
        buch_uid -> Text,
        ist_besitz -> Bool,
        lesedatum -> Nullable<Date>,
        hoerdatum -> Nullable<Date>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
        replikation_uid -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    FZ_FAHRRAD (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        bezeichnung -> Text,
        typ -> Integer,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    FZ_FAHRRADSTAND (mandant_nr, fahrrad_uid, datum, nr) {
        mandant_nr -> Integer,
        fahrrad_uid -> Text,
        datum -> Date,
        nr -> Integer,
        zaehler_km -> Double,
        periode_km -> Double,
        periode_schnitt -> Double,
        beschreibung -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
        replikation_uid -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    FZ_NOTIZ (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        thema -> Text,
        notiz -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    HH_BILANZ (mandant_nr, periode, kz, konto_uid) {
        mandant_nr -> Integer,
        periode -> Integer,
        kz -> Text,
        konto_uid -> Text,
        sh -> Text,
        betrag -> Double,
        esh -> Text,
        ebetrag -> Double,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    HH_BUCHUNG (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        soll_valuta -> Date,
        haben_valuta -> Date,
        kz -> Nullable<Text>,
        betrag -> Double,
        ebetrag -> Double,
        soll_konto_uid -> Text,
        haben_konto_uid -> Text,
        btext -> Text,
        beleg_nr -> Nullable<Text>,
        beleg_datum -> Date,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    HH_EREIGNIS (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        kz -> Nullable<Text>,
        soll_konto_uid -> Text,
        haben_konto_uid -> Text,
        bezeichnung -> Text,
        etext -> Text,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    HH_KONTO (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        sortierung -> Text,
        art -> Text,
        kz -> Nullable<Text>,
        name -> Text,
        gueltig_von -> Nullable<Date>,
        gueltig_bis -> Nullable<Date>,
        periode_von -> Integer,
        periode_bis -> Integer,
        betrag -> Double,
        ebetrag -> Double,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    HH_PERIODE (mandant_nr, nr) {
        mandant_nr -> Integer,
        nr -> Integer,
        datum_von -> Date,
        datum_bis -> Date,
        art -> Integer,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    MA_MANDANT (nr) {
        nr -> Integer,
        beschreibung -> Text,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    MA_PARAMETER (mandant_nr, schluessel) {
        mandant_nr -> Integer,
        schluessel -> Text,
        wert -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
        replikation_uid -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    SB_EREIGNIS (mandant_nr, person_uid, familie_uid, typ) {
        mandant_nr -> Integer,
        person_uid -> Text,
        familie_uid -> Text,
        typ -> Text,
        tag1 -> Integer,
        monat1 -> Integer,
        jahr1 -> Integer,
        tag2 -> Integer,
        monat2 -> Integer,
        jahr2 -> Integer,
        datum_typ -> Text,
        ort -> Nullable<Text>,
        bemerkung -> Nullable<Text>,
        quelle_uid -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
        replikation_uid -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    SB_FAMILIE (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        mann_uid -> Nullable<Text>,
        frau_uid -> Nullable<Text>,
        status1 -> Integer,
        status2 -> Integer,
        status3 -> Integer,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    SB_KIND (mandant_nr, familie_uid, kind_uid) {
        mandant_nr -> Integer,
        familie_uid -> Text,
        kind_uid -> Text,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
        replikation_uid -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    SB_PERSON (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        name -> Text,
        vorname -> Nullable<Text>,
        geburtsname -> Nullable<Text>,
        geschlecht -> Nullable<Text>,
        titel -> Nullable<Text>,
        konfession -> Nullable<Text>,
        bemerkung -> Nullable<Text>,
        quelle_uid -> Nullable<Text>,
        status1 -> Integer,
        status2 -> Integer,
        status3 -> Integer,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    SB_QUELLE (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        beschreibung -> Text,
        zitat -> Nullable<Text>,
        bemerkung -> Nullable<Text>,
        autor -> Text,
        status1 -> Integer,
        status2 -> Integer,
        status3 -> Integer,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    SO_KURSE (datum) {
        datum -> Date,
        open -> Double,
        high -> Double,
        low -> Double,
        close -> Double,
        price -> Double,
        bewertung -> Text,
        trend -> Text,
        bemerkung -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    TB_EINTRAG (mandant_nr, datum) {
        mandant_nr -> Integer,
        datum -> Date,
        eintrag -> Text,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
        replikation_uid -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    TB_EINTRAG_ORT (mandant_nr, ort_uid, datum_von, datum_bis) {
        mandant_nr -> Integer,
        ort_uid -> Text,
        datum_von -> Date,
        datum_bis -> Date,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    TB_ORT (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        bezeichnung -> Text,
        breite -> Double,
        laenge -> Double,
        hoehe -> Double,
        notiz -> Text,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(TB_EINTRAG_ORT, TB_ORT);

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    WP_ANLAGE (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        wertpapier_uid -> Text,
        bezeichnung -> Text,
        parameter -> Nullable<Text>,
        notiz -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    WP_BUCHUNG (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        wertpapier_uid -> Text,
        anlage_uid -> Text,
        datum -> Date,
        zahlungsbetrag -> Double,
        rabattbetrag -> Double,
        anteile -> Double,
        zinsen -> Double,
        btext -> Text,
        notiz -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    WP_KONFIGURATION (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        bezeichnung -> Text,
        parameter -> Text,
        status -> Text,
        notiz -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    WP_STAND (mandant_nr, wertpapier_uid, datum) {
        mandant_nr -> Integer,
        wertpapier_uid -> Text,
        datum -> Date,
        stueckpreis -> Double,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    WP_WERTPAPIER (mandant_nr, uid) {
        mandant_nr -> Integer,
        uid -> Text,
        bezeichnung -> Text,
        kuerzel -> Text,
        parameter -> Nullable<Text>,
        datenquelle -> Text,
        status -> Text,
        relation_uid -> Nullable<Text>,
        notiz -> Nullable<Text>,
        angelegt_von -> Nullable<Text>,
        angelegt_am -> Nullable<Timestamp>,
        geaendert_von -> Nullable<Text>,
        geaendert_am -> Nullable<Timestamp>,
    }
}
