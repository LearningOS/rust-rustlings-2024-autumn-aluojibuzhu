//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;
/// The task control block (TCB) of a task.
/// 
#[derive(Copy, Clone)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Copy, Clone)]
///
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    ///记录该任务第一次调用时间
    pub first_call_time:Option<TimeVal>,
    ///每个任务中系统调用调用次数
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
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

impl  TaskControlBlock{
    ///
    pub fn read_task_status(&self)->TaskStatus{
        self.task_status
    }
    ///更新么每个任务的系统调用次数
    pub fn sys_times_updata(& mut self,sys_call_id:usize){
        //println!("the first time is {}",TASK_MANAGER.read_inner().first_call_time.unwrap());
        println!("the change is ok? first{}",self.syscall_times[sys_call_id]);
        self.syscall_times[sys_call_id]+=1;
       // self.syscall_times[sys_call_id]+=1;
        println!("the change is ok? last{}",self.syscall_times[sys_call_id]);
    }

}
