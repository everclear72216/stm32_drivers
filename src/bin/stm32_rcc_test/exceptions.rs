
pub union ExceptionVector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

extern "C" {
    fn nmi_handler();
    fn hard_fault_handler();
    fn memory_management_handler();
    fn bus_fault_handler();
    fn usage_fault_handler();
    fn svcall_handler();
    fn debug_monitor_handler();
    fn pendsv_handler();
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
