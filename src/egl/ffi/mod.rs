use khronos::khronos_uint64_t as k_khronos_uint64_t;
use khronos::khronos_ssize_t as k_khronos_ssize_t;
use khronos::khronos_utime_nanoseconds_t as k_khronos_utime_nanoseconds_t;
use libc::c_void;

#[allow(non_camel_case_types)]
pub type khronos_utime_nanoseconds_t = k_khronos_utime_nanoseconds_t;
#[allow(non_camel_case_types)]
pub type khronos_uint64_t = k_khronos_uint64_t;
#[allow(non_camel_case_types)]
pub type khronos_ssize_t = k_khronos_ssize_t;

pub type EGLint = i32;
pub type EGLNativeDisplayType = *const c_void;
pub type EGLNativePixmapType = *const c_void;
pub type EGLNativeWindowType = *const c_void;
pub type NativeDisplayType = *const c_void;
pub type NativePixmapType = *const c_void;
pub type NativeWindowType = *const c_void;

include!(concat!(env!("OUT_DIR"), "/egl_bindings.rs"));

mod ext_egl_bindings;
pub use self::ext_egl_bindings::*;
