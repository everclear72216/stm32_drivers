use core::marker::PhantomData;
use core::ops::Deref;
use core::ops::DerefMut;

use rcc::regs::Registers;
use rcc::traits::HasRcc;

pub struct Rcc<T>
where
    T: HasRcc,
{
    _marker: PhantomData<T>,
}

impl<T> Rcc<T>
where
    T: HasRcc,
{
    pub fn new() -> Rcc<T> {
        Rcc::<T> {
            _marker: PhantomData::<T> {},
        }
    }

    const fn ptr() -> *const Registers {
        T::RCC as *const _
    }

    const fn mut_ptr() -> *mut Registers {
        T::RCC as *mut _
    }
}

impl<T> Deref for Rcc<T>
where
    T: HasRcc,
{
    type Target = Registers;

    fn deref(&self) -> &Registers {
        unsafe { &*Rcc::<T>::ptr() }
    }
}

impl<T> DerefMut for Rcc<T>
where
    T: HasRcc,
{
    fn deref_mut(&mut self) -> &mut Registers {
        unsafe { &mut *Rcc::<T>::mut_ptr() }
    }
}
