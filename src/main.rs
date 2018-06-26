extern crate khronos;
extern crate libc;
extern crate libudev;
extern crate dbus;
extern crate drm as libdrm;

mod compositor;
mod systemd;
mod backend;
mod launcher;
mod renderer;
mod egl;
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

use backend::drm;
use backend::Backend;

fn main() {
    let use_pixman = false;
    let use_egldevice = true;

    let backend = match drm::DRMBackend::load_backend(use_pixman, use_egldevice) {
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
