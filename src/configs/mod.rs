mod implementation;

pub use implementation::SimpleId;

pub mod parsing {
    pub use crate::configs::implementation::parsing::Config;

    pub mod nodes {
        pub use crate::configs::implementation::parsing::nodes::{Category, Config, MetaInfo};

        pub mod metrics {
            pub use crate::configs::implementation::parsing::nodes::metrics::UnitInfo;
        }
    }

    pub mod edges {
        pub use crate::configs::implementation::parsing::edges::{Category, Config, MetaInfo};

        pub mod metrics {
            pub use crate::configs::implementation::parsing::edges::metrics::UnitInfo;
        }
    }

    pub mod generating {
        pub use crate::configs::implementation::parsing::generating::Config;

        pub mod nodes {
            pub use crate::configs::implementation::parsing::generating::nodes::{
                Category, Config, MetaInfo,
            };

            pub mod metrics {
                pub use crate::configs::implementation::parsing::generating::nodes::metrics::UnitInfo;
            }
        }

        pub mod edges {
            pub use crate::configs::implementation::parsing::generating::edges::{
                Category, Config, MetaInfo,
            };

            pub mod metrics {
                pub use crate::configs::implementation::parsing::generating::edges::metrics::{
                    Category, UnitInfo,
                };
            }
        }
    }
}

pub mod writing {
    pub mod network {
        pub use crate::configs::implementation::writing::network::Config;
    }

    pub mod routing {
        pub use crate::configs::implementation::writing::routing::{Category, Config};
    }
}

pub mod routing {
    pub use crate::configs::implementation::routing::Config;
}

pub mod balancing {
    pub use crate::configs::implementation::balancing::{Config, Optimization};
}
