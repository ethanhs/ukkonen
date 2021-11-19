use criterion::{criterion_group, criterion_main, Criterion};

use eddie::Levenshtein;
use std::time::Duration;

use ukkonen_rs::ukkonen;

pub fn leven_benchmark(cr: &mut Criterion) {
    let mut group = cr.benchmark_group("leven");
    let leven = Levenshtein::new();
    let gpl = include_str!("gpl.txt");
    let apache = include_str!("apache.txt");
    let five_percent = std::cmp::min(gpl.len(), apache.len()) / 20;

    group.sample_size(20).bench_function("eddie", |bench| {
        bench.iter(|| leven.distance(gpl, apache));
    });

    group.sample_size(20).bench_function("ukkonnen", |bench| {
        bench.iter(|| ukkonen(gpl, apache, five_percent as isize));
    });
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
                .warm_up_time(Duration::from_millis(50))
                .measurement_time(Duration::from_millis(200));
    targets = leven_benchmark
}

criterion_main!(benches);
