use super::context::TaskContext;
use core::arch::global_asm;


global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(current: *mut TaskContext, next: *const TaskContext);
}
