use std::marker::PhantomData;

pub struct Registers<'a, T: Copy> {
    items: *mut T,
    lifetime: PhantomData<&'a ()>,
}

impl<'a, T: Copy> Registers<'a, T> {
    pub fn new(items: &'a mut Vec<T>) -> Self {
        Self {
            items: unsafe { items.get_unchecked_mut(0) } as *mut T,
            lifetime: Default::default(),
        }
    }

    pub fn set(&self, offset: usize, value: T) {
        unsafe { *self.items.add(offset) = value }
    }

    pub fn get(&self, offset: usize) -> T {
        unsafe { *self.items.add(offset) }
    }
}
