extern crate gl_generator;
extern crate pkg_config;


use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn generate_gl(api: Api,  version: (u8, u8), filename: &str) {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join(filename)).unwrap();

    Registry::new(api, version, Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}

fn generate_mods() {
    generate_gl(Api::Gl, (4, 5), "gl_bindings.rs");
    generate_gl(Api::Egl, (1, 5), "egl_bindings.rs");
}

fn link_c_libraries() {
    pkg_config::find_library("libsystemd").unwrap();
}

fn main() {
    generate_mods();
    link_c_libraries();
}
