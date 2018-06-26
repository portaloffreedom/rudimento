use egl;
use egl::EGLError;
use egl::device::{EGLDevice, get_egl_devices};
use renderer::Renderer;
use std::path::Path;

pub struct EGLRenderer {
    egl_device: EGLDevice,
//    opaque_attribs: egl::EGLint,
//    alpha_attribs: egl::EGLint,
//    opaque_stream_attribs: egl::EGLint,
}

impl Renderer for EGLRenderer {
}

impl EGLRenderer {
    pub fn from_drm_device_file(device_path: &Path) -> Result<Box<EGLRenderer>, EGLError> {
        load_EGL()?;
        println!("Creating EGL Renderer at {:?}", device_path);
        
        let egl_device = find_egldevice(device_path)?;
        Ok(Box::new(EGLRenderer {
            egl_device,
        }))
    }

    pub fn first_drm_device_available() -> Result<Box<EGLRenderer>, EGLError> {
        let devices = get_egl_devices()?;
        if devices.len() > 0 {
            Ok(Box::new(EGLRenderer {
                egl_device: devices.into_iter().nth(0).unwrap(),
            }))
        } else {
            Err(EGLError::from_str("No EGLDevice found"))
        }
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

fn load_EGL() -> Result<(), EGLError> {
    let library_path = "libEGL.so";
    println!("Loading EGL Library from {:?}", library_path);
    egl::loader::Load_EGL(library_path)
}

fn find_egldevice(filename: &Path) -> Result<EGLDevice, EGLError> {
    let devices = get_egl_devices()?;

    if devices.len() == 0 {
        //HACK BECAUSE WE GOT INTEL EGL PROBLEMS
        Ok(devices.into_iter().find(|_| true ).unwrap())
    } else {
        devices.into_iter()
            .find(|device| {
                match device.get_drm_device_file() {
                    Ok(drm_device) => 
                    {
                        let filename = filename.to_string_lossy();
                        filename.eq(&drm_device.to_string_lossy())
                    },
                    Err(e) => {
                        println!("ERROR READING DRM FILE FOR EGLDEVICE: {:?}", e);
                        false
                    }
                }
            })
            .ok_or_else(|| {
                EGLError::from_string(format!("Device {:?} not found", filename))
            })
    }
}
