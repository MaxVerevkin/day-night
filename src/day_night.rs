use std::time::{Duration, UNIX_EPOCH};

use chrono::prelude::*;
use sunrise::sunrise_sunset;

use crate::LAT;
use crate::LON;

#[derive(Clone, Copy, Debug)]
pub enum DayNight {
    Day,
    Night,
}

impl DayNight {
    pub fn current() -> Self {
        // Current time
        let time = Utc::now();

        // Calculated sunrise and sunset
        let (sunrise, sunset) = sunrise_sunset(LAT, LON, time.year(), time.month(), time.day());
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
