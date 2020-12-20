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
    let mut height = 2;
    compiler(x, &mut codes, &mut height);
    fn ret<'a>(_: Stack<'a, i64>, _: Tape<'a, isize>) {}
    codes.push(ret as isize);
    codes
}

fn compiler(x: &Expr, codes: &mut Vec<isize>, height: &mut usize) {
    match x {
        Expr::Lit(i) => {
            fn f<'a>(stack: Stack<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, arg) = tape.next();
                let (tape, k) = tape.next();
                let stack = stack.push(arg as i64);
                call(k, stack, tape);
            }
            codes.push(f as isize);
            codes.push(*i as isize);
            *height += 1;
        }
        Expr::Var(u) => {
            fn f<'a>(stack: Stack<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, arg) = tape.next();
                let (tape, k) = tape.next();
                let v = stack.get(arg as usize);
                let stack = stack.push(v);
                call(k, stack, tape);
            }
            codes.push(f as isize);
            codes.push((*height - u) as isize);
            *height += 1;
        }
        Expr::Add(x, y) => {
            compiler(x, codes, height);
            compiler(y, codes, height);
            fn f<'a>(stack: Stack<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, k) = tape.next();
                let (stack, x) = stack.pop();
                let (stack, y) = stack.pop();
                let stack = stack.push(x + y);
                call(k, stack, tape)
            }
            codes.push(f as isize);
            *height -= 1;
        }
        Expr::Then(x, y) => {
            let before = *height;
            compiler(x, codes, height);
            assert_eq!(before, *height, "{:?}", x);
            compiler(y, codes, height);
        }
        Expr::Assign(u, x) => {
            compiler(x, codes, height);
            fn f<'a>(stack: Stack<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, arg) = tape.next();
                let (tape, k) = tape.next();
                let (stack, v) = stack.pop();
                stack.set(arg as usize, v);
                call(k, stack, tape);
            }
            codes.push(f as isize);
            *height -= 1;
            codes.push((*height - u) as isize);
        }
        Expr::While(a, b) => {
            fn after_a<'a>(stack: Stack<'a, i64>, tape: Tape<'a, isize>) {
                let (stack, v) = stack.pop();
                let (tape, len_b) = tape.next();
                if v == 0 {
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
            compiler(a, codes, height);
            *height -= 1;
            codes.push(after_a as isize);
            codes.push((measure(b) + 2) as isize);
            compiler(b, codes, height);
            codes.push(after_b as isize);
            codes.push(-((measure(a) + measure(b) + 4) as isize))
        }
    }
}

pub fn run(compiled: &Vec<isize>) -> i64 {
    let mut stack = StackOwner::new(1000);
    let tape = Tape::new(compiled);
    let (tape, k) = tape.next();
    let s = stack.stack();
    let s = s.push(0); // v1
    let s = s.push(0); // v2
    call(k, s, tape);
    stack.peek(3)
}
