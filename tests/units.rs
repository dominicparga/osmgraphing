use osmgraphing::{
    helpers::Approx,
    units::{
        length::{Kilometers, Meters},
        speed::KilometersPerHour,
        time::Seconds,
    },
};

pub fn length() {
    let length = Meters(1_000.0);
    let duration = Seconds(1_000.0);
    let speed = KilometersPerHour(3.6);

    // meters
    assert!(
        (length / duration).approx_eq(&speed),
        format!("{} = {} / {} is wrong", speed, length, duration)
    );
    assert!(
        (length / speed).approx_eq(&duration),
        format!("{} = {} / {} is wrong", duration, length, speed)
    );

    // kilometers
    let length = Kilometers::from(length);
    assert!(
        (length / duration).approx_eq(&speed),
        format!("{} = {} / {} is wrong", speed, length, duration)
    );
    assert!(
        (length / speed).approx_eq(&duration),
        format!("{} = {} / {} is wrong", duration, length, speed)
    );
}

pub fn duration() {
    let length = Kilometers::from(Meters(1_000.0));
    let duration = Seconds(1_000.0);
    let speed = KilometersPerHour(3.6);

    assert!(
        (duration * speed).approx_eq(&length),
        format!("{} = {} * {} is wrong", length, duration, speed)
    );
}

pub fn speed() {
    let length = Kilometers::from(Meters(1_000.0));
    let duration = Seconds(1_000.0);
    let speed = KilometersPerHour(3.6);

    assert!(
        (speed * duration).approx_eq(&length),
        format!("{} = {} * {} is wrong", length, speed, duration)
    );
}
