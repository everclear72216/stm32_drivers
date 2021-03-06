use core::marker::PhantomData;
use core::ops::Deref;
use core::ops::DerefMut;

use rcc::traits::Rcc as RccTrait;

use register::ReadWriteRegister;

#[repr(C)]
pub struct Registers {
    pub cr: ReadWriteRegister<u32>,
    pub pllcfgr: ReadWriteRegister<u32>,
    pub cfgr: ReadWriteRegister<u32>,
    pub cir: ReadWriteRegister<u32>,
    pub ahb1rstr: ReadWriteRegister<u32>,
    pub ahb2rstr: ReadWriteRegister<u32>,
    pub ahb3rstr: ReadWriteRegister<u32>,
    reserved0: u32,
    pub apb1rstr: ReadWriteRegister<u32>,
    pub apb2rstr: ReadWriteRegister<u32>,
    reserved1: [u32; 2],
    pub ahb1enr: ReadWriteRegister<u32>,
    pub ahb2enr: ReadWriteRegister<u32>,
    pub ahb3enr: ReadWriteRegister<u32>,
    reserved2: u32,
    pub apb1enr: ReadWriteRegister<u32>,
    pub apb2enr: ReadWriteRegister<u32>,
    reserved3: [u32; 2],
    pub ahb1lpenr: ReadWriteRegister<u32>,
    pub ahb2lpenr: ReadWriteRegister<u32>,
    pub ahb3lpenr: ReadWriteRegister<u32>,
    reserved4: u32,
    pub apb1lpenr: ReadWriteRegister<u32>,
    pub apb2lpenr: ReadWriteRegister<u32>,
    reserved5: [u32; 2],
    pub bdcr: ReadWriteRegister<u32>,
    pub csr: ReadWriteRegister<u32>,
    reserved6: [u32; 2],
    pub sscgr: ReadWriteRegister<u32>,
    pub plli2scfgr: ReadWriteRegister<u32>,
    pub pllsaicfgr: ReadWriteRegister<u32>,
    pub dckcfgr: ReadWriteRegister<u32>,
}

pub struct Rcc<T>
where
    T: RccTrait,
{
    _marker: PhantomData<T>,
}

impl<T> Rcc<T>
where
    T: RccTrait,
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
    T: RccTrait,
{
    type Target = Registers;

    fn deref(&self) -> &Registers {
        unsafe { &*Rcc::<T>::ptr() }
    }
}

impl<T> DerefMut for Rcc<T>
where
    T: RccTrait,
{
    fn deref_mut(&mut self) -> &mut Registers {
        unsafe { &mut *Rcc::<T>::mut_ptr() }
    }
}
