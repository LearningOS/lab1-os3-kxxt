mod context;

use core::arch::global_asm;

pub use context::TrapContext;
use riscv::register::{stvec, utvec::TrapMode};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}
