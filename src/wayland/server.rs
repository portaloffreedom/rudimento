use std::ffi::{OsStr, OsString};
use std::time::Duration;
use drm::DRMDevice;

pub struct RudimentoServer {
    pub display: wayland_server::Display,
    pub event_loop: wayland_server::calloop::EventLoop<()>,
    pub socket_name: OsString
}

impl RudimentoServer {
    pub fn new_from_drm(drm: DRMDevice) -> Self {
        let event_loop = wayland_server::calloop::EventLoop::<()>::new().unwrap();
        let mut display = wayland_server::Display::new(event_loop.handle());
        let socket_name = display
            .add_socket_auto()
            .expect("Failed to create a server socket.");

        Self {
            display: display,
            event_loop: event_loop,
            socket_name: socket_name,
        }
    }
    
    pub fn answer(&mut self) {
        self.event_loop
            .dispatch(Some(Duration::from_millis(10)), &mut ())
            .unwrap();
        self.display.flush_clients();
        // TODO: find out why native_lib requires two dispatches
        self.event_loop
            .dispatch(Some(Duration::from_millis(10)), &mut ())
            .unwrap();
        self.display.flush_clients();
    }
}