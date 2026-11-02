//! A point on the map, for a location's pin and for nearest-first lists.

use serde::Deserialize;
use thiserror::Error;

/// A latitude and a longitude in degrees, both finite and in range:
///
/// ```toml
/// geo = { latitude = 53.7997, longitude = -1.5492 }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(try_from = "RawPoint")]
pub struct GeoPoint {
    latitude: f64,
    longitude: f64,
}

// Both numbers are finite, so equality is total.
impl Eq for GeoPoint {}

/// Why two numbers are not a point on the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum GeoPointError {
    /// The latitude is not between -90 and 90.
    #[error("the latitude must be a number from -90 to 90")]
    Latitude,
    /// The longitude is not between -180 and 180.
    #[error("the longitude must be a number from -180 to 180")]
    Longitude,
}

impl GeoPoint {
    /// The point at `latitude` and `longitude`, in degrees.
    ///
    /// # Errors
    ///
    /// [`GeoPointError`] for a number out of range or not finite.
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, GeoPointError> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(GeoPointError::Latitude);
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(GeoPointError::Longitude);
        }
        Ok(Self {
            latitude,
            longitude,
        })
    }

    /// Degrees north of the equator, negative for south.
    #[must_use]
    pub const fn latitude(self) -> f64 {
        self.latitude
    }

    /// Degrees east of Greenwich, negative for west.
    #[must_use]
    pub const fn longitude(self) -> f64 {
        self.longitude
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPoint {
    latitude: f64,
    longitude: f64,
}

impl TryFrom<RawPoint> for GeoPoint {
    type Error = GeoPointError;

    fn try_from(raw: RawPoint) -> Result<Self, Self::Error> {
        Self::new(raw.latitude, raw.longitude)
    }
}

#[cfg(test)]
mod tests {
    use super::{GeoPoint, GeoPointError};

    #[test]
    fn a_point_must_be_on_the_map() {
        let leeds = GeoPoint::new(53.7997, -1.5492).unwrap();
        assert_eq!(leeds.latitude().to_string(), "53.7997");
        assert_eq!(leeds.longitude().to_string(), "-1.5492");
        assert_eq!(GeoPoint::new(91.0, 0.0), Err(GeoPointError::Latitude));
        assert_eq!(GeoPoint::new(0.0, f64::NAN), Err(GeoPointError::Longitude));
    }
}
