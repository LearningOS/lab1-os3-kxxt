use lazy_static::lazy_static;

use crate::{
    config::MAX_APP_NUM,
    loader::get_num_app,
    sync::UPSafeCell,
    task::{context::TaskContext, task::TaskStatus},
};

use self::task::TaskControlBlock;

mod context;
mod switch;
mod task;

pub fn run_first_task() {
    todo!()
}

pub fn suspend_current_and_run_next() {

}

pub fn exit_current_and_run_next() {
    
}


pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM];
        for (i, t) in tasks.iter_mut().enumerate().take(num_app) {
            t.task_cx = TaskContext::goto_restore(i);
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
