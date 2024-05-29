//! Process management syscalls
use crate::{loader::get_num_app, timer::get_time_ms};
use alloc::vec::Vec;
use core::cell::UnsafeCell;
use lazy_static::lazy_static;

use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER},
    timer::get_time_us,
};

pub struct StartTime {
    time: UnsafeCell<Vec<usize>>,
}

impl Default for StartTime {
    fn default() -> Self {
        let v = (0..get_num_app()).map(|_| 0).collect();
        Self {
            time: UnsafeCell::new(v),
        }
    }
}
unsafe impl Sync for StartTime {}

impl StartTime {
    pub fn set_start_time_if_not_set(&self, id: usize, time_ms: usize) {
        unsafe {
            let st = self.time.get().as_mut().unwrap();
            if st[id] == 0 {
                st[id] = time_ms;
            }
        }
    }

    pub fn get_run_time_ms(&self, id: usize) -> usize {
        unsafe {
            let st = self.time.get().as_ref().unwrap()[id];
            get_time_ms() - st
        }
    }
}

struct SyscallLog {
    times: [u32; MAX_SYSCALL_NUM],
}

impl Default for SyscallLog {
    fn default() -> Self {
        Self {
            times: [0; MAX_SYSCALL_NUM],
        }
    }
}

impl SyscallLog {
    pub fn call_add(&mut self, call: usize) {
        self.times[call] += 1;
    }
}

pub struct TasksCallLog {
    tasks: UnsafeCell<Vec<SyscallLog>>,
}

unsafe impl Sync for TasksCallLog {}
impl Default for TasksCallLog {
    fn default() -> Self {
        let v = (0..get_num_app()).map(|_| SyscallLog::default()).collect();
        Self {
            tasks: UnsafeCell::new(v),
        }
    }
}

impl TasksCallLog {
    fn get_syscall_log(&self, id: usize) -> &SyscallLog {
        unsafe { &self.tasks.get().as_ref().unwrap()[id] }
    }

    #[deny(clippy::mut_from_ref)]
    fn get_syscall_log_mut(&self, id: usize) -> &mut SyscallLog {
        unsafe { &mut self.tasks.get().as_mut().unwrap()[id] }
    }

    pub fn log_syscall(&self, id: usize, call: usize) {
        self.get_syscall_log_mut(id).call_add(call);
    }
}

lazy_static!(
    /// 表示所有任务的状态
    pub static ref TASK_START_TIME: StartTime = {
        StartTime::default()
    };
    pub static ref TASK_SYSCALL_LOG: TasksCallLog = TasksCallLog::default();
);

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

impl Default for TaskInfo {
    fn default() -> Self {
        Self {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    unsafe {
        let info = ti.as_mut().unwrap();
        info.status = TaskStatus::Running;
        info.time = TASK_START_TIME.get_run_time_ms(TASK_MANAGER.get_current_task_id());
        info.syscall_times.copy_from_slice(
            &TASK_SYSCALL_LOG
                .get_syscall_log(TASK_MANAGER.get_current_task_id())
                .times,
        )
    }
    0
}
