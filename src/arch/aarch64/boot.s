.macro ABSOLUTE_SYMBOL register, symbol
  adrp \register, \symbol
  add  \register, \register, #:lo12:\symbol
.endm

.section .text._start

_start:
  ABSOLUTE_SYMBOL x0, __bss_start
  ABSOLUTE_SYMBOL x1, __bss_end
.L_bss_initialize:
  cmp x0, x1
  b.eq .L_init
  // This stores 16 bytes of zeroes at [x0] and then increments x0 by 16.
  stp xzr, xzr, [x0], #16
  b .L_bss_initialize
.L_init:
  ABSOLUTE_SYMBOL x0, __boot_core_stack_end
  mov sp, x0
  bl _init_os
.L_boot:
  b _start_os

.size _start, . - _start
.type _start, function
.global _start
