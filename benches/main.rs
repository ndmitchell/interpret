use criterion::{black_box, criterion_group, criterion_main, Criterion};
use interpret::alt::*;

const REPEAT: i64 = 1000;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("interpreter");
    let ast = interpret::example::ast(REPEAT);
    let closure = closure::compile(&ast);
    let closure_leak = closure_leak::compile(&ast);
    let jump_stack = jump_stack::compile(&ast);
    let jump_register = jump_register::compile(&ast);
    let jump_register_loop = jump_register_loop::compile(&ast);
    let jump_register_compact = jump_register_compact::compile(&ast);
    let bytecode = bytecode::bytecode(REPEAT);

    let mut results = Vec::new();
    results.push(raw::run(REPEAT));
    results.push(poor::run(REPEAT));
    results.push(ast::run(&ast));
    results.push(closure::run(&closure));
    results.push(closure_leak::run(closure_leak));
    results.push(jump_stack::run(&jump_stack));
    results.push(jump_register::run(&jump_register));
    results.push(jump_register_loop::run(&jump_register_loop));
    results.push(jump_register_compact::run(&jump_register_compact));
    results.push(bytecode::run(&bytecode));

    println!("{:?}", results);
    results.dedup();
    assert_eq!(results.len(), 1);

    group.bench_function("raw", |b| b.iter(|| raw::run(black_box(REPEAT))));
    group.bench_function("poor", |b| b.iter(|| poor::run(black_box(REPEAT))));
    group.bench_function("ast", |b| b.iter(|| ast::run(black_box(&ast))));
    group.bench_function("closure", |b| b.iter(|| closure::run(black_box(&closure))));
    group.bench_function("closure_leak", |b| {
        b.iter(|| closure_leak::run(black_box(closure_leak)))
    });
    group.bench_function("jump_stack", |b| {
        b.iter(|| jump_stack::run(black_box(&jump_stack)))
    });
    group.bench_function("jump_register", |b| {
        b.iter(|| jump_register::run(black_box(&jump_register)))
    });
    group.bench_function("jump_register_loop", |b| {
        b.iter(|| jump_register_loop::run(black_box(&jump_register_loop)))
    });
    group.bench_function("jump_register_compact", |b| {
        b.iter(|| jump_register_compact::run(black_box(&jump_register_compact)))
    });
    group.bench_function("bytecode", |b| {
        b.iter(|| bytecode::run(black_box(&bytecode)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
