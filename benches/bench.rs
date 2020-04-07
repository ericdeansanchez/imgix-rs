extern crate imgix;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use imgix::Url;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(r#"Url::join_params single-param"#, |b| {
        b.iter(|| Url::join_params(black_box(&[("w", "320")])))
    });

    c.bench_function(r#"Url::join_params multi-param"#, |b| {
        b.iter(|| Url::join_params(black_box(&[("w", "320"), ("h", "640"), ("fit", "crop")])))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
