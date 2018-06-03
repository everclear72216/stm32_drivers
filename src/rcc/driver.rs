use models::Stm32F427xx;
use models::Stm32F429xx;

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

impl<T> Driver<T>
where
    T: RccTrait + PeripheralBitbanding + Stm32F429xx
{
    pub unsafe fn deinit(&mut self) -> () {}
}

impl<T> Driver<T>
where
    T: RccTrait + PeripheralBitbanding + Stm32F427xx
{
    pub unsafe fn deinit(&mut self) -> () {}
}
