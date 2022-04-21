//! 计时器操作子模块

use crate::config::{CLOCK_FREQ, TASK_QUEUE_FCFS1_SLICE};
use crate::sbi::set_timer;
use riscv::register::{sie, time};

/// 读取time寄存器
pub fn get_time() -> usize {
    time::read()
}

/// 获取系统时钟(ms)
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / 1000)
}

/// 开启时钟中断
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

/// 设置下一个时钟间隔
pub fn set_next_timeout(interval: usize) {
    set_timer(time::read() + interval);
}

/// 时钟初始化
pub fn init() {
    enable_timer_interrupt();
    set_next_timeout(TASK_QUEUE_FCFS1_SLICE);
}
