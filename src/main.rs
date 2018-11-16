#![recursion_limit="2048"]

extern crate khronos;
extern crate libc;
extern crate libudev;
extern crate dbus;
extern crate drm as libdrm;
extern crate wayland_server;
#[macro_use]
extern crate mashup;
extern crate gbm;

mod compositor;
mod systemd;
mod backend;
mod launcher;
mod renderer;
mod wayland;
mod egl;
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

use backend::drm;
use backend::Backend;

fn main() {
    let use_pixman = false;
    let use_egldevice = false;
    let tty = 3;

    let backend = match drm::DRMBackend::new(tty, use_pixman, use_egldevice) {
        Ok(b) => b,
        Err(error) => {
            println!("{}", error.description());
            println!("exiting now");
            std::process::exit(1);
        }
    };

    // parse args

    // if help, print help

    // if version, print version


    println!("Hello, world!");
}
