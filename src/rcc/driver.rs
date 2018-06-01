use rcc::regs::Rcc;
use rcc::traits::HasRcc;

pub struct Driver<T>
where
    T: HasRcc,
{
    rcc: Rcc<T>,
}

impl<T> Driver<T>
where
    T: HasRcc,
{
    pub unsafe fn new() -> Driver<T> {
        Driver {
            rcc: Rcc::<T>::new(),
        }
    }

    pub unsafe fn deinit(&mut self) -> () {

    }
}
