use rcc::traits::Rcc;
use rcc::traits::RccCfgrSw;
use rcc::traits::RccCrCss;
use rcc::traits::RccCrHse;
use rcc::traits::RccCrHsi;
use rcc::traits::RccCrPll;

use bitbanding::range::Range;
use bitbanding::traits::PeripheralBitbanding;

use parts::stm32::f4::Stm32F429 as Stm32F429Trait;

pub struct Stm32F429 {}

impl Stm32F429Trait for Stm32F429 {}

impl Rcc for Stm32F429 {
    const RCC: u32 = 0x4002_3800;
    const RCC_CR_RESET_VALUE: u32 = 0x0000_0083;
    const RCC_PLLCFGR_RESET_VALUE: u32 = 0x2400_3010;
    const RCC_CFGR_RESET_VALUE: u32 = 0x0000_0000;
    const RCC_CIR_RESET_VALUE: u32 = 0x0000_0000;
}

impl RccCrHsi for Stm32F429 {
    const RCC_CR_HSION_BIT: u8 = 0x00;
    const RCC_CR_HSIRDY_BIT: u8 = 0x01;
    const RCC_CR_HSITRIM_MASK: u32 = 0x0000_00FC;
    const RCC_CR_HSITRIM_OFFS: u8 = 0x02;
    const RCC_CR_HSICAL_MASK: u32 = 0x0000_FF00;
    const RCC_CR_HSICAL_OFFS: u8 = 0x07;
}

impl RccCrHse for Stm32F429 {
    const RCC_CR_HSEON_BIT: u8 = 0x10;
    const RCC_CR_HSERDY_BIT: u8 = 0x11;
    const RCC_CR_HSEBYP_BIT: u8 = 0x12;
}

impl RccCrCss for Stm32F429 {
    const RCC_CR_CSSON_BIT: u8 = 0x13;
}

impl RccCrPll for Stm32F429 {
    const RCC_CR_PLLON_BIT: u8 = 0x18;
    const RCC_CR_PLLRDY_BIT: u8 = 0x19;
}

impl RccCfgrSw for Stm32F429 {
    const RCC_CFGR_SW_MASK: u32 = 0x0000_0003;
    const RCC_CFGR_SW_OFFS: u8 = 0x00;
    const RCC_CFGR_SWS_MASK: u32 = 0x0000_000C;
    const RCC_CFGR_SWS_OFFS: u8 = 0x02;
    const RCC_CFGR_SW_SWS_HSI: u32 = 0x0000_0000;
    const RCC_CFGR_SW_SWS_HSE: u32 = 0x0000_0001;
    const RCC_CFGR_SW_SWS_PLL: u32 = 0x0000_0002;
}

impl PeripheralBitbanding for Stm32F429 {
    const PERIPHERAL_BITBAND: Range = Range {
        start: 0x2000_0000,
        end: 0x2200_0000,
    };

    const PERIPHERAL_ALIAS: Range = Range {
        start: 0x4000_0000,
        end: 0x4200_0000,
    };
}
