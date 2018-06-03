use stm32_drivers::rcc::traits::Rcc;

use stm32_drivers::bitbanding::range::Range;
use stm32_drivers::bitbanding::traits::PeripheralBitbanding;

pub struct Stm32F429 {}

impl Rcc for Stm32F429 {
    const RCC: u32 = 0x4002_3800;
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
