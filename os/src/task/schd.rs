use super::task::*;
use crate::config::{TASK_QUEUE_FCFS1_SLICE, TASK_QUEUE_FCFS2_SLICE, TASK_QUEUE_RR_SLICE};
use alloc::collections::VecDeque;
use alloc::rc::Rc;

struct MultilevelFeedbackQueue {
    fcfs1_queue: VecDeque<Rc<ProcessControlBlock>>,
    fcfs2_queue: VecDeque<Rc<ProcessControlBlock>>,
    rr_queue: VecDeque<Rc<ProcessControlBlock>>,
}

impl MultilevelFeedbackQueue {
    pub fn new() -> Self {
        MultilevelFeedbackQueue {
            fcfs1_queue: VecDeque::new(),
            fcfs2_queue: VecDeque::new(),
            rr_queue: VecDeque::new(),
        }
    }
    pub fn requeue(&mut self, task: Rc<ProcessControlBlock>) -> bool {
        let mut inner = task.inner.borrow_mut();
        match inner.task_pos {
            TaskPos::Fcfs1 => {
                inner.task_pos = TaskPos::Fcfs2;
                drop(inner);
                self.fcfs2_queue.push_back(task);
                true
            }
            TaskPos::Fcfs2 => {
                inner.task_pos = TaskPos::Rr;
                drop(inner);
                self.rr_queue.push_back(task);
                true
            }
            TaskPos::Rr => {
                inner.task_pos = TaskPos::Rr;
                drop(inner);
                self.rr_queue.push_back(task);
                true
            }
        }
    }
    pub fn enqueue(&mut self, task: Rc<ProcessControlBlock>) {
        self.fcfs1_queue.push_back(task)
    }
    pub fn get_task(&mut self) -> Option<Rc<ProcessControlBlock>> {
        let task = self.fcfs1_queue.pop_front();
        if task.is_some() {
            return task;
        }
        let task = self.fcfs2_queue.pop_front();
        if task.is_some() {
            return task;
        }
        self.rr_queue.pop_front()
    }
}

pub struct SchdMaster {
    mlfq: MultilevelFeedbackQueue,
}

impl SchdMaster {
    pub fn new() -> Self {
        SchdMaster {
            mlfq: MultilevelFeedbackQueue::new(),
        }
    }

    pub fn requeue_current(&mut self, current_task_cb: Rc<ProcessControlBlock>) {
        self.mlfq.requeue(current_task_cb);
    }

    pub fn get_next(&mut self) -> Option<Rc<ProcessControlBlock>> {
        self.mlfq.get_task()
    }

    pub fn add_new_task(&mut self, tcb: Rc<ProcessControlBlock>) {
        self.mlfq.enqueue(tcb);
    }
}

#[inline(always)]
pub fn get_time_slice(pos: TaskPos) -> usize {
    match pos {
        TaskPos::Fcfs1 => TASK_QUEUE_FCFS1_SLICE,
        TaskPos::Fcfs2 => TASK_QUEUE_FCFS2_SLICE,
        TaskPos::Rr => TASK_QUEUE_RR_SLICE,
    }
}

#[inline(always)]
pub fn get_default_time_slice() -> usize {
    TASK_QUEUE_FCFS1_SLICE
}
