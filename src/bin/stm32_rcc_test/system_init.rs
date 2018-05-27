
use stm32f429::Stm32F429;
use stm32_drivers::rcc::driver::Driver;

#[no_mangle]
#[export_name = "system_init"]
pub unsafe extern "C" fn system_init() {
    
    let mut rcc = Driver::<Stm32F429>::new();
    rcc.deinit();

    set_system_clock();
}

unsafe fn set_system_clock() -> () {}
