mod lib;

pub use lib::{Config, SimpleId};

pub mod parsing {
    pub use crate::configs::lib::parsing::Config;

    pub mod nodes {
        pub use crate::configs::lib::parsing::nodes::{Category, Config, MetaInfo};

        pub mod metrics {
            pub use crate::configs::lib::parsing::nodes::metrics::UnitInfo;
        }
    }

    pub mod edges {
        pub use crate::configs::lib::parsing::edges::{Category, Config, MetaInfo};

        pub mod metrics {
            pub use crate::configs::lib::parsing::edges::metrics::UnitInfo;
        }
    }

    pub mod generating {
        pub use crate::configs::lib::parsing::generating::Config;

        pub mod nodes {
            pub use crate::configs::lib::parsing::generating::nodes::{Category, Config, MetaInfo};

            pub mod metrics {
                pub use crate::configs::lib::parsing::generating::nodes::metrics::UnitInfo;
            }
        }

        pub mod edges {
            pub use crate::configs::lib::parsing::generating::edges::{Category, Config, MetaInfo};

            pub mod metrics {
                pub use crate::configs::lib::parsing::generating::edges::metrics::{
                    Category, UnitInfo,
                };
            }
        }
    }
}

pub mod writing {
    pub use crate::configs::lib::writing::Config;
}

pub mod routing {
    pub use crate::configs::lib::routing::Config;
}
