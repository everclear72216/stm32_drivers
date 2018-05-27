use stm32f429::Stm32F429;

use stm32_drivers::rcc::Rcc;

#[no_mangle]
#[export_name = "system_init"]
pub unsafe extern "C" fn system_init() {

}

unsafe fn set_system_clock() -> () {}
