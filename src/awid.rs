#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use rand_core::RngCore;

use crate::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Awid {
    timestamp: Timestamp,
    random: [u8; 5],
}

impl Awid {
    /// Create a new Awid.
    pub fn new(timestamp: Timestamp, rng: &mut impl RngCore) -> Self {
        let mut random: [u8; 5] = [0; 5];
        rng.fill_bytes(&mut random);

        Self {
            timestamp,
            random,
        }
    }

    /// Create an Awid with a timestamp of the current time.
    pub fn now(rng: &mut impl RngCore) -> Self {
        let mut random: [u8; 5] = [0; 5];
        rng.fill_bytes(&mut random);

        Self {
            timestamp: Timestamp::now(),
            random,
        }
    }

    /// Return the bytes of the Awid. The first 4 bytes are the Unix timestamp, the last 5 bytes are the random data.
    pub fn as_bytes(&self) -> [u8; 9] {
        let mut bytes = Vec::new();
        bytes.extend(self.timestamp.as_bytes());
        bytes.extend(self.random);
        bytes.try_into().unwrap()
    }
}