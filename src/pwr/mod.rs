mod regs;

pub trait Pwr {
    const PWR: u32;
    const PWR_CR_RESET_VALUE: u32;
    const PWR_CSR_RESET_VALUE: u32;
}
