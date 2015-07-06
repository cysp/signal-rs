use std;
use libc;
use sys;


#[derive(Copy,Clone,Debug)]
pub struct SigInfo {
    si: sys::siginfo_t,
}

impl SigInfo {
    pub fn from_raw(si: sys::siginfo_t) -> SigInfo {
        si.into()
    }
}

impl From<sys::siginfo_t> for SigInfo {
    fn from(si: sys::siginfo_t) -> SigInfo {
        SigInfo {
            si: si,
        }
    }
}
