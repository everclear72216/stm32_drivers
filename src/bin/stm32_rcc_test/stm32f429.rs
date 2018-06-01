use stm32_drivers::rcc::traits::HasRcc;

#[derive(Default)]
pub struct Stm32F429 {}

impl HasRcc for Stm32F429 {
    const RCC: u32= 0x4002_3800;
}
