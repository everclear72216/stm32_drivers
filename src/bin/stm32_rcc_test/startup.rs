
use core::ptr;
use core::mem;

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = reset_handler;

#[no_mangle]
#[export_name = "reset_handler"]
pub unsafe extern "C" fn reset_handler() -> ! {

    extern "C" {

        fn main() -> !;

        fn system_init() -> ();

        static mut __ebss: u32;
        static mut __sbss: u32;

        static mut __sdata: u32;
        static mut __edata: u32;
        static mut __sidata: u32;
    }

    #[inline(never)]
    unsafe fn system_init_trampoline() -> () {

        system_init();
    }

    system_init_trampoline();

    // zero the .bss segment
    {
        let mut sbss: *mut u32 = &mut __sbss;
        let ebss: *mut u32 = &mut __ebss;

        while sbss < ebss {

            ptr::write_volatile(sbss, mem::zeroed());
            sbss = sbss.offset(1);
        }
    }

    // initialize the .data segment
    {
        let edata: *mut u32 = &mut __edata;
        let mut sdata: *mut u32 = &mut __sdata;
        let mut sidata: *mut u32 = &mut __sidata;

        while sdata < edata {

            ptr::write(sdata, ptr::read(sidata));
            sdata = sdata.offset(1);
            sidata = sidata.offset(1);
        }
    }

    #[inline(never)]
    unsafe fn main_trampoline() -> ! {

        main();
    }

    main_trampoline();
}
