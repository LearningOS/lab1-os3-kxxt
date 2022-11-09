mod context;

use crate::{syscall::syscall, task::{suspend_current_and_run_next, exit_current_and_run_next}, timer::set_next_trigger};
pub use context::TrapContext;
use core::arch::global_asm;
use riscv::register::{scause, sie, stval, stvec, utvec::TrapMode};

global_asm!(include_str!("trap.S"));

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    use scause::Exception::*;
    use scause::Interrupt::*;
    use scause::Trap::Exception;
    use scause::Trap::Interrupt;
    match scause.cause() {
        Exception(UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Exception(StoreFault) | Exception(StorePageFault) => {
            error!("[kernel] Store(Page)Fault in application, core dumped.");
            exit_current_and_run_next();
        }
        Exception(IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, core dumped.");
            exit_current_and_run_next();
        }
        Interrupt(SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => panic!(
            "Unsupported trap {:?}, stval = {:#x}!",
            scause.cause(),
            stval
        ),
    }
    cx
}
