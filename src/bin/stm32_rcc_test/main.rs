#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate stm32_drivers;

mod exceptions;
mod lang;
mod startup;
mod system_init;

pub use lang::panic_fmt;
pub use startup::reset_handler;
pub use startup::__RESET_VECTOR;
pub use system_init::system_init;

pub use exceptions::default_handler;
pub use exceptions::__EXCEPTIONS;
pub use exceptions::__INTERRUPTS;

#[no_mangle]
#[export_name = "main"]
pub unsafe extern "C" fn main() -> ! {
    loop {}
}
