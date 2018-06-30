use wayland_server::sys::{
    common::wl_list,
    server::wl_signal,
    server::signal::wl_signal_init,
};

pub struct Signal {
    raw: wl_signal,
}

impl Signal {
    pub fn new() -> Self {
        let mut raw: wl_signal = wl_signal{
            listener_list: wl_list {
                prev: 0 as *mut wl_list,
                next: 0 as *mut wl_list,
            },
        };
        unsafe { wl_signal_init(&mut raw) };
        Self {
            raw,
        }
    }
}