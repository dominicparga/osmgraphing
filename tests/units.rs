mod units {
    use osmgraphing::units::{length::Kilometers, speed::KilometersPerHour, time::Seconds};
    // correct defaults for calculations
    const DURATION: Seconds = Seconds(1_000.0);
    const LENGTH: Kilometers = Kilometers(1.0);
    const SPEED: KilometersPerHour = KilometersPerHour(3.6);

    mod length {
        use osmgraphing::{
            helpers::ApproxEq,
            units::{
                length::{Kilometers, Meters},
                speed::KilometersPerHour,
                time::{Hours, Minutes, Seconds},
            },
        };

        #[test]
        fn m_to_km() {
            let from = Meters(1_000.0);
            let to = Kilometers::from(from);
            let raw_from = from.0;
            let raw_to = to.0;
            let scale = 1_000.0;
            assert!(raw_from.approx_eq(&(scale * raw_to)));
        }

        #[test]
        fn km_to_m() {
            let from = Kilometers(1_000.0);
            let to = Meters::from(from);
            let raw_from = from.0;
            let raw_to = to.0;
            let scale = 0.001;
            assert!(raw_from.approx_eq(&(scale * raw_to)));
        }

        #[test]
        fn s_mul_kmph() {
            let s = Seconds::from(super::DURATION);
            let kmph = KilometersPerHour::from(super::SPEED);

            let km = Kilometers::from(super::LENGTH);
            let result = s * kmph;

            assert!(
                result.approx_eq(&km),
                format!("{} != {} = {} * {}", result, km, s, kmph)
            );
        }

        #[test]
        fn min_mul_kmph() {
            let min = Minutes::from(super::DURATION);
            let kmph = KilometersPerHour::from(super::SPEED);

            let km = Kilometers::from(super::LENGTH);
            let result = min * kmph;

            assert!(
                result.approx_eq(&km),
                format!("{} != {} = {} * {}", result, km, min, kmph)
            );
        }

        #[test]
        fn h_mul_kmph() {
            let h = Hours::from(super::DURATION);
            let kmph = KilometersPerHour::from(super::SPEED);

            let km = Kilometers::from(super::LENGTH);
            let result = h * kmph;

            assert!(
                result.approx_eq(&km),
                format!("{} != {} = {} * {}", result, km, h, kmph)
            );
        }
    }

    mod time {
        use osmgraphing::{
            helpers::ApproxEq,
            units::{
                length::{Kilometers, Meters},
                speed::KilometersPerHour,
                time::{Hours, Minutes, Seconds},
            },
        };

        #[test]
        fn s_to_min() {
            let from = Seconds(1_000.0);
            let to = Minutes::from(from);
            let raw_from = from.0;
            let raw_to = to.0;
            let scale = 1.0 / 60.0;
            assert!((scale * raw_from).approx_eq(&raw_to));
        }

        #[test]
        fn s_to_h() {
            let from = Seconds(1_000.0);
            let to = Hours::from(from);
            let raw_from = from.0;
            let raw_to = to.0;
            let scale = 1.0 / 3_600.0;
            assert!((scale * raw_from).approx_eq(&raw_to));
        }

        #[test]
        fn min_to_s() {
            let from = Minutes(1_000.0);
            let to = Seconds::from(from);
            let raw_from = from.0;
            let raw_to = to.0;
            let scale = 60.0;
            assert!((scale * raw_from).approx_eq(&raw_to));
        }

        #[test]
        fn min_to_h() {
            let from = Minutes(1_000.0);
            let to = Hours::from(from);
            let raw_from = from.0;
            let raw_to = to.0;
            let scale = 1.0 / 60.0;
            assert!((scale * raw_from).approx_eq(&raw_to));
        }

        #[test]
        fn h_to_s() {
            let from = Hours(1_000.0);
            let to = Seconds::from(from);
            let raw_from = from.0;
            let raw_to = to.0;
            let scale = 3_600.0;
            assert!((scale * raw_from).approx_eq(&raw_to));
        }

        #[test]
        fn h_to_min() {
            let from = Hours(1_000.0);
            let to = Minutes::from(from);
            let raw_from = from.0;
            let raw_to = to.0;
            let scale = 60.0;
            assert!((scale * raw_from).approx_eq(&raw_to));
        }

        #[test]
        fn m_div_kmph() {
            let m = Meters::from(super::LENGTH);
            let kmph = KilometersPerHour::from(super::SPEED);

            let h = Hours::from(super::DURATION);
            let result = m / kmph;

            assert!(
                result.approx_eq(&h),
                format!("{} != {} = {} / {}", result, h, m, kmph)
            );
        }

        #[test]
        fn km_div_kmph() {
            let km = Kilometers::from(super::LENGTH);
            let kmph = KilometersPerHour::from(super::SPEED);

            let h = Hours::from(super::DURATION);
            let result = km / kmph;

            assert!(
                result.approx_eq(&h),
                format!("{} != {} = {} / {}", result, h, km, kmph)
            );
        }
    }

    mod speed {
        use osmgraphing::{
            helpers::ApproxEq,
            units::{
                length::{Kilometers, Meters},
                speed::KilometersPerHour,
                time::{Hours, Minutes, Seconds},
            },
        };

        #[test]
        fn m_div_s() {
            let m = Meters::from(super::LENGTH);
            let s = Seconds::from(super::DURATION);

            let kmph = KilometersPerHour::from(super::SPEED);
            let result = m / s;

            assert!(
                result.approx_eq(&kmph),
                format!("{} != {} = {} / {}", result, kmph, m, s)
            );
        }

        #[test]
        fn m_div_min() {
            let m = Kilometers::from(super::LENGTH);
            let min = Minutes::from(super::DURATION);

            let kmph = KilometersPerHour::from(super::SPEED);
            let result = m / min;

            assert!(
                result.approx_eq(&kmph),
                format!("{} != {} = {} / {}", result, kmph, m, min)
            );
        }

        #[test]
        fn m_div_h() {
            let m = Kilometers::from(super::LENGTH);
            let h = Hours::from(super::DURATION);

            let kmph = KilometersPerHour::from(super::SPEED);
            let result = m / h;

            assert!(
                result.approx_eq(&kmph),
                format!("{} != {} = {} / {}", result, kmph, m, h)
            );
        }

        #[test]
        fn km_div_s() {
            let km = Kilometers::from(super::LENGTH);
            let s = Seconds::from(super::DURATION);

            let kmph = KilometersPerHour::from(super::SPEED);
            let result = km / s;

            assert!(
                result.approx_eq(&kmph),
                format!("{} != {} = {} / {}", result, kmph, km, s)
            );
        }

        #[test]
        fn km_div_min() {
            let km = Kilometers::from(super::LENGTH);
            let min = Minutes::from(super::DURATION);

            let kmph = KilometersPerHour::from(super::SPEED);
            let result = km / min;

            assert!(
                result.approx_eq(&kmph),
                format!("{} != {} = {} / {}", result, kmph, km, min)
            );
        }

        #[test]
        fn km_div_h() {
            let km = Kilometers::from(super::LENGTH);
            let h = Hours::from(super::DURATION);

            let kmph = KilometersPerHour::from(super::SPEED);
            let result = km / h;

            assert!(
                result.approx_eq(&kmph),
                format!("{} != {} = {} / {}", result, kmph, km, h)
            );
        }
    }
}
