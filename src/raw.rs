use libsoxr_sys as sys;

pub struct SoxrPtr(sys::soxr_t);

impl SoxrPtr {
    pub unsafe fn from_raw(ptr: sys::soxr_t) -> Self {
        SoxrPtr(ptr)
    }

    pub fn as_ptr(&self) -> sys::soxr_t {
        self.0
    }
}

unsafe impl Send for SoxrPtr {}
unsafe impl Sync for SoxrPtr {}

impl Drop for SoxrPtr {
    fn drop(&mut self) {
        unsafe { sys::soxr_delete(self.0); }
    }
}
