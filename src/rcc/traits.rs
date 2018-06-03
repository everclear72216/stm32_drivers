pub trait Rcc {
    const RCC: u32;
    const RCC_CR_RESET_VALUE: u32;
    const RCC_CFGR_RESET_VALUE: u32;
    const RCC_PLLCFGR_RESET_VALUE: u32;
    const RCC_CIR_RESET_VALUE: u32;
}

pub trait RccCrHsi {
    const RCC_CR_HSION_BIT: u8;
    const RCC_CR_HSIRDY_BIT: u8;
    const RCC_CR_HSITRIM_MASK: u32;
    const RCC_CR_HSITRIM_OFFS: u8;
    const RCC_CR_HSICAL_MASK: u32;
    const RCC_CR_HSICAL_OFFS: u8;
}

pub trait RccCrHse {
    const RCC_CR_HSEON_BIT: u8;
    const RCC_CR_HSERDY_BIT: u8;
    const RCC_CR_HSEBYP_BIT: u8;
}

pub trait RccCrCss {
    const RCC_CR_CSSON_BIT: u8;
}

pub trait RccCrPll {
    const RCC_CR_PLLON_BIT: u8;
    const RCC_CR_PLLRDY_BIT: u8;
}

pub trait RccCrPllI2s {
    const RCC_CR_PLLI2SON_BIT: u8;
    const RCC_CR_PLLI2SRDY_BIT: u8;
}

pub trait RccCrPllSai {
    const RCC_CR_PLLSAION_BIT: u8;
    const RCC_CR_PLLSAIRDY_BIT: u8;
}

pub trait RccCfgrSw {
    const RCC_CFGR_SW_MASK: u32;
    const RCC_CFGR_SW_OFFS: u8;
    const RCC_CFGR_SWS_MASK: u32;
    const RCC_CFGR_SWS_OFFS: u8;
    const RCC_CFGR_SW_SWS_HSI: u32;
    const RCC_CFGR_SW_SWS_HSE: u32;
    const RCC_CFGR_SW_SWS_PLL: u32;
}

pub struct InternalClock {
    pub core_clock_frequency: i32,
}

pub struct ExternalClock {
    pub ext_clock_frequency: i32,
    pub core_clock_frequency: i32,
}

pub enum ClockKind {
    Hsi(InternalClock),
    Hse(ExternalClock),
}

pub trait SystemClock {
    unsafe fn system_clock_get(&self) -> u32;
    unsafe fn system_clock_deinit(&mut self) -> ();
    unsafe fn system_clock_init(&mut self, clock: &ClockKind) -> ();
}
