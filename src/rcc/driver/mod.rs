mod stm32f429;

use rcc::regs::Rcc as RccReg;
use rcc::traits::Rcc as RccTrait;

use bitbanding::traits::PeripheralBitbanding;

pub struct Driver<T>
where
    T: RccTrait,
{
    rcc: RccReg<T>,
}

impl<T> Driver<T>
where
    T: RccTrait + PeripheralBitbanding,
{
    pub unsafe fn new() -> Driver<T> {
        Driver {
            rcc: RccReg::<T>::new(),
        }
    }
}
