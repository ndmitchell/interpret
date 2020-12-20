use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("interpreter");
    let entries = interpret::entries(1000);

    let mut results = Vec::new();
    for e in entries {
        results.push(e.apply());
        group.bench_function(e.name(), |b| b.iter(|| e.apply()));
    }
    println!("{:?}", results);
    results.dedup();
    assert_eq!(results.len(), 1);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
