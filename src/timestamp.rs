#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Timestamp {
    unix_secs: u32,
}

impl Timestamp {
    /// Creates a timestamp of the current time
    /// 
    /// # Panics
    /// Panics if the current Unix time overflows a `u32`
    pub fn now() -> Timestamp {
        let unix_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

        Self {
            unix_secs: unix_time.as_secs() as u32,
        }
    }

    /// Creates a timestamp from some Unix seconds
    pub fn from_unix_secs(unix_secs: u32) -> Timestamp {
        Self {
            unix_secs
        }
    }

    /// Returns the timestamp as bytes
    pub fn as_bytes(&self) -> [u8; 9] {
        format!("{:0<10}", self.unix_secs).as_bytes().try_into().unwrap()
    }

    /// Returns the timestamp's time as Unix seconds
    pub fn as_unix_secs(&self) -> u32 {
        self.unix_secs
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::Timestamp;

    #[test]
    fn timestamp_works() {
        let timestamp = Timestamp::now();
        
        assert_eq!(timestamp.unix_secs as u64, SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())
    }
}