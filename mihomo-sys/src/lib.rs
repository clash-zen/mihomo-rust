use libc::{c_char, c_int, size_t};

#[allow(non_camel_case_types)]
pub type mihomo_start_ret = c_int;

extern "C" {
    pub fn mihomo_start(config: *const u8, config_len: size_t) -> mihomo_start_ret;
    pub fn mihomo_stop() -> c_int;
    pub fn mihomo_last_error_copy(out: *mut c_char, out_len: size_t) -> size_t;
}

/// Start mihomo kernel (FFI).
///
/// Caller must ensure only one start/stop lifecycle in process.
pub unsafe fn start(config_yaml_bytes: &[u8]) -> c_int {
    mihomo_start(
        config_yaml_bytes.as_ptr(),
        config_yaml_bytes.len() as size_t,
    )
}

/// Stop mihomo kernel (FFI).
pub unsafe fn stop() -> c_int {
    mihomo_stop()
}

pub fn last_error() -> String {
    let len = unsafe { mihomo_last_error_copy(std::ptr::null_mut(), 0 as size_t) } as usize;
    if len == 0 {
        return String::new();
    }

    // +1 for '\0'
    let mut buf = vec![0u8; len + 1];
    unsafe {
        mihomo_last_error_copy(buf.as_mut_ptr() as *mut c_char, (len + 1) as size_t);
    }

    if let Some(pos) = buf.iter().position(|&b| b == 0) {
        buf.truncate(pos);
    }
    String::from_utf8_lossy(&buf).into_owned()
}
