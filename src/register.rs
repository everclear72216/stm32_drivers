
use core::ptr;

#[repr(C)]
pub struct ReadOnlyRegister<T> {
    value: T,
}

#[repr(C)]
pub struct WriteOnlyRegister<T> {
    value: T,
}

#[repr(C)]
pub struct ReadWriteRegister<T> {
    value: T,
}

impl<T> ReadOnlyRegister<T> {
    pub unsafe fn get(&self) -> T {
        ptr::read_volatile(&self.value)
    }

    pub unsafe fn get_bit(&self, u8 index) {

    }

    pub unsafe fn get_field(&self, T mask, u8 index) {

    }
}

impl<T> WriteOnlyRegister<T> {
    pub unsafe fn set(&mut self, value: T) -> () {
        ptr::write_volatile(&mut self.value, value);
    }
}

impl<T> ReadWriteRegister<T> {
    pub unsafe fn get(&self) -> T {
        ptr::read_volatile(&self.value)
    }

    pub unsafe fn set(&mut self, setter: fn(value: T) -> T) -> () {
        ptr::write_volatile(&mut self.value, setter(self.get()));
    }
}
