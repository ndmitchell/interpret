#![feature(box_patterns)]

pub mod ast;
pub mod bytecode;
pub mod closure;
pub mod closure_leak;
pub mod example;
pub mod jump_register;
pub mod jump_register_compact;
pub mod jump_register_loop;
pub mod jump_stack;
pub mod typ;

mod registers;
mod stack;
mod tape;
