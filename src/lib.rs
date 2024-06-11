mod awid;
mod timestamp;

pub use awid::Awid;
pub use timestamp::Timestamp;

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
}
