/// Consider a path from left to right.
/// It is important to have the smaller hop-distance at the bottom-path,
/// but the smaller weight-distance at the top-path.
#[cfg(not(feature = "custom_only"))]
mod bidirectional_bait;
mod helpers;
