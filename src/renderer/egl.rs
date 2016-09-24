use renderer::Renderer;
use egl;
use egl::{EGLint};
use egl::types::{EGLDeviceEXT, EGLBoolean};

use std::ffi::{CStr, CString};
use std::path::Path;
use std::mem;
use libc;

pub struct EglRenderer {
//    opaque_attribs: egl::EGLint,
//    alpha_attribs: egl::EGLint,
//    opaque_stream_attribs: egl::EGLint,
}

impl Renderer for EglRenderer {

}

impl EglRenderer {
    pub fn new(device_path: &Path) -> Result<EglRenderer,String> {

        let egl_device = try!(find_egldevice(device_path));
        Ok(EglRenderer {

        })
    }

    pub fn display_create() {

    }

    pub fn display(&self) /*-> egl::EGLDisplay*/ {

    }

    pub fn output_window_create() {

    }

    pub fn output_destroy() {

    }

    pub fn output_surface() /*-> egl::EGLSurface*/ {

    }

    pub fn output_set_border() {

    }

    pub fn print_egl_error_state() {

    }


    pub fn get_drm_device_file() {

    }

    pub fn output_stream_flip() {

    }
}

fn find_egldevice(filename: &Path) -> Result<egl::types::EGLDeviceEXT, String> {
    let devices = try!(get_egl_devices(0));
    unsafe{libc::free(devices as *mut libc::c_void)};

    //.filter(get drm_device_file() filename == drm_path)

    Err("not implemented yet".to_string())
}

pub fn get_egl_devices(max_devices: EGLint) ->Result<*mut EGLDeviceEXT, String> {

    let extensions = unsafe{egl::QueryString(egl::NO_DISPLAY, egl::EXTENSIONS as EGLint)};
    if extensions as usize == 0 {
        return Err("Retrieving EGL extension string failed.".to_string());
    }

    let all_extensions = unsafe {CStr::from_ptr(extensions)}
        .to_string_lossy().into_owned();

    println!("ALL EXTENSIONS: {:?}", all_extensions);

    //TODO check extensions
    // if !check_extension(all_extensions, "EGL_EXT_device_base") &&
    //    !check_extension(all_extensions, "EGL_EXT_device_query") ||
    //    !check_extension(all_extensions, "EGL_EXT_device_enumeration")
    // {
    //     return Err("EGL_EXT_device_base not supported".to_string());
    // }

    let egldevice: EGLDeviceEXT;

    let eglQueryDevicesEXT_cstring = CString::new("eglQueryDevicesEXT").unwrap();

    let deviceQuerier = unsafe {
        mem::transmute::<_, unsafe extern "system" fn(EGLint, *mut EGLDeviceEXT, *mut EGLint) -> EGLBoolean>
        (egl::GetProcAddress(eglQueryDevicesEXT_cstring.as_ptr()))
    };

    let mut num_devices: EGLint = 0;
    if unsafe {deviceQuerier(0, 0 as *mut EGLDeviceEXT, &mut num_devices)} != egl::TRUE as u32 {
        return Err("deviceQuerier(0, 0, *num_devices) call failed".to_string());
    }

    //TODO take this using a small C code
    let size_of_EGLDeviceEXT: usize = 8; // HACK!!!

    println!("egl devices found: {}", num_devices);
    println!("size of EGLDeviceEXT: {}", size_of_EGLDeviceEXT);
    println!("malloc -> {}", num_devices * size_of_EGLDeviceEXT as i32);

    let device_malloc_size: usize = (num_devices * size_of_EGLDeviceEXT as i32) as usize;

    let mut devices: *mut EGLDeviceEXT = unsafe {libc::malloc(device_malloc_size)} as *mut EGLDeviceEXT;
    if unsafe {deviceQuerier(max_devices, devices, &mut num_devices)} != egl::TRUE as u32 {
        return Err(format!("deviceQuerier({}, *devices, *num_devices) call failed", max_devices));
    }

    Ok(devices)
}
