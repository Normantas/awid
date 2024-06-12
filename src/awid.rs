use rand_core::RngCore;
use crate::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    /// Return the byte representation of the Awid. The first 4 bytes are the Unix timestamp, the last 5 bytes are the random data.
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
use base32::Alphabet;

#[cfg(feature = "base32")]
#[derive(thiserror::Error, Debug)]
pub enum FromBase32Error {
    #[error("error while decoding base32")]
    Base32DecodeError,
    #[error("the input was of invalid length")]
    InvalidLengthError,
}

#[cfg(feature = "base32")]
impl Awid {
    /// Creates an Awid from the base32 representation of it.
    pub fn from_base32(encoded: &str) -> Result<Self, FromBase32Error> {
        if let Some(decoded) = base32::decode(Alphabet::Crockford, &encoded) {
            let bytes: [u8; 9] = decoded.try_into().map_err(|_| FromBase32Error::InvalidLengthError)?;
            Ok(Self::from_bytes(bytes))
        } else {
            Err(FromBase32Error::Base32DecodeError)
        }
    }

    /// Return the base32 encoded representation of the Awid.
    pub fn to_base32(&self) -> String {
        let bytes = self.as_bytes();
        base32::encode(Alphabet::Crockford, &bytes)
    }
}

#[cfg(feature = "base32")]
use core::fmt;

#[cfg(feature = "base32")]
impl fmt::Display for Awid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_base32())
    }
}
