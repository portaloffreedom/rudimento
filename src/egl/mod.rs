mod loader;
pub mod renderer;
mod device;
mod ffi;
mod egl_error;
mod display;

pub use self::egl_error::EGLError;


use std::ffi::CStr;

pub fn query_extensions<'a>() -> Result<Vec<String>, EGLError> {
    let extensions = unsafe{ffi::QueryString(ffi::NO_DISPLAY, ffi::EXTENSIONS as ffi::EGLint)};
    if extensions as usize == 0 {
        return Err(EGLError::from_str("Retrieving EGL extension string failed."));
    }

    // This sucks as perfomance
    let extensions: Vec<String> = unsafe {CStr::from_ptr(extensions)}
        .to_string_lossy()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    // println!("ALL EXTENSIONS: {:?}", extensions);

    Ok(extensions)
}
