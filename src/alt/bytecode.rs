pub enum Bytecode {
    Assign(u32),  // assign the value at the top of the stack to a slot
    Var(u32),     // push the value in slot to the top of the stack
    Lit(i32),     // push a literal on the stack
    Add,          // Add the top two items on the stack
    Jump(u32),    // Jump to a program counter
    JumpIf0(u32), // Jump to a PC if a value is 0
    Done,
}

pub fn bytecode(x0: i64) -> Vec<Bytecode> {
    use Bytecode::*;
    let prefix = vec![Lit(x0 as i32), Assign(0), Lit(100), Assign(1)];
    let inner1 = vec![Var(1), Lit(4), Var(1), Lit(3), Add, Add, Add, Assign(1)];
    let inner2 = vec![Var(1), Lit(2), Lit(4), Add, Add, Assign(1)];
    let inner3 = vec![Var(0), Lit(-1), Add, Assign(0)];
    let prefix_len = prefix.len();
    let inner_len = inner1.len() + inner2.len() + inner3.len();

    let mut res = Vec::new();
    res.extend(prefix);
    res.push(Var(0));
    res.push(JumpIf0((res.len() + 1 + inner_len + 1) as u32));
    res.extend(inner1);
    res.extend(inner2);
    res.extend(inner3);
    res.push(Jump(prefix_len as u32));
    res.push(Var(1));
    res.push(Done);
    res
}
struct Stack {
    ptr: usize,
    vals: [i64; 100],
}

impl Stack {
    fn new() -> Self {
        Self {
            ptr: 0,
            vals: [0; 100],
        }
    }

    fn pop(&mut self) -> i64 {
        self.ptr -= 1;
        unsafe { *self.vals.get_unchecked(self.ptr) }
    }

    fn push(&mut self, x: i64) {
        unsafe { *self.vals.get_unchecked_mut(self.ptr) = x };
        self.ptr += 1;
    }
}

pub fn run(xs: &Vec<Bytecode>) -> i64 {
    let mut pc = 0;
    let mut slots = vec![0; 10];
    let mut stack = Stack::new();

    loop {
        use Bytecode::*;
        // println!("{} at {} is {:?}", pc, stack.len(), slots);
        match xs[pc] {
            Assign(x) => slots[x as usize] = stack.pop(),
            Var(x) => stack.push(slots[x as usize]),
            Lit(i) => stack.push(i as i64),
            Add => {
                let x = stack.pop();
                let y = stack.pop();
                stack.push(x + y)
            }
            Jump(pc2) => pc = pc2 as usize - 1,
            JumpIf0(pc2) => {
                if stack.pop() == 0 {
                    pc = pc2 as usize - 1;
                }
            }
            Done => return stack.pop(),
        }
        pc = pc + 1;
    }
}
