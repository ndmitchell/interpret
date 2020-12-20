use interpret::alt::*;

const REPEAT: i64 = 2;

pub fn main() {
    let ast = interpret::example::ast(REPEAT);
    let closure = closure::compile(&ast);
    let closure_leak = closure_leak::compile(&ast);
    let jump_stack = jump_stack::compile(&ast);
    let jump_register = jump_register::compile(&ast);
    let jump_register_loop = jump_register_loop::compile(&ast);
    let jump_register_compact = jump_register_compact::compile(&ast);
    let bytecode = bytecode::bytecode(REPEAT);

    let mut results = Vec::new();
    println!("A");
    results.push(raw::run(REPEAT));
    println!("B");
    results.push(ast::run(&ast));
    println!("C");
    results.push(closure::run(&closure));
    println!("D");
    results.push(closure_leak::run(closure_leak));
    println!("E");
    results.push(jump_stack::run(&jump_stack));
    println!("F");
    results.push(jump_register::run(&jump_register));
    results.push(jump_register_loop::run(&jump_register_loop));
    println!("F");
    results.push(jump_register_compact::run(&jump_register_compact));
    results.push(bytecode::run(&bytecode));
    println!("G");

    println!("{:?}", results);
    results.dedup();
    assert_eq!(results.len(), 1);
}
