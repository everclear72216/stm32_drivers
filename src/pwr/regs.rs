use core::marker::PhantomData;
use core::ops::Deref;
use core::ops::DerefMut;

use pwr::Pwr as PwrTrait;

use register::ReadWriteRegister;

#[repr(C)]
pub struct Registers {
    pub cr: ReadWriteRegister<u32>,
    pub csr: ReadWriteRegister<u32>,
}

pub struct Pwr<T> 
where
    T: PwrTrait,
{
    _marker: PhantomData<T>
}

impl<T> Pwr<T> 
where
    T: PwrTrait,
{
    pub fn new() -> Pwr<T> {
        Pwr::<T> {
            _marker: PhantomData::<T> {},
        }
    }

    const fn ptr() -> *const Registers {
        T::PWR as *const _
    }

    const fn mut_ptr() -> *mut Registers {
        T::PWR as *mut _
    }
}

impl<T> Deref for Pwr<T>
where
    T: PwrTrait,
{
    type Target = Registers;

    fn deref(&self) -> &Registers {
        unsafe { &*Pwr::<T>::ptr() }
    }
}

impl<T> DerefMut for Pwr<T>
where
    T: PwrTrait,
{
    fn deref_mut(&mut self) -> &mut Registers {
        unsafe { &mut *Pwr::<T>::mut_ptr() }
    }
}
