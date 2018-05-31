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

impl<T> ReadOnlyRegister<T>
where
    T: ::core::convert::From<u8>
        + ::core::ops::BitAnd<T, Output = T>
        + ::core::ops::Shl<T, Output = T>
        + ::core::ops::Shr<T, Output = T>
        + ::core::cmp::PartialOrd,
{
    pub unsafe fn get(&self) -> T {
        ptr::read_volatile(&self.value)
    }

    pub unsafe fn get_bit(&self, index: u8) -> bool {
        (self.get() & (T::from(1u8) << T::from(index))) > T::from(0u8)
    }

    pub unsafe fn get_field(&self, mask: T, index: u8) -> T {
        (self.get() & mask) >> T::from(index)
    }
}

impl<T> WriteOnlyRegister<T>
where
    T: ::core::convert::From<u8>
        + ::core::ops::Shl<T, Output = T>
        + ::core::ops::Shr<T, Output = T>
        + ::core::ops::Not<Output = T>,
{
    pub unsafe fn set(&mut self, value: T) -> () {
        ptr::write_volatile(&mut self.value, value);
    }

    pub unsafe fn set_bit(&mut self, index: u8) -> () {
        self.set(T::from(1) << T::from(index));
    }

    pub unsafe fn clear_bit(&mut self, index: u8) -> () {
        self.set(!(T::from(1) << T::from(index)));
    }

    pub unsafe fn set_field(&mut self, value: T, index: u8) -> () {
        self.set(value << T::from(index));
    }

    pub unsafe fn clear_field(&mut self, mask: T) -> () {
        self.set(!mask);
    }
}

impl<T> ReadWriteRegister<T>
where
    T: ::core::convert::From<u8>
        + ::core::ops::BitOr<T, Output = T>
        + ::core::ops::BitAnd<T, Output = T>
        + ::core::ops::Shl<T, Output = T>
        + ::core::ops::Shr<T, Output = T>
        + ::core::ops::Not<Output = T>
        + ::core::cmp::PartialOrd,
{
    pub unsafe fn get(&self) -> T {
        ptr::read_volatile(&self.value)
    }

    pub unsafe fn set(&mut self, value: T) -> () {
        ptr::write_volatile(&mut self.value, value);
    }

    pub unsafe fn get_bit(&self, index: u8) -> bool {
        (self.get() & (T::from(1u8) << T::from(index))) > T::from(0u8)
    }

    pub unsafe fn set_bit(&mut self, index: u8) -> () {
        let x = self.get() | (T::from(1u8) << T::from(index));
        self.set(x);
    }

    pub unsafe fn clear_bit(&mut self, index: u8) -> () {
        let x = self.get() & !(T::from(1u8) << T::from(index));
        self.set(x);
    }

    pub unsafe fn get_field(&self, mask: T, index: u8) -> T {
        (self.get() & mask) >> T::from(index)
    }

    pub unsafe fn set_field(&mut self, mask: T, index: u8, value: T) -> () {
        let x = (self.get() & !mask) | (value << T::from(index));
        self.set(x);
    }

    pub unsafe fn clear_field(&mut self, mask: T) -> () {
        self.set(!mask);
    }
}
