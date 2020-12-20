use criterion::{black_box, criterion_group, criterion_main, Criterion};

const REPEAT: i64 = 1000;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("interpreter");
    let ast = interpret::example::ast(REPEAT);
    let closure = interpret::closure::compile(&ast);
    let closure_leak = interpret::closure_leak::compile(&ast);
    let jump_stack = interpret::jump_stack::compile(&ast);
    let jump_register = interpret::jump_register::compile(&ast);
    let jump_register_loop = interpret::jump_register_loop::compile(&ast);
    let jump_register_compact = interpret::jump_register_compact::compile(&ast);
    let bytecode = interpret::bytecode::bytecode(REPEAT);

    let mut results = Vec::new();
    results.push(interpret::example::raw(REPEAT));
    results.push(interpret::example::poor(REPEAT));
    results.push(interpret::ast::interpret(&ast));
    results.push(interpret::closure::run(&closure));
    results.push(interpret::closure_leak::run(closure_leak));
    results.push(interpret::jump_stack::run(&jump_stack));
    results.push(interpret::jump_register::run(&jump_register));
    results.push(interpret::jump_register_loop::run(&jump_register_loop));
    results.push(interpret::jump_register_compact::run(
        &jump_register_compact,
    ));
    results.push(interpret::bytecode::run(&bytecode));

    println!("{:?}", results);
    results.dedup();
    assert_eq!(results.len(), 1);

    group.bench_function("raw", |b| {
        b.iter(|| interpret::example::raw(black_box(REPEAT)))
    });
    group.bench_function("poor", |b| {
        b.iter(|| interpret::example::poor(black_box(REPEAT)))
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
    group.bench_function("jump_stack", |b| {
        b.iter(|| interpret::jump_stack::run(black_box(&jump_stack)))
    });
    group.bench_function("jump_register", |b| {
        b.iter(|| interpret::jump_register::run(black_box(&jump_register)))
    });
    group.bench_function("jump_register_loop", |b| {
        b.iter(|| interpret::jump_register_loop::run(black_box(&jump_register_loop)))
    });
    group.bench_function("jump_register_compact", |b| {
        b.iter(|| interpret::jump_register_compact::run(black_box(&jump_register_compact)))
    });
    group.bench_function("bytecode", |b| {
        b.iter(|| interpret::bytecode::run(black_box(&bytecode)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
