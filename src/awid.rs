#[cfg(feature = "base32")]
use base32::Alphabet;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use core::fmt;
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

        Self { timestamp, random }
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

    /// Creates an Awid from the (little endian) byte representation of it.
    pub fn from_bytes(bytes: [u8; 9]) -> Self {
        Self {
            timestamp: Timestamp::from_bytes(bytes[0..4].try_into().unwrap()),
            random: bytes[4..9].try_into().unwrap(),
        }
    }

    /// Return the bytes of the Awid. The first 4 bytes are the Unix timestamp, the last 5 bytes are the random data.
    pub fn as_bytes(&self) -> [u8; 9] {
        let mut bytes = Vec::new();
        bytes.extend(self.timestamp.as_bytes());
        bytes.extend(self.random);
        bytes.try_into().unwrap()
    }

    /// Gets the timestamp of the Awid
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

#[cfg(feature = "base32")]
impl fmt::Display for Awid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = self.as_bytes();
        write!(f, "{}", base32::encode(Alphabet::Crockford, &bytes))
    }
}
