.altmacro
# TaskContext: ra, sp, s0~11
.macro SAVE_SN n
    sd s\n, (\n+2)*8(sp)
.endm

.macro LOAD_SN n
    ld s\n, (\n+2)*8(sp)
.endm

    .section .text
    .globl __switch
__switch:
    # __switch(current: *mut TaskContext, next: *const TaskContext)
    # store kernel stack of current task

    # save ra
    sd ra, 0(a0)
    # save sp
    sd sp, 8(a0)

    # save callee saved registers
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr

    # restore ra
    ld ra, 0(a1)
    # restore sp
    ld sp, 8(a1)
    # restore callee saved regs
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n+1
    .endr
    
    ret