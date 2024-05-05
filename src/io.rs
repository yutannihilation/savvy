use savvy_ffi::{REprintf, R_NilValue, Rprintf};

use std::{ffi::CString, io::Write, os::raw::c_char};

pub(crate) const LINEBREAK: [c_char; 2] = [b'\n' as _, b'\0' as _];

pub fn r_print(msg: &str, linebreak: bool) {
    if !msg.is_empty() {
        // ignore error
        let _ = r_stdout().write_all(msg.as_bytes());
    }

    unsafe {
        if linebreak {
            Rprintf(LINEBREAK.as_ptr());
        }
    }
}

pub fn r_eprint(msg: &str, linebreak: bool) {
    if !msg.is_empty() {
        // ignore error
        let _ = r_stderr().write_all(msg.as_bytes());
    }

    unsafe {
        if linebreak {
            REprintf(LINEBREAK.as_ptr());
        }
    }
}

/// Show a warning.
///
/// Note that, a warning can raise error when `options(warn = 2)`, so you should
/// not ignore the error from `r_warn()`. The error should be propagated to the
/// R session.
pub fn r_warn(msg: &str) -> crate::error::Result<()> {
    unsafe {
        let msg = CString::new(msg).unwrap_or_default();
        crate::unwind_protect(|| {
            savvy_ffi::Rf_warningcall(R_NilValue, msg.as_ptr());
            R_NilValue
        })?;
        Ok(())
    }
}

pub struct RStdout {}

pub fn r_stdout() -> RStdout {
    RStdout {}
}

impl std::io::Write for RStdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let msg = CString::new(buf)?;
        unsafe { savvy_ffi::Rprintf(msg.as_ptr()) };
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct RStderr {}

pub fn r_stderr() -> RStderr {
    RStderr {}
}

impl std::io::Write for RStderr {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let msg = CString::new(buf)?;
        unsafe { savvy_ffi::REprintf(msg.as_ptr()) };
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
