// C helper for a heap
extern crate libc;

// use libc::c_char;
// use std::ffi::CStr;
use std::ptr::null_mut;

use std::cmp::Ordering;
use crate::heap::Heap;

#[repr(C)]
#[derive(PartialEq, Clone)]
pub struct Job {
    task_id: cty::c_int,
    priority: cty::c_int,
    tie_break: cty::c_int
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.priority == other.priority {
            Some(self.tie_break.cmp(&other.tie_break))
        } else {
            Some(self.priority.cmp(&other.priority))
        }
    }
}

pub struct CheapJob {
    // queue_name: String,
    queue: Heap<Job>
}

impl CheapJob {
    // fn new(name: String) -> CheapJob {
    fn new() -> CheapJob {
        CheapJob {
            // queue_name: name,
            queue: Heap::new()
        }
    }

    fn pop_job(&mut self) -> Option<Job> {
        self.queue.pop()
    }

    fn push_job(&mut self, job: Job) {
        self.queue.push(job)
    }
}

// pub extern "C" fn new_cheap_job(name: *const c_char) -> *mut CheapJob {
//     let name = unsafe {
//         assert!(!name.is_null());
//         CStr::from_ptr(name)
//     };
#[no_mangle]
pub extern "C" fn new_cheap_job() -> *mut CheapJob {
    Box::into_raw(Box::new(CheapJob::new()))
}

#[no_mangle]
pub extern "C" fn free_cheap_job(ptr: *mut CheapJob) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(ptr);
    }
}

// #[no_mangle]
// pub extern "C" fn cheap_job_name(ptr: *const CheapJob) -> *const c_char {
//     let cj = unsafe {
//         assert!(!ptr.is_null());
//         &mut *ptr
//     };
//     CStr::to_ptr(cj.queue_name).unwrap().into_raw()
// }

#[no_mangle]
pub extern "C" fn push_cheap_job(ptr: *mut CheapJob, job: *const Job) {
    let cj = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let job = unsafe {
        assert!(!job.is_null());
        &*job
    };

    cj.push_job(job.clone());
}

#[no_mangle]
pub extern "C" fn pop_cheap_job(ptr: *mut CheapJob) -> *mut Job {
    let cj = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match cj.pop_job() {
        // do I need to do something like Box::into_raw or something?
        Some(mut job) => &mut job,
        None => null_mut()
    }
}
