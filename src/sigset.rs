use std;
use libc;
use ffi;


#[derive(Copy,Clone,Debug)]
pub struct SigSet {
    set: ffi::sigset_t,
}

impl SigSet {
    pub fn from_raw(sigset: ffi::sigset_t) -> SigSet {
        SigSet {
            set: sigset,
        }
    }
    pub fn as_raw(&self) -> *const ffi::sigset_t {
        &self.set
    }

    pub fn as_raw_mut(&mut self) -> *mut ffi::sigset_t {
        &mut self.set
    }

    pub fn builder() -> SigSetBuilder {
        SigSetBuilder {
            set: SigSet::new(),
        }
    }

    pub fn new() -> SigSet {
        let mut set: ffi::sigset_t = unsafe { std::mem::uninitialized() };
        unsafe { ffi::sigemptyset(&mut set) };
        SigSet {
            set: set,
        }
    }

    pub fn empty(&mut self) {
        unsafe { ffi::sigemptyset(&mut self.set) };
    }

    pub fn fill(&mut self) {
        unsafe { ffi::sigfillset(&mut self.set) };
    }

    pub fn add(&mut self, signo: libc::c_int) {
        unsafe { ffi::sigaddset(&mut self.set, signo) };
    }

    pub fn del(&mut self, signo: libc::c_int) {
        unsafe { ffi::sigdelset(&mut self.set, signo) };
    }

    pub fn is_member(&self, signo: libc::c_int) -> bool {
        let b = unsafe {
            ffi::sigismember(&self.set, signo)
        };
        b != 0
    }

    pub fn is_empty(&self) -> bool {
        for i in 1..ffi::SIGNO_MAX {
            if self.is_member(i) {
                return false;
            }
        }
        true
    }
}


#[derive(Copy,Clone,Debug)]
pub struct SigSetBuilder {
    set: SigSet,
}

impl SigSetBuilder {
    pub fn empty(mut self) -> SigSetBuilder {
        self.set.empty();
        self
    }

    pub fn fill(mut self) -> SigSetBuilder {
        self.set.fill();
        self
    }

    pub fn add(mut self, signo: libc::c_int) -> SigSetBuilder {
        self.set.add(signo);
        self
    }

    pub fn del(mut self, signo: libc::c_int) -> SigSetBuilder {
        self.set.del(signo);
        self
    }

    pub fn build(self) -> SigSet {
        self.set
    }
}
