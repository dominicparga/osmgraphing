use crate::{helpers::ApproxEq, units::length::Kilometers};
use std::{fmt, fmt::Display};

/// Coordinate storing `lat`/`lon` as `i32` with accuracy `1e-7`.
#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}

impl Coordinate {
    pub fn zero() -> Coordinate {
        Coordinate { lat: 0.0, lon: 0.0 }
    }

    /// Attention with rouding errors, if important.
    /// It is very likely that it doesn't matter, since dµ-(lat, lon) is already precise for sub-meters.
    pub fn from_decimicro(decimicro_lat: i32, decimicro_lon: i32) -> Coordinate {
        Coordinate {
            lat: (decimicro_lat as f64 * 1e-7) as f64,
            lon: (decimicro_lon as f64 * 1e-7) as f64,
        }
    }
}

impl Eq for Coordinate {}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.lat.approx_eq(&other.lat) && self.lon.approx_eq(&other.lon)
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(lat: {}, lon: {})", self.lat, self.lon)
    }
}

/// The haversince-distance is the distance (e.g. in meters) between two points on a sphere (given in latitude and longitude).
///
/// The earth-radius is taken as `6371 km` since
///
/// 1. the resuluting sphere has same volume as the earth-ellipsoid, and
/// 1. it is the average radius.
///
/// Note that the result could have rounding errors due to cutting (f64 -> u32) and up-scaling (* 1000.0) afterwards.
///
/// ## Additional info
///
/// - [detailled information](http://www.movable-type.co.uk/scripts/latlong.html)
/// - [cpp](https://geographiclib.sourceforge.io/)
pub fn haversine_distance_km(from: &Coordinate, to: &Coordinate) -> Kilometers {
    let earth_mean_radius = 6_371.0; // kilometers

    let from_lat = from.lat as f64;
    let from_lon = from.lon as f64;
    let to_lat = to.lat as f64;
    let to_lon = to.lon as f64;

    let delta_lat = (from_lat - to_lat).to_radians();
    let delta_lon = (from_lon - to_lon).to_radians();

    let from_lat_rad = from_lat.to_radians();
    let to_lat_rad = to_lat.to_radians();

    let sin_lat = (delta_lat / 2.0).sin();
    let sin_lon = (delta_lon / 2.0).sin();

    Kilometers(
        (sin_lat * sin_lat + from_lat_rad.cos() * to_lat_rad.cos() * sin_lon * sin_lon)
            .sqrt()
            .asin()
            * (2.0 * earth_mean_radius),
    )
}
