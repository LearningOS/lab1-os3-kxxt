use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskInfo, TASK_MANAGER},
    timer::{read_time, TimeVal},
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

pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    // info!("[kernel] time read: {}ms", get_time_ms());
    unsafe { *ts = read_time() }
    0
}

pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    unsafe { *ti = TASK_MANAGER.get_current_task_info() }
    0
}
