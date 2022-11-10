use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MICRO_PER_SEC: usize = 1_000_000;
const MILLI_PER_SEC: usize = 1_000;

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn get_time() -> usize {
    time::read()
}

pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}

pub fn get_time_ms() -> usize {
    let ms0 = time::read() / (CLOCK_FREQ / MILLI_PER_SEC);
    let time = read_time();
    let ms2 = (time.sec & 0xffff) * 1000 + time.usec / 1000;
    debug!("delta ms: {}", ms2 - ms0);
    ms2
}

pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

pub fn read_time() -> TimeVal {
    let usecs = get_time_us();
    // info!("[kernel] time read: {}ms", usecs / 1000);
    // let sp: usize;
    // unsafe {
    //     asm!("mv {}, sp", out(reg) sp);
    // }
    // trace!("SP: 0x{:x}", sp);
    TimeVal {
        sec: usecs / MICRO_PER_SEC,
        usec: usecs % MICRO_PER_SEC,
    }
}
