use std::cmp::Ordering;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    pub decimicro_lat: i32,
    pub decimicro_lon: i32,
}
impl Coordinate {
    pub fn new(decimicro_lat: i32, decimicro_lon: i32) -> Coordinate {
        Coordinate {
            decimicro_lat,
            decimicro_lon,
        }
    }

    pub fn from(lat: f64, lon: f64) -> Coordinate {
        Coordinate {
            decimicro_lat: (lat * 1e7) as i32,
            decimicro_lon: (lon * 1e7) as i32,
        }
    }

    pub fn lat(&self) -> f64 {
        self.decimicro_lat as f64 * 1e-7
    }

    pub fn lon(&self) -> f64 {
        self.decimicro_lon as f64 * 1e-7
    }
}
impl Eq for Coordinate {}
impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.decimicro_lat.cmp(&other.decimicro_lat) == Ordering::Equal
            && self.decimicro_lon.cmp(&other.decimicro_lon) == Ordering::Equal
    }
}
impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(dµ_lat, dµ_lon): ({}, {})",
            self.decimicro_lat, self.decimicro_lon
        )
    }
}

// TODO enum/struct Distance
// return value is in kilometers
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
