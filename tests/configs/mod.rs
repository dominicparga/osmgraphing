pub mod fmi {
    // https://serde.rs/enum-representations.html
    // https://docs.rs/serde_yaml/0.8.11/serde_yaml/
    // https://stackoverflow.com/questions/53243795/how-do-you-read-a-yaml-file-in-rust

    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Point {
        x: f64,
        y: f64,
    }

    pub fn deserialize() -> Result<(), serde_yaml::Error> {
        let point = Point { x: 1.0, y: 2.0 };

        let s = serde_yaml::to_string(&point)?;
        assert_eq!(s, "---\nx: 1.0\ny: 2.0");

        let deserialized_point: Point = serde_yaml::from_str(&s)?;
        assert_eq!(point, deserialized_point);
        Ok(())
    }
}
