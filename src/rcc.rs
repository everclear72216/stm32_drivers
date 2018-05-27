
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use register::ReadWriteRegister;

pub trait HasRcc {
    const RCC: u32;
}

#[repr(C)]
pub struct Registers {
    pub cr:         ReadWriteRegister<u32>,
    pub pllcfgr:    ReadWriteRegister<u32>,
    pub cfgr:       ReadWriteRegister<u32>,
    pub cir:        ReadWriteRegister<u32>,
    pub ahb1rstr:   ReadWriteRegister<u32>,
    pub ahb2rstr:   ReadWriteRegister<u32>,
    pub ahb3rstr:   ReadWriteRegister<u32>,
    reserved0:      u32,
    pub apb1rstr:   ReadWriteRegister<u32>,
    pub apb2rstr:   ReadWriteRegister<u32>,
    reserved1:      [u32; 2],
    pub ahb1enr:    ReadWriteRegister<u32>,
    pub ahb2enr:    ReadWriteRegister<u32>,
    pub ahb3enr:    ReadWriteRegister<u32>,
    reserved2:      u32,
    pub apb1enr:    ReadWriteRegister<u32>,
    pub apb2enr:    ReadWriteRegister<u32>,
    reserved3:      [u32; 2],
    pub ahb1lpenr:  ReadWriteRegister<u32>,
    pub ahb2lpenr:  ReadWriteRegister<u32>,
    pub ahb3lpenr:  ReadWriteRegister<u32>,
    reserved4:      u32,
    pub apb1lpenr:  ReadWriteRegister<u32>,
    pub apb2lpenr:  ReadWriteRegister<u32>,
    reserved5:      [u32; 2],
    pub bdcr:       ReadWriteRegister<u32>,
    pub csr:        ReadWriteRegister<u32>,
    reserved6:      [u32; 2],
    pub sscgr:      ReadWriteRegister<u32>,
    pub plli2scfgr: ReadWriteRegister<u32>,
    pub pllsaicfgr: ReadWriteRegister<u32>,
    pub dckcfgr:    ReadWriteRegister<u32>,
}

#[derive(Default)]
pub struct Rcc<T>
    where T: HasRcc
{
    pub _marker: PhantomData<T>,
}

impl<T> Rcc<T> 
    where T: HasRcc
{
    fn ptr() -> *const Registers {

        T::RCC as *const _
    }

    fn mut_ptr() -> *mut Registers {

        T::RCC as *mut _
    }
}

impl<T> Deref for Rcc<T>
    where T: HasRcc
{
    type Target = Registers;

    fn deref(&self) -> &Registers {

        unsafe {

            &*Rcc::<T>::ptr()
        }
    }
}

impl<T> DerefMut for Rcc<T>
    where T: HasRcc
{
    fn deref_mut(&mut self) -> &mut Registers {

        unsafe {

            &mut *Rcc::<T>::mut_ptr()
        }
    }
}
