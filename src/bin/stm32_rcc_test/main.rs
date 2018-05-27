
#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate stm32_drivers;

mod lang;
mod startup;
mod exceptions;
mod stm32f429;
mod system_init;

pub use startup::reset_handler;
pub use startup::__RESET_VECTOR;
pub use system_init::system_init;

pub use exceptions::default_handler;
pub use exceptions::__INTERRUPTS;
pub use exceptions::__EXCEPTIONS;

#[no_mangle]
#[export_name = "main"]
pub unsafe extern "C" fn main() -> ! {

    loop { }
}
