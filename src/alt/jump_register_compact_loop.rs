use crate::typ::{add, Expr};
use crate::{registers::Registers, tape::Tape};

type Compiled<'a> = fn(Registers<'a, i64>, Tape<'a, isize>);

fn call<'a>(k: isize, stack: Registers<'a, i64>, tape: Tape<'a, isize>) {
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
    fn ret<'a>(_: Registers<'a, i64>, _: Tape<'a, isize>) {}
    codes.push(ret as isize);
    codes
}

fn compact(x: usize, y: usize, z: usize) -> isize {
    unsafe { std::mem::transmute([x as u8, y as u8, z as u8, 0, 0, 0, 0, 0]) }
}

fn uncompact(xyz: isize) -> (usize, usize, usize) {
    let [x, y, z, _, _, _, _, _]: [u8; 8] = unsafe { std::mem::transmute(xyz) };
    (x as usize, y as usize, z as usize)
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
            fn f<'a>(stack: Registers<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, xyz) = tape.next();
                let (x, y, z) = uncompact(xyz);
                let (tape, k) = tape.next();
                stack.set(
                    z as usize,
                    add(stack.get(x as usize), stack.get(y as usize)),
                );
                call(k, stack, tape)
            }
            codes.push(f as isize);
            codes.push(compact(x, y, res));
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
            fn f<'a>(stack: Registers<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, xyz) = tape.next();
                let (x, y, z) = uncompact(xyz);
                let (tape, k) = tape.next();
                stack.set(z as usize, stack.get(x as usize) + stack.get(y as usize));
                call(k, stack, tape)
            }
            codes.push(f as isize);
            codes.push(compact(x, y, *u));
            *u
        }
        Expr::Assign(u, x) => {
            let from = compiler(x, codes, reg);
            fn f<'a>(stack: Registers<'a, i64>, tape: Tape<'a, isize>) {
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
            let len = measure(a) + measure(b);

            fn after_a<'a>(stack: Registers<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, r) = tape.next();
                let (tape, len) = tape.next();
                if stack.get(r as usize) == 0 {
                    let tape = tape.jump(len);
                    let (tape, k) = tape.next();
                    call(k, stack, tape);
                } else {
                    let (tape, k) = tape.next();
                    call(k, stack, tape);
                }
            }
            fn after_ba<'a>(stack: Registers<'a, i64>, tape: Tape<'a, isize>) {
                let (tape, r) = tape.next();
                let (tape, len) = tape.next();
                if stack.get(r as usize) == 0 {
                    let (tape, k) = tape.next();
                    call(k, stack, tape);
                } else {
                    let tape = tape.jump(len);
                    let (tape, k) = tape.next();
                    call(k, stack, tape);
                }
            }
            let res = compiler(a, codes, reg);
            codes.push(after_a as isize);
            codes.push(res as isize);
            codes.push((len + 3) as isize);

            compiler(b, codes, reg);
            let res = compiler(a, codes, reg);
            codes.push(after_ba as isize);
            codes.push(res as isize);
            codes.push(-((len + 3) as isize));
            0
        }
    }
}

pub fn run(compiled: &Vec<isize>) -> i64 {
    let mut registers = vec![0; 100];
    let tape = Tape::new(compiled);
    let (tape, k) = tape.next();
    registers[2] = -1;
    registers[3] = 2;
    registers[4] = 3;
    registers[5] = 4;
    registers[6] = 100;
    registers[7] = 1000;
    registers[8] = 1000000;
    call(k, Registers::new(&mut registers), tape);
    registers[1]
}
