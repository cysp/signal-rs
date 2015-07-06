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

    pub mod siginfo {
        const __SI_MAX_SIZE: usize = 128;
        #[cfg(target_pointer_width = "32")]
        const __SI_PAD_SIZE: usize = (__SI_MAX_SIZE / 4) - 3;
        #[cfg(target_pointer_width = "64")]
        const __SI_PAD_SIZE: usize = (__SI_MAX_SIZE / 4) - 4;

        #[repr(C)]
        #[derive(Copy,Clone,Debug)]
        pub struct siginfo_t {
            pub si_signo: super::super::libc::c_int, /* signal number */
            pub si_errno: super::super::libc::c_int, /* errno association */
            pub si_code: super::super::libc::c_int, /* signal code */
            __pad: [super::super::libc::c_int; __SI_PAD_SIZE],
        }

        pub const SI_ASYNCNL: super::super::libc::c_int = -60;
        pub const SI_TKILL: super::super::libc::c_int = -6;
        pub const SI_SIGIO: super::super::libc::c_int = -5;
        pub const SI_ASYNCIO: super::super::libc::c_int = -4;
        pub const SI_MESGQ: super::super::libc::c_int = -3;
        pub const SI_TIMER: super::super::libc::c_int = -2;
        pub const SI_QUEUE: super::super::libc::c_int = -1;
        pub const SI_USER: super::super::libc::c_int = 0;
        pub const SI_KERNEL: super::super::libc::c_int = 128;

        #[repr(C)]
        #[derive(Copy,Clone,Debug)]
        pub struct siginfo_kill_t {
            pub si_signo: super::super::libc::c_int, /* signal number */
            pub si_errno: super::super::libc::c_int, /* errno association */
            pub si_code: super::super::libc::c_int, /* signal code */
            pub si_pid: super::super::libc::int32_t, /* sending process */
            pub si_uid: super::super::libc::uint32_t, /* sender's ruid */
        }

        #[repr(C)]
        #[derive(Copy,Clone,Debug)]
        pub struct siginfo_timer_t {
            pub si_signo: super::super::libc::c_int, /* signal number */
            pub si_errno: super::super::libc::c_int, /* errno association */
            pub si_code: super::super::libc::c_int, /* signal code */
            pub si_tid: super::super::libc::int32_t, /* timer id */
            pub si_overrun: super::super::libc::int32_t, /* overrun */
            pub si_sigval: *const super::super::libc::c_void, /* signal value */
        }

        #[repr(C)]
        #[derive(Copy,Clone,Debug)]
        pub struct siginfo_rt_t {
            pub si_signo: super::super::libc::c_int, /* signal number */
            pub si_errno: super::super::libc::c_int, /* errno association */
            pub si_code: super::super::libc::c_int, /* signal code */
            pub si_pid: super::super::libc::int32_t, /* sending process */
            pub si_uid: super::super::libc::uint32_t, /* sender's ruid */
            pub si_sigval: *const super::super::libc::c_void, /* signal value */
        }

        pub const CLD_EXITED: super::super::libc::c_int = 1;
        pub const CLD_KILLED: super::super::libc::c_int = 2;
        pub const CLD_DUMPED: super::super::libc::c_int = 3;
        pub const CLD_TRAPPED: super::super::libc::c_int = 4;
        pub const CLD_STOPPED: super::super::libc::c_int = 5;
        pub const CLD_CONTINUED: super::super::libc::c_int = 6;

        // siginfo_sigchld_t

        pub const ILL_ILLOPC: super::super::libc::c_int = 1;
        pub const ILL_ILLOPN: super::super::libc::c_int = 2;
        pub const ILL_ILLADR: super::super::libc::c_int = 3;
        pub const ILL_ILLTRP: super::super::libc::c_int = 4;
        pub const ILL_PRVOPC: super::super::libc::c_int = 5;
        pub const ILL_PRVREG: super::super::libc::c_int = 6;
        pub const ILL_COPROC: super::super::libc::c_int = 7;
        pub const ILL_BADSTK: super::super::libc::c_int = 8;

        pub const FPE_INTDIV: super::super::libc::c_int = 1;
        pub const FPE_INTOVF: super::super::libc::c_int = 2;
        pub const FPE_FLTDIV: super::super::libc::c_int = 3;
        pub const FPE_FLTOVF: super::super::libc::c_int = 4;
        pub const FPE_FLTUND: super::super::libc::c_int = 5;
        pub const FPE_FLTRES: super::super::libc::c_int = 6;
        pub const FPE_FLTINV: super::super::libc::c_int = 7;
        pub const FPE_FLTSUB: super::super::libc::c_int = 8;

        pub const SEGV_MAPERR: super::super::libc::c_int = 1;
        pub const SEGV_ACCERR: super::super::libc::c_int = 2;

        pub const BUS_ADRALN: super::super::libc::c_int = 1;
        pub const BUS_ADRERR: super::super::libc::c_int = 2;
        pub const BUS_OBJERR: super::super::libc::c_int = 3;
        pub const BUS_MCEERR_AR: super::super::libc::c_int = 4;
        pub const BUS_MCEERR_AO: super::super::libc::c_int = 5;

        pub const TRAP_BRKPT: super::super::libc::c_int = 1;
        pub const TRAP_TRACE: super::super::libc::c_int = 2;

        #[repr(C)]
        #[derive(Copy,Clone,Debug)]
        pub struct siginfo_sigfault_t {
            pub si_signo: super::super::libc::c_int, /* signal number */
            pub si_errno: super::super::libc::c_int, /* errno association */
            pub si_code: super::super::libc::c_int, /* signal code */
            pub si_addr: *const super::super::libc::c_void, /* faulting insn / memory ref */
            pub si_addr_lsb: super::super::libc::int16_t, /* valid lsb of the reported address */
        }

        pub const POLL_IN: super::super::libc::c_int = 1;
        pub const POLL_OUT: super::super::libc::c_int = 2;
        pub const POLL_MSG: super::super::libc::c_int = 3;
        pub const POLL_ERR: super::super::libc::c_int = 4;
        pub const POLL_PRI: super::super::libc::c_int = 5;
        pub const POLL_HUP: super::super::libc::c_int = 6;

        #[repr(C)]
        #[derive(Copy,Clone,Debug)]
        pub struct siginfo_sigpoll_t {
            pub si_signo: super::super::libc::c_int, /* signal number */
            pub si_errno: super::super::libc::c_int, /* errno association */
            pub si_code: super::super::libc::c_int, /* signal code */
            pub si_band: super::super::libc::c_long, /* band event for sigpoll */
            pub si_fd: super::super::libc::c_int,
        }

        #[repr(C)]
        #[derive(Copy,Clone,Debug)]
        pub struct siginfo_sigsys_t {
            pub si_signo: super::super::libc::c_int, /* signal number */
            pub si_errno: super::super::libc::c_int, /* errno association */
            pub si_code: super::super::libc::c_int, /* signal code */
            pub _call_addr: *const super::super::libc::c_void, /* calling user insn */
            pub _syscall: super::super::libc::c_int, /* triggering system call number */
            pub _arch: super::super::libc::c_uint, /* AUDIT_ARCH_* of syscall */
        }
    }
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

    pub mod siginfo {
        #[repr(C)]
        #[derive(Copy,Clone,Debug)]
        pub struct siginfo_t {
            pub si_signo: super::super::libc::c_int, /* signal number */
            pub si_errno: super::super::libc::c_int, /* errno association */
            pub si_code: super::super::libc::c_int, /* signal code */
            pub si_pid: super::super::libc::int32_t, /* sending process */
            pub si_uid: super::super::libc::uint32_t, /* sender's ruid */
            pub si_status: super::super::libc::c_int, /* exit value */
            pub si_addr: *const super::super::libc::c_void, /* faulting instruction */
            pub si_value: *const super::super::libc::c_void, /* signal value */
            pub si_band: super::super::libc::c_long, /* band event for SIGPOLL */
            __pad: [super::super::libc::c_ulong; 7], /* Reserved for Future Use */
        }

        pub const SI_USER: super::super::libc::c_int = 0x10001; /* [CX] signal from kill() */
        pub const SI_QUEUE: super::super::libc::c_int = 0x10002; /* [CX] signal from sigqueue() */
        pub const SI_TIMER: super::super::libc::c_int = 0x10003; /* [CX] timer expiration */
        pub const SI_ASYNCIO: super::super::libc::c_int = 0x10004; /* [CX] aio request completion */
        pub const SI_MESGQ: super::super::libc::c_int = 0x10005; /* [CX] from message arrival on empty queue */
    }
}

pub use os::{SIG_BLOCK, SIG_UNBLOCK, SIG_SETMASK};
pub use os::{sigset_t, SIGNO_MAX, SIGSET_EMPTY};
pub use os::siginfo::*;


extern {
    pub fn sigaddset(set: *mut sigset_t, signo: libc::c_int) -> libc::c_int;
    pub fn sigdelset(set: *mut sigset_t, signo: libc::c_int) -> libc::c_int;
    pub fn sigemptyset(set: *mut sigset_t) -> libc::c_int;
    pub fn sigfillset(set: *mut sigset_t) -> libc::c_int;
    pub fn sigismember(set: *const sigset_t, signo: libc::c_int) -> libc::c_int;

    pub fn pthread_sigmask(how: libc::c_int, set: *const sigset_t, oset: *mut sigset_t) -> libc::c_int;
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
