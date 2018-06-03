use stm32_drivers::rcc::driver::Driver;
use stm32_drivers::rcc::traits::ClockKind;
use stm32_drivers::rcc::traits::ExternalClock;
use stm32_drivers::rcc::traits::SystemClock;

use stm32_drivers::parts::stm32::f4::stm32f429::Stm32F429;

#[no_mangle]
#[export_name = "system_init"]
pub unsafe extern "C" fn system_init() {
    let mut rcc = Driver::<Stm32F429>::new();
    let clock = ClockKind::Hse(ExternalClock {
        ext_clock_frequency: 8_000_000,
        core_clock_frequency: 180_000_000,
    });

    rcc.system_clock_deinit();
    rcc.system_clock_init(&clock);
}
