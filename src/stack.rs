use std::marker::PhantomData;

pub struct Stack<'a, T: Copy> {
    top: *mut T,
    lifetime: PhantomData<&'a ()>,
}

pub struct StackOwner<T: Copy> {
    items: Vec<T>,
}

impl<T: Copy> StackOwner<T> {
    pub fn new(size: usize) -> Self
    where
        T: Default,
    {
        Self {
            items: vec![T::default(); size],
        }
    }

    pub fn stack(&mut self) -> Stack<T> {
        Stack {
            top: unsafe { self.items.get_unchecked_mut(0) } as *mut T,
            lifetime: Default::default(),
        }
    }

    pub fn peek(&self, i: usize) -> T {
        self.items[i]
    }
}

impl<'a, T: Copy> Stack<'a, T> {
    pub fn push(self, x: T) -> Self {
        let top2 = unsafe { self.top.add(1) };
        unsafe { *top2 = x };
        Self {
            top: top2,
            lifetime: self.lifetime,
        }
    }

    pub fn pop(self) -> (Self, T) {
        (
            Self {
                top: unsafe { self.top.sub(1) },
                lifetime: self.lifetime,
            },
            unsafe { *self.top },
        )
    }

    pub fn set(&self, offset: usize, value: T) {
        unsafe { *self.top.sub(offset) = value }
    }

    pub fn get(&self, offset: usize) -> T {
        unsafe { *self.top.sub(offset) }
    }
}
