//! Process management syscalls
//use riscv::register::sstatus;
use crate::syscall::{SYSCALL_TASK_INFO,SYSCALL_GET_TIME,SYSCALL_YIELD,SYSCALL_EXIT};
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    TASK_MANAGER.read_inner_systimes_updata(SYSCALL_EXIT);
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    TASK_MANAGER.read_inner_systimes_updata(SYSCALL_YIELD);
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    //if TASK_MANAGER.read_current_task()==2{
    //println!("system time {}",TASK_MANAGER.read_inner().syscall_times[SYSCALL_GET_TIME]);
    //}
    TASK_MANAGER.read_inner_systimes_updata(SYSCALL_GET_TIME);
    let us = get_time_us();
    unsafe {
        (*ts) = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
        //println!("the time is change?  {}",(*ts).sec);
    }    
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    unsafe {
        TASK_MANAGER.read_inner_systimes_updata(SYSCALL_TASK_INFO);
        let mut now_time= TimeVal::new();
        sys_get_time( & mut now_time,0);
     //   println!("now _time {}",now_time.usec);
    //println!("the current time is {}",TASK_INFO_GLOB.time);
    *_ti=TaskInfo{
        status:TASK_MANAGER.read_inner().task_status,
        syscall_times:TASK_MANAGER.read_inner().syscall_times,
        time:((now_time.sec*1_000_000+now_time.usec)/1000-(TASK_MANAGER.read_inner().first_call_time.unwrap().sec*1_000_000+TASK_MANAGER.read_inner().first_call_time.unwrap().usec)/1000),
    }
    
}
    return 0
}
    
impl  TimeVal{
    pub fn new()->TimeVal{
        TimeVal{
            sec: 0,
            usec: 0
        }
    }

}

