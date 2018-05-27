
use core::ops::{Deref, DerefMut};

use register::ReadWriteRegister;

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

pub struct Rcc { }

impl Rcc {

    pub fn ptr() -> *const Registers {

        0x4002_3800 as *const _
    }

    pub fn mut_ptr() -> *mut Registers {

        0x4002_3800 as *mut _
    }
}

impl Deref for Rcc {

    type Target = Registers;

    fn deref(&self) -> &Registers {

        unsafe {

            &*Rcc::ptr()
        }
    }
}

impl DerefMut for Rcc {

    fn deref_mut(&mut self) -> &mut Registers {

        unsafe {

            &mut *Rcc::mut_ptr()
        }
    }
}
