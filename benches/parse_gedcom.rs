use criterion::{criterion_group, criterion_main, Criterion};
use gedcom_rs::parse::parse_gedcom;

use std::time::Duration;

const FILENAME: &str = "data/complete.ged";

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse-gedcom");
    group.measurement_time(Duration::from_secs(30));

    // TODO: Benchmark individual types?
    group.bench_function("parse gedcom", |b| b.iter(|| parse_gedcom(FILENAME)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
