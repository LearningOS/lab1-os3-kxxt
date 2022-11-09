mod context;

use core::arch::global_asm;

pub use context::TrapContext;
use riscv::register::{scause, stval, stvec, utvec::TrapMode};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    use scause::Exception::*;
    use scause::Trap::Exception;
    match scause.cause() {
        Exception(UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Exception(StoreFault) | Exception(StorePageFault) => {
            error!("[kernel] Store(Page)Fault in application, core dumped.");
            run_next_app();
        }
        Exception(IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, core dumped.");
            run_next_app();
        }
        _ => panic!(
            "Unsupported trap {:?}, stval = {:#x}!",
            scause.cause(),
            stval
        ),
    }
    cx
}
