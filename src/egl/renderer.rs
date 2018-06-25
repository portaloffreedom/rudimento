use renderer::Renderer;
use egl;
use egl::{EGLint};
use egl::types::{EGLDeviceEXT, EGLBoolean};

use std::ffi::{CStr, CString};
use std::path::Path;
use std::mem;
use libc;

pub struct EglRenderer {
    egl_device: EGLDeviceEXT,
//    opaque_attribs: egl::EGLint,
//    alpha_attribs: egl::EGLint,
//    opaque_stream_attribs: egl::EGLint,
}

impl Renderer for EglRenderer {
}

impl EglRenderer {
    pub fn new(device_path: &Path) -> Result<Box<EglRenderer>,String> {
        let library_path = "/usr/lib/libEGL.so";
        println!("Loading EGL Library from {:?}", library_path);
        egl::loader::Load_EGL(library_path).map_err(|e| {
            format!("Impossible to load EGL Library: {:?}", e)
        })?;

        println!("Creating EGL Renderer at {:?}", device_path);
        
        let egl_device = find_egldevice(device_path)?;
        Ok(Box::new(EglRenderer {
            egl_device,
        }))
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

    pub fn get_devices() {

    }

    pub fn get_drm_device_file() {

    }

    pub fn output_stream_flip() {

    }
}

fn find_egldevice(filename: &Path) -> Result<egl::types::EGLDeviceEXT, String> {
    let devices = get_egl_devices()?;
    //.filter(get drm_device_file() filename == drm_path)

    //TODO let the compiler decide when to do this
    unsafe{libc::free(devices as *mut libc::c_void)};

    Err("not implemented yet".to_string())
}

pub fn get_egl_devices() ->Result<*mut EGLDeviceEXT, String> {

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

    let eglQueryDevicesEXT_cstring = CString::new("eglQueryDevicesEXT").unwrap();

    let device_querier = unsafe {
        mem::transmute::<_, unsafe extern "system" fn(EGLint, *mut EGLDeviceEXT, *mut EGLint) -> EGLBoolean>
        (egl::GetProcAddress(eglQueryDevicesEXT_cstring.as_ptr()))
    };

    let mut num_devices: EGLint = 0;
    if unsafe {device_querier(0, 0 as *mut EGLDeviceEXT, &mut num_devices)} != egl::TRUE as u32 {
        return Err("device_querier(0, 0, *num_devices) call failed".to_string());
    }

    if num_devices < 1 {
        return Err("No device found! ".to_string());
    }

    //TODO take this using a small C code
    let size_of_EGLDeviceEXT: usize = 8; // HACK!!!

    println!("egl devices found: {}", num_devices);
    println!("size of EGLDeviceEXT: {}", size_of_EGLDeviceEXT);
    println!("malloc -> {}", num_devices * size_of_EGLDeviceEXT as i32);

    let device_malloc_size: usize = (num_devices * size_of_EGLDeviceEXT as i32) as usize;

    let devices: *mut EGLDeviceEXT = unsafe {libc::malloc(device_malloc_size)} as *mut EGLDeviceEXT;

    if devices.is_null() {
        return Err("Error allocating space for EGL Devices scan".to_string());
    }

    if unsafe {device_querier(num_devices, devices, &mut num_devices)} != egl::TRUE as u32 {
        unsafe{libc::free(devices as *mut libc::c_void)};
        return Err(format!("device_querier({}, *devices, *num_devices) call failed", num_devices));
    }

    Ok(devices)
}
