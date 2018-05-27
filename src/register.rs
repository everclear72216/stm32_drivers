
use core::ptr;

pub struct RO<T> {

    value: T,
}

pub struct RW<T> {

    value: T,
}

pub struct WO<T> {
    
    value: T,
}

impl<T> RO<T> {

    pub unsafe fn get(&self) -> T
        where T: Copy
    {
        ptr::read_volatile(&self.value)
    }
}
