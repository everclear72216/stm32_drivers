pub struct Driver<T>
where
    T: HasRcc,
{
    rcc: ::rcc::regs::Rcc<T>,
}

impl<T> Driver<T>
where
    T: rcc::traits::HasRcc + bitbanding::traits::PeripheralBitbanding,
{
    pub unsafe fn new() -> Driver<T> {
        Driver {
            rcc: Rcc::<T>::new(),
        }
    }

    pub unsafe fn deinit(&mut self) -> () {}
}
