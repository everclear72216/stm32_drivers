use bitbanding::range::Range;

pub trait SramBitbanding {
    const SRAM_BITBAND: Range;
    const SRAM_ALIAS: Range;
}

pub trait PeripheralBitbanding {
    const PERIPHERAL_BITBAND: Range;
    const PERIPHERAL_ALIAS: Range;
}
