mod fs;
mod process;

use fs::*;
use process::*;

// System Call Numbers

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_TASK_INFO: usize = 410;

pub fn syscall(syscall_num: usize, args: [usize; 3]) -> isize {
    match syscall_num {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        _ => panic!("Unknown system call number: {}", syscall_num),
    }
}
