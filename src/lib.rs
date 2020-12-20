#![feature(box_patterns)]
#![feature(test)]

use std::{any::Any, hint::black_box};

pub mod alt;
pub mod example;
mod registers;
mod stack;
mod tape;
pub mod typ;

pub struct Entry {
    name: &'static str,
    // Existential type wrapped in an Any to encode higher-rank polymorphism
    arg: Box<dyn Any>,
    run: Box<dyn Fn(&dyn Any) -> i64>,
}

impl Entry {
    pub fn new<T: 'static>(name: &'static str, x: T, f: fn(&T) -> i64) -> Self {
        Self {
            name,
            arg: Box::new(x),
            run: Box::new(move |x| f(x.downcast_ref::<T>().unwrap())),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn apply(&self) -> i64 {
        (self.run)(black_box(self.arg.as_ref()))
    }
}

pub fn entries(repeat: i64) -> Vec<Entry> {
    use alt::*;
    let ast = example::ast(repeat);

    vec![
        Entry::new("raw", repeat, |x| raw::run(*x)),
        Entry::new("poor", repeat, |x| poor::run(*x)),
        Entry::new("ast", ast.as_ref().clone(), ast::run),
        Entry::new("bytecode", bytecode::bytecode(repeat), bytecode::run),
        Entry::new("closure", closure::compile(&ast), closure::run),
        Entry::new("closure_leak", closure_leak::compile(&ast), |x| {
            closure_leak::run(*x)
        }),
        Entry::new("jump_stack", jump_stack::compile(&ast), jump_stack::run),
        Entry::new(
            "jump_register",
            jump_register::compile(&ast),
            jump_register::run,
        ),
        Entry::new(
            "jump_register_loop",
            jump_register_loop::compile(&ast),
            jump_register_loop::run,
        ),
        Entry::new(
            "jump_register_compact",
            jump_register_compact::compile(&ast),
            jump_register_compact::run,
        ),
    ]
}
