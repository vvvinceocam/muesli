use criterion::{criterion_group, criterion_main, Criterion};

use muesli::session_decode;

pub fn criterion_benchmark(c: &mut Criterion) {
    let session = include_bytes!("data/test.session");
    c.bench_function("decode large session", |b| {
        b.iter(|| session_decode(session));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
