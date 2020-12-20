use crate::typ::Expr;
use crate::{
    stack::{Stack, StackOwner},
    tape::Tape,
};

type Compiled<'a> = fn(Stack<'a, i64>, Tape<'a, isize>);

fn call<'a>(k: isize, stack: Stack<'a, i64>, tape: Tape<'a, isize>) {
    let k: Compiled<'a> = unsafe { std::mem::transmute(k) };
    k(stack, tape)
}

fn measure(x: &Expr) -> usize {
    let mut codes = Vec::new();
    let mut height = 2;
    compiler(x, &mut codes, &mut height);
    codes.len()
}

pub fn compile(x: &Expr) -> Vec<isize> {
    let mut codes = Vec::new();
    let mut reg = 8;
    compiler(x, &mut codes, &mut reg);
    fn ret<'a>(_: Stack<'a, i64>, _: Tape<'a, isize>) {}
    codes.push(ret as isize);
    codes
}

fn compiler(x: &Expr, codes: &mut Vec<isize>, reg: &mut usize) -> usize {
    match x {
        Expr::Lit(i) => match *i {
            -1 => 2,
            2 => 3,
            3 => 4,
            4 => 5,
            100 => 6,
            1000 => 7,
            1000000 => 8,
            _ => unimplemented!("No constant {}", i),
        },
        Expr::Var(u) => *u,
        Expr::Add(x, y) => {
            let x = compiler(x, codes, reg);
            let y = compiler(y, codes, reg);
            let res = *reg;
            *reg += 1;
            fn f<'a>(stack: Stack<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, x) = tape.next();
                let (tape, y) = tape.next();
                let (tape, z) = tape.next();
                let (tape, k) = tape.next();
                stack.set(z as usize, stack.get(x as usize) + stack.get(y as usize));
                call(k, stack, tape)
            }
            codes.push(f as isize);
            codes.push(x as isize);
            codes.push(y as isize);
            codes.push(res as isize);
            res
        }
        Expr::Then(x, y) => {
            let before = *reg;
            compiler(x, codes, reg);
            *reg = before;
            compiler(y, codes, reg)
        }
        Expr::Assign(u, box Expr::Add(x, y)) => {
            let x = compiler(x, codes, reg);
            let y = compiler(y, codes, reg);
            fn f<'a>(stack: Stack<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, x) = tape.next();
                let (tape, y) = tape.next();
                let (tape, z) = tape.next();
                let (tape, k) = tape.next();
                stack.set(z as usize, stack.get(x as usize) + stack.get(y as usize));
                call(k, stack, tape)
            }
            codes.push(f as isize);
            codes.push(x as isize);
            codes.push(y as isize);
            codes.push(*u as isize);
            *u
        }
        Expr::Assign(u, x) => {
            let from = compiler(x, codes, reg);
            fn f<'a>(stack: Stack<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, from) = tape.next();
                let (tape, to) = tape.next();
                let (tape, k) = tape.next();
                stack.set(to as usize, stack.get(from as usize));
                call(k, stack, tape);
            }
            codes.push(f as isize);
            codes.push(from as isize);
            codes.push(*u as isize);
            0
        }
        Expr::While(a, b) => {
            fn after_a<'a>(stack: Stack<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, r) = tape.next();
                let (tape, len_b) = tape.next();
                if stack.get(r as usize) == 0 {
                    let tape = tape.jump(len_b);
                    let (tape, k) = tape.next();
                    call(k, stack, tape);
                } else {
                    let (tape, k) = tape.next();
                    call(k, stack, tape);
                }
            }
            fn after_b<'a>(stack: Stack<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, len_ab) = tape.next();
                let tape = tape.jump(len_ab);
                let (tape, k) = tape.next();
                call(k, stack, tape);
            }
            let res = compiler(a, codes, reg);
            codes.push(after_a as isize);
            codes.push(res as isize);
            codes.push((measure(b) + 2) as isize);
            compiler(b, codes, reg);
            codes.push(after_b as isize);
            codes.push(-((measure(a) + measure(b) + 5) as isize));
            0
        }
    }
}

pub fn run(compiled: &Vec<isize>) -> i64 {
    let mut stack = StackOwner::new(1000);
    let tape = Tape::new(compiled);
    let (tape, k) = tape.next();
    let mut s = stack.stack();
    for _ in 1..20 {
        s = s.push(0);
    }
    s = s.push(1000000); // Constants
    s = s.push(1000);
    s = s.push(100);
    s = s.push(4);
    s = s.push(3);
    s = s.push(2);
    s = s.push(-1);
    s = s.push(0); // v2
    s = s.push(0); // v1
    call(k, s, tape);
    stack.peek(27)
}
