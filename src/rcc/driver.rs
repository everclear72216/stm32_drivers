
use rcc::ll::Rcc;
use rcc::traits::HasRcc;

pub struct Driver<T>
where
    T: HasRcc
{
    rcc: Rcc<T>,
}

impl<T> Driver<T> 
where
    T: HasRcc
{
    pub unsafe fn new() -> Driver<T> {
        Driver { rcc: Rcc::<T>::new() }
    }

    pub unsafe fn deinit(&mut self) -> () {
        self.rcc.cr.set(|v| v | 0x0000_0001);
        self.rcc.cfgr.set(|_v| 0x0000_0000);
        self.rcc.cr.set(|v| v & 0xEAF6FFFF);
        self.rcc.pllcfgr.set(|_v| 0x2400_3010);
        self.rcc.plli2scfgr.set(|_v| 0x2000_3000);
        self.rcc.pllsaicfgr.set(|_v| 0x2400_3000);
        self.rcc.cr.set(|v| v & 0xFFFB_FFFF);
        self.rcc.cir.set(|_v| 0x0000_0000);
        self.rcc.dckcfgr.set(|_v| 0x0000_0000);
    }
}
