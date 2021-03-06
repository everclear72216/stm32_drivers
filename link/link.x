/* # Developer notes

- Symbols that start with a double underscore (__) are considered "private"

- Symbols that start with a single underscore (_) are considered "semi-public"; they can be
  overridden in a user linker script, but should not be referred from user code (e.g. `extern "C" {
  static mut __sbss }`).

- `EXTERN` forces the linker to keep a symbol in the final binary. We use this to make sure a
  symbol if not dropped if it appears in or near the front of the linker arguments and "it's not
  needed" by any of the preceding objects (linker arguments)

- `PROVIDE` is used to provide default values that can be overridden by a user linker script

- On alignment: it's important for correctness that the VMA boundaries of both .bss and .data *and*
  the LMA of .data are all 4-byte aligned. These alignments are assumed by the RAM initialization
  routine. There's also a second benefit: 4-byte aligned boundaries means that you won't see
  "Address (..) is out of bounds" in the disassembly produced by `objdump`.
*/

/* Provides information about the memory layout of the device */
/* This will be provided by the user (see `memory.x`) or by a Board Support Crate */
INCLUDE memory.x

/* # Entry point = reset vector */
ENTRY(reset_handler);
EXTERN(__RESET_VECTOR); /* depends on the `reset_handler` symbol */

/* # Exception vectors */
/* This is effectively weak aliasing at the linker level */
/* The user can override any of these aliases by defining the corresponding symbol themselves (cf.
   the `exception!` macro) */
EXTERN(__EXCEPTIONS); /* depends on all the these PROVIDED symbols */

EXTERN(default_handler);

PROVIDE(nmi_handler = default_handler);
PROVIDE(hard_fault_handler = default_handler);
PROVIDE(memory_management_handler = default_handler);
PROVIDE(bus_fault_handler = default_handler);
PROVIDE(usage_fault_handler = default_handler);
PROVIDE(secure_fault_handler = default_handler);
PROVIDE(svcall_handler = default_handler);
PROVIDE(debug_monitor_handler = default_handler);
PROVIDE(pendsv_handler = default_handler);
PROVIDE(sys_tick_handler = default_handler);

/* Provides weak aliases (cf. PROVIDED) for device specific interrupt handlers */
/* INCLUDE device.x" */

/* # Interrupt vectors */
EXTERN(__INTERRUPTS); /* `static` variable similar to `__EXCEPTIONS` */

/* # User overridable symbols I */
/* Lets the user place the stack in a different RAM region */
PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

/* # Sections */
SECTIONS
{
  /* ## Sections in FLASH */
  /* ### Vector table */
  .vector_table ORIGIN(FLASH) : ALIGN(4)
  {
    /* Initial Stack Pointer (SP) value */
    __STACK_START = .; /* Just to get a nicer name in the disassembly */
    LONG(_stack_start);

    /* Reset vector */
    KEEP(*(.vector_table.reset_vector)); /* this is `__RESET_VECTOR` symbol */
    __reset_vector = ABSOLUTE(.);

    /* Exceptions */
    KEEP(*(.vector_table.exceptions)); /* this is `__EXCEPTIONS` symbol */
    __eexceptions = ABSOLUTE(.);

    /* Device specific interrupts */
    KEEP(*(.vector_table.interrupts)); /* this is `__INTERRUPTS` symbol */
    __einterrupts = ABSOLUTE(.);
  } > FLASH

  /* ### .text */
  .text _stext :
  {
    *(.text .text.*);
    __etext = ABSOLUTE(.);
  } > FLASH

  /* ### .rodata */
  .rodata :
  {
    . = ALIGN(4); /* 4-byte align the start (VMA) of this section */
    /* __srodata = ABSOLUTE(.); */

    *(.rodata .rodata.*);

    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __erodata = ABSOLUTE(.);
  } > FLASH

  /* ## Sections in RAM */
  /* ### .data */
  .data : AT(__erodata) /* LMA */
  {
    . = ALIGN(4); /* 4-byte align the start (VMA) of this section */
    __sdata = ABSOLUTE(.);

    *(.data .data.*);

    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __edata = ABSOLUTE(.);
  } > RAM

  /* ### .bss */
  .bss :
  {
    . = ALIGN(4); /* 4-byte align the start (VMA) of this section */
    __sbss = ABSOLUTE(.);

    *(.bss .bss.*);

    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __ebss = ABSOLUTE(.);
  } > RAM

  /* ## Fake output .got section */
  /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
     the input files and raise an error if relocatable code is found */
  .got :
  {
    __sgot = ABSOLUTE(.);
    KEEP(*(.got .got.*));
    __egot = ABSOLUTE(.);
  } > FLASH

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx.*);
  }
}

/* # User overridable symbols II */
/* (The user overridable symbols are split in two parts because LLD demands that the RHS of PROVIDE
    to be defined before the PROVIDE invocation) */
/* Lets the user override this to place .text a bit further than the vector table. Required by
microcontrollers that store their configuration right after the vector table. */
PROVIDE(_stext = __einterrupts);

/* # Hardcoded symbols */
/* Place `.bss` at the start of the RAM region */
__sidata = LOADADDR(.data);
/* Place the heap right after `.bss` and `.data` */
__sheap = __ebss;

/* # Sanity checks */

ASSERT(__reset_vector == ORIGIN(FLASH) + 0x8, "
cortex-m-rt: The reset vector is missing. This is a bug in cortex-m-rt. Please file a bug
report at: https://github.com/japaric/cortex-m-rt/issues");

ASSERT(__eexceptions - ORIGIN(FLASH) == 0x40, "
cortex-m-rt: The exception vectors are missing. This is a bug in cortex-m-rt. Please file
a bug report at: https://github.com/japaric/cortex-m-rt/issues");

ASSERT(__sheap >= __ebss, "
cortex-m-rt: The heap overlaps with the .bss section. This is a bug in cortex-m-rt. Please
file a bug report at: https://github.com/japaric/cortex-m-rt/issues");

ASSERT(__sheap >= __edata, "
cortex-m-rt: The heap overlaps with the .data section. This is a bug in cortex-m-rt.
Please file a bug report at: https://github.com/japaric/cortex-m-rt/issues");

ASSERT(__einterrupts - __eexceptions > 0, "
cortex-m-rt: The interrupt vectors are missing. Possible solutions, from most likely to
less likely:
- Link to a device crate
- Disable the 'device' feature of cortex-m-rt to build a generic application (a dependency
  may be enabling it)
- Supply the interrupt handlers yourself. Check the documentation for details.");

ASSERT(__einterrupts <= _stext, "
cortex-m-rt: The '.text' section can't be placed inside the '.vector_table' section. Set
'_stext' to an address greater than '__einterrupts' (cf. `nm` output)");

ASSERT(_stext < ORIGIN(FLASH) + LENGTH(FLASH), "
cortex-m-rt The '.text' section must be placed inside the FLASH memory. Set '_stext' to an
address smaller than 'ORIGIN(FLASH) + LENGTH(FLASH)");

ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, "
.bss is not 4-byte aligned at its boundaries. This is a cortex-m-rt bug.");

ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, "
.data is not 4-byte aligned at its boundaries. This is a cortex-m-rt bug.");

ASSERT(__sidata % 4 == 0, "
__sidata is not 4-byte aligned. This is a cortex-m-rt bug.");

ASSERT(__sgot == __egot, "
.got section detected in the input object files. Dynamic relocations are not supported.
If you are linking to C code compiled using the `cc` crate then modify your build script
to compile the C code _without_ the -fPIC flag. See the documentation of the
`cc::Build.pic` method for details.");

ASSERT(__einterrupts - __eexceptions <= 0x3C0, "
There can't be more than 240 interrupt handlers. This may be a bug in
your device crate, or you may have registered more than 240 interrupt
handlers.");
