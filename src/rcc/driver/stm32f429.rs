use rcc::driver::Driver;

use rcc::traits::Rcc;
use rcc::traits::RccCrCss;
use rcc::traits::RccCrHse;
use rcc::traits::RccCrHsi;
use rcc::traits::RccCrPll;
use rcc::traits::RccCfgrSw;
use rcc::traits::ClockKind;
use rcc::traits::SystemClock;

use bitbanding::driver::peripheral::Driver as BB;

use parts::stm32::f4::stm32f429::Stm32F429;

impl SystemClock for Driver<Stm32F429> {
    unsafe fn system_clock_get(&self) -> u32 {
        0u32
    }

    unsafe fn system_clock_deinit(&mut self) -> () {
        let rcc_cr_addr = self.rcc.cr.get_addr();
        // enable the internal high speed oscillator
        BB::<Stm32F429>::set_peripheral_bit(rcc_cr_addr, Stm32F429::RCC_CR_HSION_BIT);
        // reset clock configuration register - selects internal high speed oscillator
        self.rcc.cfgr.set(Stm32F429::RCC_CFGR_RESET_VALUE);
        // wait for internal high speed oscillator to be selected
        while Stm32F429::RCC_CFGR_SW_SWS_HSI
            != self
                .rcc
                .cfgr
                .get_field(Stm32F429::RCC_CFGR_SWS_MASK, Stm32F429::RCC_CFGR_SWS_OFFS)
        {}
        // disable external oscillator, clock security and pll
        BB::<Stm32F429>::clear_peripheral_bit(rcc_cr_addr, Stm32F429::RCC_CR_HSEON_BIT);
        BB::<Stm32F429>::clear_peripheral_bit(rcc_cr_addr, Stm32F429::RCC_CR_CSSON_BIT);
        BB::<Stm32F429>::clear_peripheral_bit(rcc_cr_addr, Stm32F429::RCC_CR_PLLON_BIT);
        // reset the pll configuration register
        self.rcc.pllcfgr.set(Stm32F429::RCC_PLLCFGR_RESET_VALUE);
        // disable external high speed oscillator bypassing
        BB::<Stm32F429>::clear_peripheral_bit(rcc_cr_addr, Stm32F429::RCC_CR_HSEBYP_BIT);
        // disable all RCC interrupts
        self.rcc.cir.set(Stm32F429::RCC_CIR_RESET_VALUE);
    }

    unsafe fn system_clock_init(&mut self, clock: &ClockKind) -> () {
        let rcc_cr_addr = self.rcc.cr.get_addr();
        match clock {
            ClockKind::Hse(ext_clock) => {
                // enable the external high speed oscillator
                BB::<Stm32F429>::set_peripheral_bit(rcc_cr_addr, Stm32F429::RCC_CR_HSEON_BIT);
                // wait for external high speed oscillator to become ready
                while BB::<Stm32F429>::get_peripheral_bit(rcc_cr_addr, Stm32F429::RCC_CR_HSERDY_BIT) {}
                
            }
            ClockKind::Hsi(_int_clock) => {

            }
        }
    }
}
