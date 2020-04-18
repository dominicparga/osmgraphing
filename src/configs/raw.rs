use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub parsing: parsing::Config,
    pub writing: Option<writing::Config>,
    pub routing: Option<routing::Config>,
}

pub mod parsing {
    use serde::Deserialize;
    use std::path::PathBuf;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Config {
        pub map_file: PathBuf,
        pub vehicles: vehicles::Config,
        pub nodes: nodes::Config,
        pub edges: edges::Config,
    }

    pub mod vehicles {
        use crate::network::vehicles::Category as VehicleCategory;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub struct Config {
            pub category: VehicleCategory,
            pub are_drivers_picky: bool,
        }
    }

    pub mod nodes {
        use crate::configs::SimpleId;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct Config(pub Vec<Category>);

        #[derive(Clone, Debug, Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum Category {
            Meta { info: MetaInfo, id: SimpleId },
            Metric { unit: UnitInfo, id: SimpleId },
            Ignored,
        }

        #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
        pub enum MetaInfo {
            NodeId,
            Level,
        }

        #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
        pub enum UnitInfo {
            Latitude,
            Longitude,
        }
    }

    pub mod edges {
        use crate::configs::SimpleId;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct Config(pub Vec<Category>);

        #[derive(Clone, Debug, Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum Category {
            Meta { info: MetaInfo, id: SimpleId },
            Metric { unit: UnitInfo, id: SimpleId },
            Ignored,
        }

        #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
        pub enum MetaInfo {
            SrcId,
            DstId,
            ShortcutEdgeIdx0,
            ShortcutEdgeIdx1,
        }

        #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
        pub enum UnitInfo {
            Meters,
            Kilometers,
            Seconds,
            Minutes,
            Hours,
            KilometersPerHour,
            LaneCount,
            F64,
        }
    }
}

pub mod writing {
    use serde::Deserialize;
    use std::path::PathBuf;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Config {
        pub map_file: PathBuf,
        #[serde(flatten)]
        pub nodes: nodes::Config,
        #[serde(flatten)]
        pub edges: edges::Config,
    }

    pub mod nodes {
        use crate::configs::SimpleId;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct Config {
            #[serde(rename = "nodes")]
            pub categories: Vec<Category>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum Category {
            Id(SimpleId),
            Ignored,
        }
    }

    pub mod edges {
        use crate::configs::SimpleId;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct Config {
            #[serde(rename = "edges")]
            pub categories: Vec<Category>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum Category {
            Id(SimpleId),
            Ignored,
        }
    }
}

pub mod routing {
    use crate::configs::SimpleId;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Config {
        pub is_ch_dijkstra: Option<bool>,
        pub metrics: Vec<Entry>,
    }

    impl Config {
        pub fn from_str(yaml_str: &str) -> Result<Config, String> {
            #[derive(Deserialize)]
            struct Wrapper {
                routing: Config,
            }

            let wrapper: Result<Wrapper, _> = serde_yaml::from_str(yaml_str);
            match wrapper {
                Ok(wrapper) => Ok(wrapper.routing),
                Err(e) => Err(format!("{}", e)),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Entry {
        pub id: SimpleId,
        pub alpha: Option<f64>,
    }
}
