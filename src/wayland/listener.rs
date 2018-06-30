use wayland_server::sys::server::wl_listener;

pub struct Listener {
    raw: wl_listener,
}