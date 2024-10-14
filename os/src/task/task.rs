//! Types related to task management


use super::TaskContext;
//小小用一下
use crate::config::MAX_SYSCALL_NUM;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    //这里添加新属性 一个统计call 一个统计距离第一次隔了多久 一个记录第一次run的时候

    ///在我们的实验中，系统调用号一定小于 500，所以直接使用一个长为 MAX_SYSCALL_NUM=500 的数组做桶计数。
    pub sys_calls: [u32; MAX_SYSCALL_NUM], //uszie怎么你了
    pub first_sys_run_time: usize,
    pub first_to_this_sys_time:usize
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
