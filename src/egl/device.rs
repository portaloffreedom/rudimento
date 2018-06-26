use egl;
use egl::ffi::EGLint;
use egl::ffi::types::{EGLDeviceEXT, EGLBoolean};
use std::ffi::{CStr, CString};
use std::mem;
use egl::EGLError;

pub struct EGLDevice {
    raw_device: EGLDeviceEXT,
}

impl EGLDevice {
    pub fn from_raw(raw_device: EGLDeviceEXT) -> Self {
        Self {
            raw_device
        }
    }

    pub fn raw(&self) -> EGLDeviceEXT {
        self.raw_device
    }

    /// This function will fail for mesa EGL and nvidia EGL querying an INTEL device
    pub fn get_drm_device_file(&self) -> Result<&'static CStr, EGLError> {
        let extensions = egl::query_extensions()?;

        let mut found_EGL_EXT_device_base = false;
        let mut found_EGL_EXT_device_query = false;
        let mut found_EGL_EXT_device_enumeration = false;
        extensions.iter()
            .for_each(|ext| {
                let ext = ext.trim();
                if ext == "EGL_EXT_device_base" { found_EGL_EXT_device_base = true }
                if ext == "EGL_EXT_device_query" { found_EGL_EXT_device_query = true }
                if ext == "EGL_EXT_device_enumeration" { found_EGL_EXT_device_enumeration = true }
            });

        //TODO EGL_EXT_device_query and EGL_EXT_device_enumeration not supported on the intel?
        if !found_EGL_EXT_device_base { return Err(EGLError::from_str("EGL_EXT_device_base not supported")); }
        // if !found_EGL_EXT_device_query { return Err(EGLError::from_str("EGL_EXT_device_query not supported")); }
        // if !found_EGL_EXT_device_enumeration { return Err(EGLError::from_str("EGL_EXT_device_enumeration not supported")); }

        egl::ffi::load_QueryDeviceStringEXT()
            .map_err(|_| EGLError::from_str("QueryDeviceStringEXT is necessary"))?;

        let supported_string_ext = unsafe {
            let response = egl::ffi::QueryDeviceStringEXT(self.raw_device, egl::ffi::EXTENSIONS as egl::ffi::EGLint);
            if response.is_null() {
                let error_message = egl::egl_error::egl_error_state();
                return Err(EGLError::from_string(format!("QueryDeviceStringEXT({:?}, EXTENSIONS) returned NULL: {}", self.raw_device, error_message)))
            }
            CStr::from_ptr(response)
        };

        let found_EGL_EXT_device_drm = supported_string_ext.to_string_lossy()
            .split_whitespace()
            // .map(|a| {
            //     println!("\t- {}", a);
            //     a
            // })
            .any(|ext| ext.eq("EGL_EXT_device_drm"));

        if !found_EGL_EXT_device_drm {
            return Err(EGLError::from_str("EGL_EXT_device_drm not supported"));
        }

        let drm_device_filename = unsafe {
            // let response = egl::ffi::QueryDeviceStringEXT(self.raw_device, egl::ffi::DRM_DEVICE_FILE_EXT as egl::ffi::EGLint);
            let response = egl::ffi::QueryDeviceStringEXT(self.raw_device, 0x3233);
            if response.is_null() {
                let error_message = egl::egl_error::egl_error_state();
                return Err(EGLError::from_string(format!("QueryDeviceStringEXT({:?}, 0x{:x}) returned NULL: {}", self.raw_device, egl::ffi::DRM_DEVICE_FILE_EXT, error_message)))
            }
            CStr::from_ptr(response)
        };


        Ok(drm_device_filename)
    }
}

pub fn get_egl_devices() ->Result<Vec<EGLDevice>, EGLError> {

    let extensions = egl::query_extensions()?;

    let mut found_EGL_EXT_device_base = false;
    let mut found_EGL_EXT_device_query = false;
    let mut found_EGL_EXT_device_enumeration = false;
    extensions.iter()
        .for_each(|ext| {
            println!("Extension: \"{}\"", ext);
            let ext = ext.trim();
            if ext == "EGL_EXT_device_base" { found_EGL_EXT_device_base = true }
            if ext == "EGL_EXT_device_query" { found_EGL_EXT_device_query = true }
            if ext == "EGL_EXT_device_enumeration" { found_EGL_EXT_device_enumeration = true }
        });

    //TODO EGL_EXT_device_query and EGL_EXT_device_enumeration not supported on the intel?
    if !found_EGL_EXT_device_base { return Err(EGLError::from_str("EGL_EXT_device_base not supported")); }
    // if !found_EGL_EXT_device_query { return Err(EGLError::from_str("EGL_EXT_device_query not supported")); }
    // if !found_EGL_EXT_device_enumeration { return Err(EGLError::from_str("EGL_EXT_device_enumeration not supported")); }

    egl::ffi::load_QueryDevicesEXT()
        .map_err(|_| EGLError::from_str("QueryDevicesEXT is necessary"))?;

    let mut num_devices: EGLint = 0;
    if unsafe {egl::ffi::QueryDevicesEXT(0, 0 as *mut EGLDeviceEXT, &mut num_devices)} != egl::ffi::TRUE as u32 {
        let error_message = egl::egl_error::egl_error_state();
        return Err(EGLError::from_string(format!("device_querier(0, 0, *num_devices) call failed: {}", error_message)));
    }

    if num_devices < 1 {
        return Err(EGLError::from_str("No device found!"));
    } else {
        println!("egl devices found: {}", num_devices);
    }

    let mut devices: Vec<EGLDeviceEXT> = Vec::with_capacity(num_devices as usize);
    devices.resize(num_devices as usize, 0 as EGLDeviceEXT);

    if unsafe {egl::ffi::QueryDevicesEXT(num_devices, devices.as_mut_ptr(), &mut num_devices)} != egl::ffi::TRUE as u32 {
        let error_message = egl::egl_error::egl_error_state();
        return Err(EGLError::from_string(format!("device_querier({}, *devices, *num_devices) call failed: {}", num_devices, error_message)));
    }

    let devices = devices.into_iter().map(|d| EGLDevice::from_raw(d)).collect();

    Ok(devices)
}