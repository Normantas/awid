use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Timestamp {
    unix_secs: u32,
}

impl Timestamp {
    /// Creates a timestamp of the current time.
    ///
    /// # Panics
    /// Panics if the current Unix time overflows a `u32`
    pub fn now() -> Self {
        let unix_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        Self {
            unix_secs: unix_time.as_secs() as u32,
        }
    }

    /// Creates a timestamp from some Unix seconds.
    pub fn from_unix_secs(unix_secs: u32) -> Self {
        Self { unix_secs }
    }

    /// Creates a timestamp from the (little endian) byte representation of it.
    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        Self {
            unix_secs: u32::from_le_bytes(bytes),
        }
    }

    /// Returns the timestamp as bytes.
    pub fn as_bytes(&self) -> [u8; 4] {
        self.unix_secs.to_le_bytes()
    }

    /// Returns the timestamp's time as Unix seconds.
    pub fn as_unix_secs(&self) -> u32 {
        self.unix_secs
    }
}

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[cfg(feature = "chrono")]
#[derive(thiserror::Error, Debug)]
pub enum DateTimeError {
    #[error("the provided datetime is too large")]
    DateTimeTooLarge,
}

#[cfg(feature = "chrono")]
impl Timestamp {
    /// Returns the timestamp's time as a Chrono DateTime.
    pub fn to_datetime(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.unix_secs as i64, 0).unwrap()
    }

    /// Creates a timestamp from a Chrono DateTime.
    pub fn from_datetime(datetime: DateTime<Utc>) -> Result<Self, DateTimeError> {
        if let Ok(unix_secs) = datetime.timestamp().try_into() {
            Ok(Self::from_unix_secs(unix_secs))
        } else {
            Err(DateTimeError::DateTimeTooLarge)
        }
    }
}