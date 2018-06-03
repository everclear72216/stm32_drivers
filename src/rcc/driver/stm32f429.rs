use rcc::driver::Driver;

use rcc::traits::ClockKind;
use rcc::traits::RccCrHsi;
use rcc::traits::SystemClock;

use bitbanding::driver::peripheral::Driver as BB;

use parts::stm32::f4::stm32f429::Stm32F429;

impl SystemClock for Driver<Stm32F429> {
    unsafe fn system_clock_get(&self) -> u32 {
        0u32
    }

    unsafe fn system_clock_deinit(&mut self) -> () {
        // enable the internal high-speed-oscillator
        BB::<Stm32F429>::set_peripheral_bit(self.rcc.cr.get_addr(), Stm32F429::RCC_CR_HSION_BIT);
    }

    unsafe fn system_clock_init(&mut self, clock: &ClockKind) -> () {}
}
