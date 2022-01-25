// C helper for a heap
extern crate libc;

use libc::c_char;
use std::ffi::CStr;

use std::cmp::Ordering;
use crate::heap::Heap;

#[repr(C)]
#[derive(PartialEq)]
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
    queue_name: String,
    queue: Heap<Job>
}

impl CheapJob {
    fn new(name: String) -> CheapJob {
        CheapJob {
            queue_name: name,
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

#[no_mangle]
pub extern "C" fn new_cheap_job(name: *const c_char) -> *mut CheapJob {
    let name = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    Box::into_raw(Box::new(CheapJob::new(name.to_str().unwrap().to_string())))
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
