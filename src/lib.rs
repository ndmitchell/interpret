#![feature(box_patterns)]

pub mod ast;
pub mod bytecode;
pub mod closure;
pub mod closure_jump;
pub mod closure_jump_register;
pub mod closure_jump_register_compact;
pub mod closure_jump_register_loop;
pub mod closure_leak;
pub mod example;
pub mod typ;

mod registers;
mod stack;
mod tape;
