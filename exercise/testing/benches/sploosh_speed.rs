use criterion::{black_box, Criterion, criterion_group, criterion_main};
use testing::sploosh;

fn sploosh_benchmark(c: &mut Criterion) {
    c.bench_function(
        "sploosh(8, 9, 10)",
        |b| b.iter(
            || sploosh(
                black_box(8),
                black_box(9),
                black_box(10),
            )
        ),
    );
}

criterion_group!(benches, sploosh_benchmark);
criterion_main!(benches);
