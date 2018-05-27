
pub union ExceptionVector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

extern "C" {
    #[allow(dead_code)]
    fn nmi_handler();

    #[allow(dead_code)]
    fn hard_fault_handler();

    #[allow(dead_code)]
    fn memory_management_handler();

    #[allow(dead_code)]
    fn bus_fault_handler();

    #[allow(dead_code)]
    fn usage_fault_handler();

    #[allow(dead_code)]
    fn svcall_handler();

    #[allow(dead_code)]
    fn debug_monitor_handler();

    #[allow(dead_code)]
    fn pendsv_handler();

    #[allow(dead_code)]
    fn sys_tick_handler();
}

#[no_mangle]
pub unsafe extern "C" fn default_handler() {

    loop { }
}

#[no_mangle]
#[link_section = ".vector_table.exceptions"]
pub static __EXCEPTIONS: [ExceptionVector; 14] = [
    ExceptionVector { handler:      nmi_handler                 },
    ExceptionVector { handler:      hard_fault_handler          },
    ExceptionVector { handler:      memory_management_handler   },
    ExceptionVector { handler:      bus_fault_handler           },
    ExceptionVector { handler:      usage_fault_handler         },
    ExceptionVector { reserved:     0                           },
    ExceptionVector { reserved:     0                           },
    ExceptionVector { reserved:     0                           },
    ExceptionVector { reserved:     0                           },
    ExceptionVector { handler:      svcall_handler              },
    ExceptionVector { handler:      debug_monitor_handler       },
    ExceptionVector { reserved:     0                           },
    ExceptionVector { handler:      pendsv_handler              },
    ExceptionVector { handler:      sys_tick_handler            },
];

#[no_mangle]
#[link_section = ".vector_table.interrupts"]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 240] = [{
    default_handler
}; 240];
