
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

impl Registers {
    const cr_offset: u32 = 0x00u32;
    const pllcfgr_offset: u32 = 0x04u32;
    const cfgr_offset: u32 = 0x80u32;
    const cir_offset: u32 = 0xCu32;
    const ahb1rstr_offset: u32 = 0x10u32;
    const ahb2rstr_offset: u32 = 0x14u32;
    const ahb3rstr_offset: u32 = 0x18u32;
    const apb1rstr_offset: u32 = 0x20u32;
    const apb2rstr_offset: u32 = 0x24u32;
    const ahb1enr_offset: u32 = 0x30u32;
    const ahb2enr_offset: u32 = 0x34u32;
    const ahb3enr_offset: u32 = 0x38u32;
    const apb1enr_offset: u32 = 0x40u32;
    const apb2enr_offset: u32 = 0x44u32;
    const ahb1lpenr_offset: u32 = 0x50u32;
    const ahb2lpenr_offset: u32 = 0x54u32;
    const ahb3lpenr_offset: u32 = 0x58u32;
    const apb1lpenr_offset: u32 = 0x60u32;
    const apb2lpenr_offset: u32 = 0x64u32;
    const bdcr_offset: u32 = 0x70u32;
    const csr_offset: u32 = 0x74u32;
    const sscgr_offset: u32 = 0x80u32;
    const ppli2scfgr_offset: u32 = 0x84u32;
    const pllsaicfgr_offset: u32 = 0x88u32;
    const dckcfgr_offset: u32 = 0x8Cu32;
}
