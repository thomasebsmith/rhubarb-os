.macro ABSOLUTE_SYMBOL register, symbol
  adrp \register, \symbol
  add  \register, \register, #:lo12:\symbol
.endm

// Raspberry Pis have 4 cores. The last 2 bits of MPIDR_EL1 distinguish these
// cores. The kernel starts on core 0.
.equ _cpu_core_affinity_mask, 0b11
.equ _boot_core_id, 0

.section .text._start

_start:
  mrs x1, MPIDR_EL1
  and x1, x1, _cpu_core_affinity_mask
  cmp x1, _boot_core_id
  b.ne .L_wfe
  ABSOLUTE_SYMBOL x0, __bss_start
  ABSOLUTE_SYMBOL x1, __bss_end
.L_bss_initialize:
  cmp x0, x1
  b.eq .L_boot
  // This stores 16 bytes of zeroes at [x0] and then increments x0 by 16.
  stp xzr, xzr, [x0], #16
  b .L_bss_initialize
.L_boot:
  ABSOLUTE_SYMBOL x0, __boot_core_stack_end
  mov sp, x0
  b _start_os

.L_wfe:
  wfe
  b .L_wfe

.size _start, . - _start
.type _start, function
.global _start
