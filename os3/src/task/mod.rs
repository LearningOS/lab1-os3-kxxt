use lazy_static::lazy_static;

use crate::{
    config::{MAX_APP_NUM, MAX_SYSCALL_NUM},
    loader::{get_num_app, init_app_cx},
    sync::UPSafeCell,
    task::context::TaskContext,
    timer::get_time_ms,
};

use self::{switch::__switch, task::TaskControlBlock};

mod context;
mod switch;
mod task;

pub use task::{TaskInfo, TaskStatus};

pub fn run_first_task() {
    debug!("[kernel] Preparing to call run_first_task.");
    TASK_MANAGER.run_first_task();
}

pub fn suspend_current_and_run_next() {
    TASK_MANAGER.mark_current_suspended();
    TASK_MANAGER.run_next_task();
}

pub fn exit_current_and_run_next() {
    TASK_MANAGER.mark_current_exited();
    TASK_MANAGER.run_next_task();
}

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {
    fn run_first_task(&self) -> ! {
        debug!("[kernel]: ENTERING");
        let mut inner = self.inner.exclusive_access();
        debug!("[kernel]: ARE YOU OK?");
        inner.record_current_task_start_time();
        println!("[KERN]: STILL HERE?");
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        // 声明此变量的意义仅仅是为了避免其他数据被覆盖。
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_cx_ptr);
        }
        panic!("Unreachable code in run_first_task!");
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        // Running -> Ready
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        // Running -> Exited
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn run_next_task(&self) {
        let Some(next) = self.find_next_task() else { panic!("All applications completed!") };
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[next].task_status = TaskStatus::Running;
        inner.current_task = next;
        if inner.tasks[next].time_start == 0 {
            inner.record_current_task_start_time();
        }
        let current_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
        let next_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
        drop(inner); // give up exclusive access
        unsafe {
            __switch(current_cx_ptr, next_cx_ptr);
        }
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|&id| inner.tasks[id].task_status == TaskStatus::Ready)
    }

    pub(crate) fn get_current_task_info(&self) -> TaskInfo {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let task_cb = inner.tasks[current];
        let time_ms = get_time_ms();
        info!(
            "[kernel] get current task info called by task {} at time {}",
            current, time_ms
        );
        TaskInfo {
            status: task_cb.task_status,
            syscall_times: task_cb.syscall_times,
            time: time_ms - task_cb.time_start,
        }
    }

    pub(crate) fn increase_syscall_counter(&self, num: usize) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].syscall_times[num] += 1;
    }
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

impl TaskManagerInner {
    fn record_current_task_start_time(&mut self) {
        let t = get_time_ms();
        info!("[kernel] task {} started at time {}", self.current_task, t);
        self.tasks[self.current_task].time_start = t;
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time_start: 0,
        }; MAX_APP_NUM];
        for (i, t) in tasks.iter_mut().enumerate().take(num_app) {
            t.task_cx = TaskContext::goto_restore(init_app_cx(i));
            t.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}
