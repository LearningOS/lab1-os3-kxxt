mod fs;
mod process;

use fs::*;
use process::*;

use crate::task::{TaskInfo, TASK_MANAGER};

// System Call Numbers

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_TASK_INFO: usize = 410;

pub fn syscall(syscall_num: usize, args: [usize; 3]) -> isize {
    TASK_MANAGER.increase_syscall_counter(syscall_num);
    match syscall_num {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, 0),
        SYSCALL_TASK_INFO => sys_task_info(args[0] as *mut TaskInfo),
        _ => panic!("Unknown system call number: {}", syscall_num),
    }
}
