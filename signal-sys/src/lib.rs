extern crate libc;


#[cfg(
    all(target_os = "linux",
        any(target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "arm",
            target_arch = "aarch64"
        )
    )
)]
mod os {
    use libc;

    pub const SIG_BLOCK: libc::c_int = 0;
    pub const SIG_UNBLOCK: libc::c_int = 1;
    pub const SIG_SETMASK: libc::c_int = 2;

    #[repr(C)]
    #[derive(Copy,Clone,Debug)]
    pub struct sigset_t {
        #[cfg(target_pointer_width = "32")]
        pub __val: [libc::uint32_t; 1024/32],
        #[cfg(target_pointer_width = "64")]
        pub __val: [libc::uint64_t; 1024/64],
    }

    pub const SIGNO_MAX: libc::c_int = 64;

    #[cfg(target_pointer_width = "32")]
    pub const SIGSET_EMPTY: sigset_t = sigset_t { __val: [0; 1024/32]};
    #[cfg(target_pointer_width = "64")]
    pub const SIGSET_EMPTY: sigset_t = sigset_t { __val: [0; 1024/64]};
}

#[cfg(target_os = "macos")]
mod os {
    use libc;

    pub const SIG_BLOCK: libc::c_int = 1;
    pub const SIG_UNBLOCK: libc::c_int = 2;
    pub const SIG_SETMASK: libc::c_int = 3;

    #[repr(C)]
    #[derive(Copy,Clone,Debug)]
    pub struct sigset_t(libc::uint32_t);

    pub const SIGNO_MAX: libc::c_int = 32;

    pub const SIGSET_EMPTY: sigset_t = sigset_t(0);
}

pub use os::{SIG_BLOCK, SIG_UNBLOCK, SIG_SETMASK};
pub use os::{sigset_t, SIGNO_MAX, SIGSET_EMPTY};


extern {
    pub fn sigaddset(set: *mut sigset_t, signo: libc::c_int) -> libc::c_int;
    pub fn sigdelset(set: *mut sigset_t, signo: libc::c_int) -> libc::c_int;
    pub fn sigemptyset(set: *mut sigset_t) -> libc::c_int;
    pub fn sigfillset(set: *mut sigset_t) -> libc::c_int;
    pub fn sigismember(set: *const sigset_t, signo: libc::c_int) -> libc::c_int;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        let sigset = SIGSET_EMPTY;

        for i in 1..SIGNO_MAX {
            assert!(unsafe { sigismember(&sigset, i) } == 0, "signal {} is set in empty set", i);
        }
    }

    #[test]
    fn smoke_set_operations1() {
        let mut sigset = SIGSET_EMPTY;

        let _ = unsafe { sigaddset(&mut sigset, 2) };

        for i in 1..SIGNO_MAX {
            if i == 2 {
                assert!(unsafe { sigismember(&sigset, i) } == 1, "signal {} is not set", i);
            } else {
                assert!(unsafe { sigismember(&sigset, i) } == 0, "signal {} is set", i);
            }
        }

        let _ = unsafe { sigdelset(&mut sigset, 2) };

        for i in 1..SIGNO_MAX {
            assert!(unsafe { sigismember(&sigset, i) } == 0, "signal {} is set", i);
        }
    }

    #[test]
    fn smoke_set_operations2() {
        let mut sigset = SIGSET_EMPTY;

        let _ = unsafe { sigaddset(&mut sigset, 2) };

        for i in 1..SIGNO_MAX {
            if i == 2 {
                assert!(unsafe { sigismember(&sigset, i) } == 1, "signal {} is not set", i);
            } else {
                assert!(unsafe { sigismember(&sigset, i) } == 0, "signal {} is set", i);
            }
        }

        let _ = unsafe { sigaddset(&mut sigset, 2) };

        for i in 1..SIGNO_MAX {
            if i == 2 {
                assert!(unsafe { sigismember(&sigset, i) } == 1, "signal {} is not set", i);
            } else {
                assert!(unsafe { sigismember(&sigset, i) } == 0, "signal {} is set", i);
            }
        }

        let _ = unsafe { sigdelset(&mut sigset, 2) };

        for i in 1..SIGNO_MAX {
            assert!(unsafe { sigismember(&sigset, i) } == 0, "signal {} is set", i);
        }
    }
}
