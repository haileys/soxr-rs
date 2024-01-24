use core::ffi::CStr;
use core::fmt::{Display, Debug};
use core::ptr::null_mut;
use libsoxr_sys as sys;

pub struct Error(&'static CStr);

pub(crate) const CHANNEL_COUNT_TOO_LARGE: Error = Error(
    unsafe { CStr::from_bytes_with_nul_unchecked(b"channel count does not fit in c_uint\0") }
);

impl Error {
    pub unsafe fn from_raw(error: sys::soxr_error_t) -> Self {
        Error(CStr::from_ptr(error))
    }

    pub(crate) unsafe fn check(error: sys::soxr_error_t) -> Result<(), Error> {
        if error == null_mut() {
            Ok(())
        } else {
            Err(Error::from_raw(error))
        }
    }

    pub fn as_cstr(&self) -> &'static CStr {
        self.0
    }

    pub fn as_str(&self) -> &'static str {
        // SAFETY: all soxr error strings are valid utf-8
        unsafe { core::str::from_utf8_unchecked(self.0.to_bytes()) }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Error").field(&self.as_str()).finish()
    }
}
