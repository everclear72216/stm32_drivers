use rcc::traits::Rcc;
use rcc::traits::RccCrHsi;

use bitbanding::range::Range;
use bitbanding::traits::PeripheralBitbanding;

use parts::stm32::f4::Stm32F429 as Stm32F429Trait;

pub struct Stm32F429 {}

impl Stm32F429Trait for Stm32F429 {}

impl Rcc for Stm32F429 {
    const RCC: u32 = 0x4002_3800;
}

impl RccCrHsi for Stm32F429 {
    const RCC_CR_HSION_BIT: u8 = 0x00;
    const RCC_CR_HSIRDY_BIT: u8 = 0x01;
    const RCC_CR_HSITRIM_MASK: u32 = 0x0000_00FC;
    const RCC_CR_HSITRIM_OFFS: u8 = 0x02;
    const RCC_CR_HSICAL_MASK: u32 = 0x0000_FF00;
    const RCC_CR_HSICAL_OFFS: u8 = 0x07;
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
