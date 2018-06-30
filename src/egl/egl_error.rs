use std::ffi::CStr;
use std::fmt;
use egl;
use egl::ffi::types::{EGLint, EGLenum};

#[derive(Debug)]
pub struct EGLError {
    message: String,
    code: EGLint,
}

impl EGLError {
    pub fn from_string(message: String) -> Self {
        Self {
            message,
            code: get_error(),
        }
    }

    pub fn from_str(message: &str) -> Self {
        Self {
            message: message.to_string(),
            code: get_error(),
        }
    }

    pub fn from_cstr(message: &CStr) -> Self {
        //TODO make this better
        Self {
            message: format!("{:?}", message),
            code: get_error(),
        }
    }
}

impl fmt::Display for EGLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EGLError {}: {}", self.message, egl_error_string(self.code))
    }
}

fn egl_error_string(code: EGLint) -> &'static str
{
	match code as EGLenum {
        egl::ffi::SUCCESS => "SUCCESS",
        egl::ffi::NOT_INITIALIZED => "NOT_INITIALIZED",
        egl::ffi::BAD_ACCESS => "BAD_ACCESS",
        egl::ffi::BAD_ALLOC => "BAD_ALLOC",
        egl::ffi::BAD_ATTRIBUTE => "BAD_ATTRIBUTE",
        egl::ffi::BAD_CONTEXT => "BAD_CONTEXT",
        egl::ffi::BAD_CONFIG => "BAD_CONFIG",
        egl::ffi::BAD_CURRENT_SURFACE => "BAD_CURRENT_SURFACE",
        egl::ffi::BAD_DISPLAY => "BAD_DISPLAY",
        egl::ffi::BAD_SURFACE => "BAD_SURFACE",
        egl::ffi::BAD_MATCH => "BAD_MATCH",
        egl::ffi::BAD_PARAMETER => "BAD_PARAMETER",
        egl::ffi::BAD_NATIVE_PIXMAP => "BAD_NATIVE_PIXMAP",
        egl::ffi::BAD_NATIVE_WINDOW => "BAD_NATIVE_WINDOW",
        egl::ffi::CONTEXT_LOST => "CONTEXT_LOST",
        _ => "unknown",
    }
}

fn get_error() -> EGLint {
    unsafe { egl::ffi::GetError()}
}

pub fn egl_error_state() -> String
{
	let code = get_error();
	format!("EGL error state: {} ({:?})\n", egl_error_string(code), code)
}