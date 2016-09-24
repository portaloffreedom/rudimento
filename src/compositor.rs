use std::mem;

use libc;
use libc::clockid_t;

pub struct Compositor {
    presentation_clock: clockid_t,
}

impl Compositor {
    pub fn new(clock_id: clockid_t) -> Result<Compositor, String> {
        // test gettime
        let mut time: libc::timespec = unsafe {mem::zeroed()};

        let r = unsafe {libc::clock_gettime(clock_id, &mut time as *mut libc::timespec)};
        if r < 0 {
            return Err("Error retriving time, clock_id probably invalid".to_string());
        }

        Ok(Compositor {
            presentation_clock: clock_id,
        })
    }
}
