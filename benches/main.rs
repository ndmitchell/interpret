use criterion::{black_box, criterion_group, criterion_main, Criterion};

const REPEAT: i64 = 1000;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("interpreter");
    let ast = interpret::example::ast(REPEAT);
    let closure = interpret::closure::compile(&ast);
    let closure_leak = interpret::closure_leak::compile(&ast);

    group.bench_function("raw", |b| {
        b.iter(|| interpret::example::raw(black_box(REPEAT)))
    });
    group.bench_function("ast", |b| {
        b.iter(|| interpret::ast::interpret(black_box(&ast)))
    });
    group.bench_function("closure", |b| {
        b.iter(|| interpret::closure::run(black_box(&closure)))
    });
    group.bench_function("closure_leak", |b| {
        b.iter(|| interpret::closure_leak::run(black_box(closure_leak)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
