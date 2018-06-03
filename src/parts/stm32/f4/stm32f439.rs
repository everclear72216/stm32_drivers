use rcc::traits::Rcc;
use rcc::traits::Hsi;

use bitbanding::range::Range;
use bitbanding::traits::PeripheralBitbanding;

use parts::stm32::f4:Stm32F439 as Stm32F439Trait;

pub struct Stm32F439 {}

pub trait Stm32F439Trait {}

impl Stm32F439Trait for Stm32F439 {}

impl Rcc for Stm32F439 {
    const RCC: u32 = 0x4002_3800;
}

impl RccCrHsi for Stm32F439 {
    const RCC_HSION_BIT: u8 = 0x00;
    const RCC_HSIRDY_BIT: u8 = 0x01;
    const RCC_HSITRIM_MASK: u32 = 0x0000_00FC;
    const RCC_HSITRIM_OFFS: u8 = 0x02;
    const RCC_HSICAL_MASK: u32 = 0x0000_FF00;
    const RCC_HSICAL_OFFS: u8 = 0x07;
}

impl PeripheralBitbanding for Stm32F439 {
    const PERIPHERAL_BITBAND: Range = Range {
        start: 0x2000_0000,
        end: 0x2200_0000,
    };

    const PERIPHERAL_ALIAS: Range = Range {
        start: 0x4000_0000,
        end: 0x4200_0000,
    };
}
