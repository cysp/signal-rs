use std;
use libc;
use ffi;
use sigset::SigSet;


pub enum SigMaskHow {
    GetMask,
    Block(SigSet),
    Unblock(SigSet),
    SetMask(SigSet),
}

impl From<SigMaskHow> for libc::c_int {
    fn from(how: SigMaskHow) -> libc::c_int {
        match how {
            SigMaskHow::GetMask => 0,
            SigMaskHow::Block(_) => ffi::SIG_BLOCK,
            SigMaskHow::Unblock(_) => ffi::SIG_UNBLOCK,
            SigMaskHow::SetMask(_) => ffi::SIG_SETMASK,
        }
    }
}


pub fn pthread_sigmask(how: SigMaskHow, oset: Option<&mut SigSet>) -> Result<(), std::io::Error> {
    let (how, set): (libc::c_int, Option<SigSet>) = match how {
        SigMaskHow::GetMask => (0, None),
        SigMaskHow::Block(set) => (ffi::SIG_BLOCK, Some(set)),
        SigMaskHow::Unblock(set) => (ffi::SIG_UNBLOCK, Some(set)),
        SigMaskHow::SetMask(set) => (ffi::SIG_SETMASK, Some(set)),
    };

    let set: *const ffi::sigset_t = match set {
        None => 0 as *const _,
        Some(ref set) => set.as_raw(),
    };
    let oset: *mut ffi::sigset_t = match oset {
        None => 0 as *mut _,
        Some(oset) => oset.as_raw_mut(),
    };

    let rv = unsafe {
        ffi::pthread_sigmask(how, set, oset)
    };
    if rv == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use sigset::SigSet;
    use ffi::SIGNO_MAX;

    #[test]
    fn smoke() {
        let mut sigset = SigSet::new();

        pthread_sigmask(SigMaskHow::GetMask, Some(&mut sigset)).unwrap();
        assert!(sigset.is_empty());

        pthread_sigmask(SigMaskHow::GetMask, Some(&mut sigset)).unwrap();
        assert!(sigset.is_empty());

        sigset.add(1);

        {
            let mut oldsigset = SigSet::new();
            pthread_sigmask(SigMaskHow::Block(sigset), Some(&mut oldsigset)).unwrap();
            assert!(oldsigset.is_empty());
            for i in 1..SIGNO_MAX {
                assert!(!oldsigset.is_member(i), "signal {} is set", i);
            }
        }

        {
            let mut oldsigset = SigSet::new();
            pthread_sigmask(SigMaskHow::GetMask, Some(&mut oldsigset)).unwrap();
            for i in 1..SIGNO_MAX {
                if i == 1 {
                    assert!(oldsigset.is_member(i), "signal {} is not set", i);
                } else {
                    assert!(!oldsigset.is_member(i), "signal {} is set", i);
                }
            }
        }

        sigset.empty();
        sigset.add(2);

        {
            let mut oldsigset = SigSet::new();
            pthread_sigmask(SigMaskHow::Block(sigset), Some(&mut oldsigset)).unwrap();
            for i in 1..SIGNO_MAX {
                if i == 1 {
                    assert!(oldsigset.is_member(i), "signal {} is not set", i);
                } else {
                    assert!(!oldsigset.is_member(i), "signal {} is set", i);
                }
            }
        }

        {
            let mut oldsigset = SigSet::new();
            pthread_sigmask(SigMaskHow::GetMask, Some(&mut oldsigset)).unwrap();
            for i in 1..SIGNO_MAX {
                if i == 1 {
                    assert!(oldsigset.is_member(i), "signal {} is not set", i);
                } else if i == 2 {
                    assert!(oldsigset.is_member(i), "signal {} is not set", i);
                } else {
                    assert!(!oldsigset.is_member(i), "signal {} is set", i);
                }
            }
        }

        sigset.empty();
        sigset.add(1);

        {
            let mut oldsigset = SigSet::new();
            pthread_sigmask(SigMaskHow::Unblock(sigset), Some(&mut oldsigset)).unwrap();
            for i in 1..SIGNO_MAX {
                if i == 1 {
                    assert!(oldsigset.is_member(i), "signal {} is not set", i);
                } else if i == 2 {
                    assert!(oldsigset.is_member(i), "signal {} is not set", i);
                } else {
                    assert!(!oldsigset.is_member(i), "signal {} is set", i);
                }
            }
        }

        {
            let mut oldsigset = SigSet::new();
            pthread_sigmask(SigMaskHow::GetMask, Some(&mut oldsigset)).unwrap();
            for i in 1..SIGNO_MAX {
                if i == 2 {
                    assert!(oldsigset.is_member(i), "signal {} is not set", i);
                } else {
                    assert!(!oldsigset.is_member(i), "signal {} is set", i);
                }
            }
        }

        sigset.empty();

        {
            let mut oldsigset = SigSet::new();
            pthread_sigmask(SigMaskHow::SetMask(sigset), Some(&mut oldsigset)).unwrap();
            for i in 1..SIGNO_MAX {
                if i == 2 {
                    assert!(oldsigset.is_member(i), "signal {} is not set", i);
                } else {
                    assert!(!oldsigset.is_member(i), "signal {} is set", i);
                }
            }
        }

        {
            let mut oldsigset = SigSet::new();
            pthread_sigmask(SigMaskHow::GetMask, Some(&mut oldsigset)).unwrap();
            for i in 1..SIGNO_MAX {
                assert!(!sigset.is_member(i), "signal {} is set", i);
            }
        }
    }
}
