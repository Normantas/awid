use awid::{Awid, Timestamp};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn create_awid() -> Awid {
    let mut rng = rand::thread_rng();

    Awid::now(&mut rng)
}

fn create_awid_from_timestamp(timestamp: Timestamp) -> Awid {
    let mut rng = rand::thread_rng();

    Awid::new(timestamp, &mut rng)
}

fn criterion_benchmark(c: &mut Criterion) {
    let timestamp = Timestamp::now();

    c.bench_function("create Awid", |b| b.iter(|| create_awid()));
    c.bench_function("create Awid from timestamp", |b| {
        b.iter(|| create_awid_from_timestamp(black_box(timestamp)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
