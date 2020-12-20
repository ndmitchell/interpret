use crate::typ::Expr;

type Compiled = &'static dyn Fn(&mut Vec<i64>) -> i64;

pub fn compile(x: &Expr) -> Compiled {
    let res: Box<dyn Fn(&mut Vec<i64>) -> i64> = match x {
        Expr::Lit(i) => {
            let i = *i;
            Box::new(move |_| i)
        }
        Expr::Var(u) => {
            let u = *u;
            Box::new(move |slots| unsafe { *slots.get_unchecked(u) })
        }
        Expr::Add(x, y) => {
            let x = compile(x);
            let y = compile(y);
            Box::new(move |slots| x(slots) + y(slots))
        }
        Expr::Then(x, y) => {
            let x = compile(x);
            let y = compile(y);
            Box::new(move |slots| {
                x(slots);
                y(slots)
            })
        }
        Expr::Assign(u, x) => {
            let x = compile(x);
            let u = *u;
            Box::new(move |slots| {
                let v = x(slots);
                unsafe { *slots.get_unchecked_mut(u) = v };
                v
            })
        }
        Expr::While(a, b) => {
            let a = compile(a);
            let b = compile(b);
            Box::new(move |slots| {
                while a(slots) != 0 {
                    b(slots);
                }
                0
            })
        }
    };
    Box::leak(res)
}

pub fn run(compiled: Compiled) -> i64 {
    let mut slots = vec![0; 10];
    compiled(&mut slots)
}
