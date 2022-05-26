mod context;
mod id;
pub mod schd;
mod switch;
mod task;

use crate::tools::uninit_cell::UninitCell;
use crate::{fs::rfs::find_inode, interrupt::timer};
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
pub use context::TaskContext;
use schd::{get_time_slice, SchdMaster};
pub use switch::__switch;
pub use task::{ProcessControlBlock, TaskPos, TaskStatus};

pub struct TaskManager {
    current_task: Rc<ProcessControlBlock>,
    schd: SchdMaster,
}

impl TaskManager {
    fn new(deamon: Rc<ProcessControlBlock>) -> Self {
        Self {
            current_task: deamon,
            schd: SchdMaster::new(),
        }
    }

    fn switch_to_next_task(&mut self) {
        let current_task = self.current_task.clone();
        let mut current_task_inner = current_task.inner.borrow_mut();
        let current_task_cx = &mut current_task_inner.task_cx as *mut TaskContext;
        let current_task_status = current_task_inner.task_status;
        if current_task_status != TaskStatus::Exited {
            current_task_inner.task_status = TaskStatus::Ready;
            drop(current_task_inner);
            self.schd.requeue_current(current_task);
        } else {
            drop(current_task_inner);
            drop(current_task);
        }
        let next_task = self.schd.get_next().unwrap();
        let mut next_task_inner = next_task.inner.borrow_mut();
        let next_task_cx = &mut next_task_inner.task_cx as *mut TaskContext;
        timer::set_next_timeout(get_time_slice(next_task_inner.task_pos));
        drop(next_task_inner);
        self.current_task = next_task;
        unsafe {
            if current_task_status != TaskStatus::Exited {
                __switch(current_task_cx, next_task_cx);
            } else {
                let mut _unused = TaskContext::zero_init();
                __switch(&mut _unused, next_task_cx);
            }
        }
    }

    pub fn get_current_process(&self) -> Rc<ProcessControlBlock> {
        self.current_task.clone()
    }
}

pub static mut TASK_MANAGER: UninitCell<TaskManager> = UninitCell::uninit();

pub static mut DEAMON: UninitCell<Rc<ProcessControlBlock>> = UninitCell::uninit();

pub fn add_new_task(task: Rc<ProcessControlBlock>) {
    unsafe {
        TASK_MANAGER.schd.add_new_task(task);
    }
}

pub fn exit_current_and_run_next(exit_code: i32) {
    let proc = get_current_process();
    let mut inner = proc.inner.borrow_mut();
    println!(
        "[kernel] Process {} exit with code {}",
        proc.pid.0, exit_code
    );
    inner.task_status = TaskStatus::Exited;
    inner.exit_code = exit_code;
    unsafe {
        let mut deamon_inner = DEAMON.inner.borrow_mut();
        for child in inner.children.iter() {
            child.inner.borrow_mut().parent = Rc::downgrade(&DEAMON);
            deamon_inner.children.push(child.clone());
        }
    }
    drop(inner);
    drop(proc);
    suspend_current_and_run_next();
}

pub fn suspend_current_and_run_next() {
    unsafe {
        TASK_MANAGER.switch_to_next_task();
    }
}

/// the callback function used in the supervisor time interrupt
/// to implement the basic task scheduling
pub fn schedule_callback() {
    unsafe {
        TASK_MANAGER.switch_to_next_task();
    }
}

pub fn get_current_process() -> Rc<ProcessControlBlock> {
    unsafe { TASK_MANAGER.get_current_process() }
}

pub fn init() {
    unsafe {
        id::init();
        let app_inode = find_inode("/bin/deamon").expect("[kernel] deamon not found!");
        let size = app_inode.get_file_size() as usize;
        let mut app_data = vec![0u8; size];
        app_inode.read_at(0, &mut app_data);
        DEAMON = UninitCell::init(Rc::new(ProcessControlBlock::new(&app_data)));
        TASK_MANAGER = UninitCell::init(TaskManager::new(DEAMON.clone()));
        println!("mod task initialized!");
    }
}

pub fn run() {
    unsafe {
        let current_task = TASK_MANAGER.current_task.clone();
        let mut current_task_inner = current_task.inner.borrow_mut();
        let current_task_cx = &mut current_task_inner.task_cx as *mut TaskContext;
        drop(current_task_inner);
        let mut _unused = TaskContext::zero_init();
        __switch(&mut _unused as *mut TaskContext, current_task_cx);
        unreachable!();
    }
}
