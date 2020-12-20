const REPEAT: i64 = 2;

pub fn main() {
    let ast = interpret::example::ast(REPEAT);
    let closure = interpret::closure::compile(&ast);
    let closure_leak = interpret::closure_leak::compile(&ast);
    let jump_stack = interpret::jump_stack::compile(&ast);
    let jump_register = interpret::jump_register::compile(&ast);
    let jump_register_loop = interpret::jump_register_loop::compile(&ast);
    let jump_register_compact = interpret::jump_register_compact::compile(&ast);
    let bytecode = interpret::bytecode::bytecode(REPEAT);

    let mut results = Vec::new();
    println!("A");
    results.push(interpret::example::raw(REPEAT));
    println!("B");
    results.push(interpret::ast::interpret(&ast));
    println!("C");
    results.push(interpret::closure::run(&closure));
    println!("D");
    results.push(interpret::closure_leak::run(closure_leak));
    println!("E");
    results.push(interpret::jump_stack::run(&jump_stack));
    println!("F");
    results.push(interpret::jump_register::run(&jump_register));
    results.push(interpret::jump_register_loop::run(&jump_register_loop));
    println!("F");
    results.push(interpret::jump_register_compact::run(
        &jump_register_compact,
    ));
    results.push(interpret::bytecode::run(&bytecode));
    println!("G");

    println!("{:?}", results);
    results.dedup();
    assert_eq!(results.len(), 1);
}
