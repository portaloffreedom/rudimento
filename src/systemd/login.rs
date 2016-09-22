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
    let r: c_int;
    let mut session_c_char: *mut c_char = 0 as *mut c_char;
    let session: CString;

    unsafe {
        r = login::sd_pid_get_session(pid, &mut session_c_char as *mut *mut c_char);
        session = CString::from_raw(session_c_char);
    }

    if r < 0 {
        // error
        Err("logind: not running in a systemd session".to_string())
    } else {
        Ok(session)
    }
}

pub fn get_session() -> Result<CString, String> {
    let pid: pid_t = unsafe {getpid()};
    pid_get_session(pid)
}

pub fn session_get_seat(session_id: &String) -> Result<CString, String> {
    let r: c_int;
    let mut seat_c_char: *mut c_char = 0 as *mut c_char;
    let seat: CString;

    unsafe {
        r = login::sd_session_get_seat(session_id.as_ptr() as *const c_char, &mut seat_c_char as *mut *mut c_char);
        seat = CString::from_raw(seat_c_char);
    }

    if r < 0 {
        // error
        Err("logind: failed to get session seat".to_string())
    } else {
        Ok(seat)
    }
}

pub fn session_get_vt(session_id: &String) -> Result<u32, String> {
    let r: c_int;
    let mut vt: c_uint = 0;

    unsafe {
        r = login::sd_session_get_vt(session_id.as_ptr() as *const c_char, &mut vt as *mut c_uint);
    }

    if r < 0 {
        Err("logind: session not running on a VT".to_string())
    } else {
        Ok(vt)
    }
}

