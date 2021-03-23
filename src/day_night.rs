use std::time::{Duration, UNIX_EPOCH};

use chrono::prelude::*;
use sunrise::sunrise_sunset;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DayNight {
    Day,
    Night,
}

impl DayNight {
    pub fn current(latitude: f64, longitude: f64) -> Self {
        // Current time
        let time = Utc::now();

        // Calculated sunrise and sunset
        let (sunrise, sunset) =
            sunrise_sunset(latitude, longitude, time.year(), time.month(), time.day());
        let sunrise = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(sunrise as u64));
        let sunset = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(sunset as u64));

        // Now we check
        if time > sunrise && time < sunset {
            Self::Day
        } else {
            Self::Night
        }
    }
}
