use crate::typ::Expr;

pub fn run(x: &Expr) -> i64 {
    fn f(x: &Expr, slots: &mut Vec<i64>) -> i64 {
        match x {
            Expr::Lit(i) => *i,
            Expr::Var(u) => unsafe { *slots.get_unchecked(*u) },
            Expr::Add(x, y) => f(x, slots) + f(y, slots),
            Expr::Then(x, y) => {
                f(x, slots);
                f(y, slots)
            }
            Expr::Assign(u, x) => {
                let v = f(x, slots);
                unsafe { *slots.get_unchecked_mut(*u) = v };
                v
            }
            Expr::While(a, b) => {
                while f(a, slots) != 0 {
                    f(b, slots);
                }
                0
            }
        }
    }

    let mut slots = vec![0; 10];
    f(&*x, &mut slots)
}
