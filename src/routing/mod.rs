pub mod astar; // pub for astar::Path
pub use astar::{
    Astar,        // pub bc it's a trait
    GenericAstar, // pub for own cost-fn or estimation-fn
};

pub mod factory; // easy to use
