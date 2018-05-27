
#![no_std]
#![no_main]
#![feature(lang_items)]

mod lang;
mod startup;
mod exceptions;
mod system_init;

pub use startup::reset_handler;
pub use system_init::system_init;

pub use exceptions::default_handler;

#[no_mangle]
#[export_name = "main"]
pub unsafe extern "C" fn main() -> ! {

    loop { }
}
