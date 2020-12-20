use crate::typ::Expr;

pub fn ast(x0: i64) -> Box<Expr> {
    fn assign(x: usize, y: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Assign(x, y))
    }
    fn thens(mut x: Vec<Box<Expr>>) -> Box<Expr> {
        let mut res = x.pop().unwrap();
        while let Some(v) = x.pop() {
            res = Box::new(Expr::Then(v, res));
        }
        res
    }
    fn adds(mut x: Vec<Box<Expr>>) -> Box<Expr> {
        let mut res = x.pop().unwrap();
        while let Some(v) = x.pop() {
            res = Box::new(Expr::Add(v, res));
        }
        res
    }
    fn lit(x: i64) -> Box<Expr> {
        Box::new(Expr::Lit(x))
    }
    fn while_(x: Box<Expr>, y: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::While(x, y))
    }
    fn var(x: usize) -> Box<Expr> {
        Box::new(Expr::Var(x))
    }

    thens(vec![
        assign(0, lit(x0)),
        assign(1, lit(100)),
        while_(
            var(0),
            thens(vec![
                assign(1, adds(vec![var(1), lit(4), var(1), lit(3)])),
                assign(1, adds(vec![var(1), lit(2), lit(4)])),
                assign(0, adds(vec![var(0), lit(-1)])),
            ]),
        ),
        var(1),
    ])
}
