const REPEAT: i64 = 2;

pub fn main() {
    let ast = interpret::example::ast(REPEAT);
    let closure = interpret::closure::compile(&ast);
    let closure_leak = interpret::closure_leak::compile(&ast);
    let closure_jump = interpret::closure_jump::compile(&ast);
    let closure_jump_register = interpret::closure_jump_register::compile(&ast);
    let closure_jump_register_loop = interpret::closure_jump_register_loop::compile(&ast);
    let closure_jump_register_compact = interpret::closure_jump_register_compact::compile(&ast);
    let best = interpret::best::compile(&ast);
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
    results.push(interpret::closure_jump::run(&closure_jump));
    println!("F");
    results.push(interpret::closure_jump_register::run(
        &closure_jump_register,
    ));
    results.push(interpret::closure_jump_register_loop::run(
        &closure_jump_register_loop,
    ));
    println!("F");
    results.push(interpret::closure_jump_register_compact::run(
        &closure_jump_register_compact,
    ));
    results.push(interpret::best::run(&best));
    println!("F");
    results.push(interpret::bytecode::run(&bytecode));
    println!("G");

    println!("{:?}", results);
    results.dedup();
    assert_eq!(results.len(), 1);
}
