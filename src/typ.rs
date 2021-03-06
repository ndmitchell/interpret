#[derive(Debug, Clone)]
pub enum Expr {
    Lit(i64),
    Var(usize),
    Add(Box<Expr>, Box<Expr>),
    Assign(usize, Box<Expr>),
    Then(Box<Expr>, Box<Expr>),
    While(Box<Expr>, Box<Expr>),
}

#[inline(never)]
pub fn add(x: i64, y: i64) -> i64 {
    x + y
}
