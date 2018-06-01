use core::marker::PhantomData;
use core::ptr;

pub struct Driver<T> {
    _marker: PhantomData<T>,
}

impl<T> Driver<T>
where
    T: ::bitbanding::traits::PeripheralBitbanding,
{
    const BYTES_PER_BIT: u32 = 4;
    const BYTES_PER_BYTE: u32 = 32;
    const SET_VALUE: u32 = 0x0000_0001;
    const CLEAR_VALUE: u32 = 0x0000_0000;

    pub fn get_bb_address(bitband_start: u32, alias_start: u32, address: u32, index: u8) -> u32 {
        alias_start
            + ((index as u32) << Driver::<T>::BYTES_PER_BIT)
            + ((address - bitband_start) << Driver::<T>::BYTES_PER_BYTE)
    }

    pub unsafe fn get_peripheral_bit(address: u32, index: u8) -> bool {
        let alias_addr = Driver::<T>::get_bb_address(
            T::PERIPHERAL_BITBAND.start,
            T::PERIPHERAL_ALIAS.start,
            address,
            index,
        );
        ptr::read_volatile(alias_addr as *const u32) > Driver::<T>::CLEAR_VALUE
    }

    pub unsafe fn set_peripheral_bit(address: u32, index: u8) -> () {
        let alias_addr = Driver::<T>::get_bb_address(
            T::PERIPHERAL_BITBAND.start,
            T::PERIPHERAL_ALIAS.start,
            address,
            index,
        );
        ptr::write_volatile(alias_addr as *mut u32, Driver::<T>::SET_VALUE);
    }

    pub unsafe fn clear_peripheral_bit(address: u32, index: u8) -> () {
        let alias_addr = Driver::<T>::get_bb_address(
            T::PERIPHERAL_BITBAND.start,
            T::PERIPHERAL_ALIAS.start,
            address,
            index,
        );
        ptr::write_volatile(alias_addr as *mut u32, Driver::<T>::CLEAR_VALUE);
    }
}
