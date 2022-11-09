//! File and filesystem-related syscalls

const FD_STDOUT: usize = 1;

// YOUR JOB: 修改 sys_write 使之通过测试
pub fn sys_write(file_descriptor: usize, buf: *const u8, len: usize) -> isize {
    match file_descriptor {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => panic!(
            "Unknown file descriptor {} passed to sys_write!",
            file_descriptor
        ),
    }
}
