mod awid;
mod timestamp;

pub use awid::*;
pub use timestamp::*;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Awid;

    #[test]
    fn awids_unique() {
        let mut rng = rand::thread_rng();

        let mut awids = HashSet::new();
        for _ in 0..10 {
            awids.insert(Awid::now(&mut rng));
        }
    }

    #[test]
    fn awids_bytes() {
        let mut rng = rand::thread_rng();
        let awid = Awid::now(&mut rng);
        let timestamp = awid.timestamp();

        let encoded = awid.as_bytes();
        let decoded = Awid::from_bytes(encoded);

        assert_eq!(timestamp, decoded.timestamp());
    }

    #[test]
    fn awid_from_bytes() {
        let bytes = [174, 142, 104, 102, 19, 152, 22, 129, 181];
        let awid = Awid::from_bytes(bytes);

        assert_eq!(awid.timestamp().as_unix_secs(), 1718128302)
    }

    #[test]
    fn awid_from_base32() {
        let encoded = "467PGSP7P6H01S8";
        let awid = Awid::from_base32(encoded).unwrap();

        assert_eq!(awid.timestamp().as_unix_secs(), 1718128417)
    }
}
