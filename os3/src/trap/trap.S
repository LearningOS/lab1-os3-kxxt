.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    # Exchange sp and sscratch,
    # so that user stack pointer goes into sscratch,
    # sp points to kernel stack after exchange
    csrrw sp, sscratch, sp
    # allocate space for a TrapContext (32+2) * sizeof usize
    addi sp, sp, -34*8
    # save general purpose registers
    sd x1, 8(sp)
    # x2 is sp, deal with it later
    sd x3, 3*8(sp)
    # skip tp(x4), apps do not use thread pointer
    # save x5~x31
    # use macro to simplify code
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we have saved t0, t1. 
    # save current sstatus, sepc
    csrr t0, sstatus
    csrr t1, sepc
    # TrapContext < registers: [0*8,31*8], sstatus: 32*8, sepc: 33*8 >
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # Read the addr of user stack from sscratch
    csrr t2, sscratch
    # sp is x2
    sd t2, 2*8(sp)
    # Now, we have saved all user state.
    # Use the TrapContext as the first parameter 
    mv a0, sp
    call trap_handler

__restore:
    # we do not need to get sp from a0 because sp is restored in __switch
    # case1: start running app by __restore
    # case2: back to U after handling trap
    # mv sp, a0
    # restore sstatus, sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sepc, t1
    csrw sstatus, t0
    # set sscratch to user stack
    ld t2, 2*8(sp)
    # ??? Maybe sscratch is changed in trap_handler
    csrw sscratch, t2
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release the TrapContext struct
    addi sp, sp, 34*8
    csrrw sp, sscratch, sp
    # return to U mode
    sret
