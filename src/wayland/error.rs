use std::fmt;
use std::ffi::CStr;

#[derive(Debug)]
pub struct WaylandError {
    message: String,
}

impl WaylandError {
    pub fn from_string(message: String) -> Self {
        Self {
            message,
        }
    }

    pub fn from_str(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn from_cstr(message: &CStr) -> Self {
        //TODO make this better
        Self {
            message: format!("{:?}", message),
        }
    }
}

impl fmt::Display for WaylandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WaylandError: {}", self.message)
    }
}