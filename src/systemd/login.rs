use std::ffi::CString;
use std::result::Result;
use std::string::String;

use libc::{pid_t, c_int, c_char, c_uint};
use libc::getpid;

use systemd::ffi::login;

/// This function returns the systemd session for the given pid
///
/// # Examples
///
/// ```
/// use systemd::login::pid_get_session;
/// use libc::getpid;
///
/// let pid = unsafe {getpid()};
/// match pid_get_session(pid) {
///     Ok(session) => {},
///     Err(e) => panic! ,
/// }
/// ```
pub fn pid_get_session(pid: pid_t) -> Result<CString, String> {
    let mut session_c_char: *mut c_char = 0 as *mut c_char;

    let r: c_int = unsafe {
        login::sd_pid_get_session(pid, &mut session_c_char as *mut *mut c_char)
    };

    if r < 0 {
        // error
        let detail_message: &str;

        use libc::{ESRCH, EBADF, ENODATA, EINVAL, ENOMEM};
        if r == -ESRCH {
            detail_message = "The specified PID does not refer to a running process.";
        } else if r == -EBADF {
            detail_message = "The specified socket file descriptor was invalid.";
        } else if r == -ENODATA {
            detail_message = "The given field is not specified for the described process or peer.";
        } else if r == -EINVAL {
            detail_message = "An input parameter was invalid (out of range, or NULL, where that is not accepted).";
        } else if r == -ENOMEM {
            detail_message = "Memory allocation failed.";
        } else {
            detail_message = "Unknown error.";
        }

        Err(format!("logind: failed to get session from pid. {}", detail_message))

    } else {
        let session = unsafe {CString::from_raw(session_c_char)};
        Ok(session)
    }
}

pub fn get_session() -> Result<CString, String> {
    let pid: pid_t = unsafe {getpid()};
    pid_get_session(pid)
}

pub fn session_get_seat(session_id: &String) -> Result<CString, String> {
    let mut seat_c_char: *mut c_char = 0 as *mut c_char;
    let session_id_cstring = CString::new(session_id.as_str()).unwrap();

    let r: c_int = unsafe {
        login::sd_session_get_seat(session_id_cstring.as_ptr() as *const c_char, &mut seat_c_char as *mut *mut c_char)
    };

    if r < 0 {
        // error
        let detail_message: &str;

        use libc::{ENXIO, ENODATA, EINVAL, ENOMEM};
        if r == -ENXIO {
            detail_message = "The specified session does not exist.";
        } else if r == -ENODATA {
            detail_message = "The given field is not specified for the described session.";
        } else if r == -EINVAL {
            detail_message = "An input parameter was invalid (out of range, or NULL, where that is not accepted).";
        } else if r == -ENOMEM {
            detail_message = "Memory allocation failed.";
        } else {
            detail_message = "Unknown error.";
        }

        Err(format!("logind: failed to get session seat. {}", detail_message))

    } else {
        let seat = unsafe {CString::from_raw(seat_c_char)};
        Ok(seat)
    }
}

pub fn session_get_vt(session_id: &String) -> Result<u32, String> {
    let r: c_int;
    let mut vt: c_uint = 0;
    let session_id_cstring = CString::new(session_id.as_str()).unwrap();

    unsafe {
        r = login::sd_session_get_vt(session_id_cstring.as_ptr() as *const c_char, &mut vt as *mut c_uint);
    }

    if r < 0 {
        Err("logind: session not running on a VT".to_string())
    } else {
        Ok(vt)
    }
}

