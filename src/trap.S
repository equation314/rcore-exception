.intel_syntax noprefix
.globl _alltraps

_alltraps:
    jmp [rip + _RUST_TRAP_FN]
    ret
