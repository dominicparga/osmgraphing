mod units {
    use osmgraphing::{
        helpers::ApproxEq,
        units::{length::Kilometers, speed::KilometersPerHour, time::Seconds},
    };

    #[test]
    pub fn length() {
        let length = Kilometers(1.0);
        let duration = Seconds(1_000.0);
        let speed = KilometersPerHour(3.6);

        assert!(
            (length / duration).approx_eq(&speed),
            format!("{} = {} / {} is wrong", speed, length, duration)
        );
        assert!(
            (length / speed).approx_eq(&duration),
            format!("{} = {} / {} is wrong", duration, length, speed)
        );
    }

    #[test]
    pub fn duration() {
        let length = Kilometers(1.0);
        let duration = Seconds(1_000.0);
        let speed = KilometersPerHour(3.6);

        assert!(
            (duration * speed).approx_eq(&length),
            format!("{} = {} * {} is wrong", length, duration, speed)
        );
    }

    #[test]
    pub fn speed() {
        let length = Kilometers(1.0);
        let duration = Seconds(1_000.0);
        let speed = KilometersPerHour(3.6);

        assert!(
            (speed * duration).approx_eq(&length),
            format!("{} = {} * {} is wrong", length, speed, duration)
        );
    }
}
