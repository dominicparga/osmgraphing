use crate::units::length::Meters;
use std::{cmp::Ordering, fmt, fmt::Display};

/// Coordinate storing `lat`/`lon` as `i32` with accuracy `1e-7`.
#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    decimicro_lat: i32,
    decimicro_lon: i32,
}

impl Default for Coordinate {
    fn default() -> Coordinate {
        Coordinate::zero()
    }
}

impl Coordinate {
    pub fn zero() -> Coordinate {
        (0, 0).into()
    }

    pub fn lat(&self) -> f64 {
        self.decimicro_lat as f64 * 1e-7
    }

    pub fn lon(&self) -> f64 {
        self.decimicro_lon as f64 * 1e-7
    }

    pub fn decimicro_lat(&self) -> i32 {
        self.decimicro_lat
    }

    pub fn decimicro_lon(&self) -> i32 {
        self.decimicro_lon
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from((decimicro_lat, decimicro_lon): (i32, i32)) -> Coordinate {
        Coordinate {
            decimicro_lat,
            decimicro_lon,
        }
    }
}

impl From<(f64, f64)> for Coordinate {
    fn from((lat, lon): (f64, f64)) -> Coordinate {
        Coordinate {
            decimicro_lat: (lat * 1e7) as i32,
            decimicro_lon: (lon * 1e7) as i32,
        }
    }
}

impl Eq for Coordinate {}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.decimicro_lat.cmp(&other.decimicro_lat) == Ordering::Equal
            && self.decimicro_lon.cmp(&other.decimicro_lon) == Ordering::Equal
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(dµ_lat: {}, dµ_lon: {})",
            self.decimicro_lat, self.decimicro_lon
        )
    }
}

/// The haversince-distance is the distance (e.g. in meters) between two points on a sphere (given in latitude and longitude).
///
/// The earth-radius is taken as `6371 km` since
///
/// 1. the resuluting sphere has same volume as the earth-ellipsoid, and
/// 1. it is the average radius.
///
///
/// ## Additional info
///
/// - [detailled information](http://www.movable-type.co.uk/scripts/latlong.html)
/// - [cpp](https://geographiclib.sourceforge.io/)
pub fn haversine_distance(from: &Coordinate, to: &Coordinate) -> f64 {
    let earth_mean_radius = 6_371.0; // kilometers

    let from_lat = from.lat();
    let from_lon = from.lon();
    let to_lat = to.lat();
    let to_lon = to.lon();

    let delta_lat = (from_lat - to_lat).to_radians();
    let delta_lon = (from_lon - to_lon).to_radians();

    let from_lat_rad = from_lat.to_radians();
    let to_lat_rad = to_lat.to_radians();

    let sin_lat = (delta_lat / 2.0).sin();
    let sin_lon = (delta_lon / 2.0).sin();

    (sin_lat * sin_lat + from_lat_rad.cos() * to_lat_rad.cos() * sin_lon * sin_lon)
        .sqrt()
        .asin()
        * (2.0 * earth_mean_radius)
}

/// Note that the result could have rounding errors due to up-scaling (* 1000.0) and cutting afterwards (f64 -> u32)
pub fn haversine_distance_m(from: &Coordinate, to: &Coordinate) -> Meters {
    Meters::from((1_000.0 * haversine_distance(from, to)) as u32)
}
