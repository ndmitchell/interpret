use std::marker::PhantomData;

pub struct Tape<'a, T: Copy> {
    top: *const T,
    lifetime: PhantomData<&'a ()>,
}

impl<'a, T: Copy> Tape<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        Self {
            top: unsafe { data.get_unchecked(0) },
            lifetime: Default::default(),
        }
    }

    pub fn next(self) -> (Self, T) {
        (
            Self {
                top: unsafe { self.top.add(1) },
                lifetime: self.lifetime,
            },
            unsafe { *self.top },
        )
    }

    pub fn jump(self, offset: isize) -> Self {
        Self {
            top: unsafe { self.top.offset(offset) },
            lifetime: self.lifetime,
        }
    }
}
