

use core::default::Default;

use stm32f429::Stm32F429;

use stm32_drivers::rcc::Rcc;

#[no_mangle]
#[export_name = "system_init"]
pub unsafe extern "C" fn system_init() {

    let mut rcc: Rcc<Stm32F429> = Default::default();

    rcc.cr.set(|v| v | 0x0000_0001);
    rcc.cfgr.set(|_v| 0x0000_0000);
    rcc.cr.set(|v| v & 0xFEF6_FFFF);
    rcc.pllcfgr.set(|_v| 0x2400_3010);
    rcc.cr.set(|v| v & 0xFFFB_FFFF);
    rcc.cir.set(|_v| 0x0000_0000);

    set_system_clock();
}

unsafe fn set_system_clock() -> () {

}
