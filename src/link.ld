__entry = 0x80000;

ENTRY(__entry)

PHDRS {
  rx_segment PT_LOAD FLAGS(0x4 | 0x1); /* Readable and executable */
  rw_segment PT_LOAD FLAGS(0x4 | 0x2); /* Readable and writable */
}

SECTIONS {
  . = __entry;

  __boot_core_stack_end = .;

  .text : {
    /* Always include _start since it is the entry point. */
    KEEP(*(.text._start))
    *(.text._start_os)
    *(.text*)
  } :rx_segment

  .rodata : ALIGN(8) { *(.rodata*) } :rx_segment
  .got    : ALIGN(8) { *(.got)     } :rx_segment

  .data : { *(.data*) } :rw_segment

  /* Needs to be aligned to 16-byte segments to work with bss initialization in
   * boot.s. */
  .bss : ALIGN(16) {
    __bss_start = .;
    *(.bss*);
    . = ALIGN(16);
    __bss_end = .;
  } :NONE
}
