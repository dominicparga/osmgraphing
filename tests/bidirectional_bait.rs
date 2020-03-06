mod helpers;

mod maps {
    /// Consider a path from left to right.
    /// It is important to have the smaller hop-distance at the bottom-path,
    /// but the smaller weight-distance at the top-path.
    mod bidirectional_bait {
        mod parsing;
        mod routing {
            mod fastest;
            mod shortest;
        }
    }
}
