#[inline(never)]
fn add(x: i64, y: i64) -> i64 {
    x + y
}

pub fn run(mut x0: i64) -> i64 {
    let mut x1: i64 = 100;
    while x0 != 0 {
        x1 = add(add(add(x1, 4), x1), 3);
        x1 = add(add(x1, 2), 4);
        x0 = add(x0, -1);
    }
    x1
}
