use std::ffi::CStr;
use egl;
use egl::ffi::types::{EGLint, EGLenum};

#[derive(Debug)]
pub struct EGLError {
    message: String,
}

impl EGLError {
    pub fn from_string(message: String) -> Self {
        Self {
            message
        }
    }

    pub fn from_str(message: &str) -> Self {
        Self {
            message: message.to_string()
        }
    }

    pub fn from_cstr(message: &CStr) -> Self {
        //TODO make this better
        Self {
            message: format!("{:?}", message)
        }
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

pub fn egl_error_state() -> String
{
	let code = unsafe {egl::ffi::GetError()};
	format!("EGL error state: {} ({:?})\n", egl_error_string(code), code)
}