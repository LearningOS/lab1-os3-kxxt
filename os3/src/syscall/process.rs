use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::{get_time, get_time_us},
};

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable code in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    unsafe {
        *ts = TimeVal {
            sec: get_time(),
            usec: get_time_us(),
        }
    }
    0
}
