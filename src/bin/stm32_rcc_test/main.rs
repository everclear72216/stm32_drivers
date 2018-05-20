
#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;

pub mod exceptions;

use core::fmt::Write;
use cortex_m_semihosting::hio;

entry!(main);

fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();

    loop {}
}
