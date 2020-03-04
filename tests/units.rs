use osmgraphing::{
    helpers::Approx,
    units::{length::Meters, speed::KilometersPerHour, time::Seconds},
};

pub fn length() {
    // meters
    assert!(
        (Meters(1.0) / Seconds(1.0)).approx_eq(&Seconds(3.6)),
        "kmph = m / s is wrong"
    );
    assert!(
        (Meters(1.0) / KilometersPerHour(1.0)).approx_eq(&Seconds(3.6)),
        "s = m / kmph is wrong"
    );

    // kilometers
    assert!(
        (Meters(1.0) / Seconds(1.0)).approx_eq(&Seconds(3.6)),
        "kmph = m / s is wrong"
    );
    assert!(
        (Meters(1.0) / KilometersPerHour(1.0)).approx_eq(&Seconds(3.6)),
        "s = m / kmph is wrong"
    );
}

pub fn duration() {
    // seconds
    assert!(
        (Seconds(3.6) * KilometersPerHour(1.0)).approx_eq(&Meters(1.0)),
        "m = s * kmph is wrong"
    );
}

pub fn speed() {
    // kmph
    assert!(
        (KilometersPerHour(3.6) * Seconds(1.0)).approx_eq(&Meters(1.0)),
        "m = kmph * s is wrong"
    );
}
