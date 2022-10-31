use core::{ops, marker::PhantomData};

pub struct MMIODerefWrapper<T> {
    start_addr: usize,
    phantom_data: PhantomData<T>,
}

impl<T> MMIODerefWrapper<T> {
    pub const unsafe fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom_data: PhantomData,
        }
    }
}

impl<T> ops::Deref for MMIODerefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(self.start_addr as *const _)
        }
    }
}