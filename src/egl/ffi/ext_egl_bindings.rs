use super::*;

#[allow(dead_code, non_upper_case_globals)] pub const DRM_DEVICE_FILE_EXT: types::EGLenum = 0x3233;


#[allow(non_snake_case, dead_code)] #[inline]
pub unsafe fn QueryDeviceStringEXT(device: types::EGLDeviceEXT, name: types::EGLint) -> *const __gl_imports::raw::c_char {
    self::__gl_imports::mem::transmute::<_, extern "system" fn(types::EGLDeviceEXT, types::EGLint) -> *const __gl_imports::raw::c_char>
        (ext_storage::QueryDeviceStringEXT.f)(device, name)
}

#[allow(non_snake_case, dead_code)] #[inline]
pub unsafe fn QueryDevicesEXT(max_devices: types::EGLint, devices: *mut types::EGLDeviceEXT, num_devices: *mut types::EGLint) -> types::EGLBoolean {
    __gl_imports::mem::transmute::<_, extern "system" fn(types::EGLint, *mut types::EGLDeviceEXT, *mut types::EGLint) -> types::EGLBoolean>
        (ext_storage::QueryDevicesEXT.f)(max_devices, devices, num_devices)
}

#[allow(non_snake_case, dead_code)] #[inline]
unsafe fn load_GetProcAddress(function_name: &str) -> Result<*const __gl_imports::raw::c_void, ()> {
    use std::ffi::CString;
    let function_name_cstring = CString::new(function_name).unwrap();
    let fun_ptr = super::GetProcAddress(function_name_cstring.as_ptr()) as *const __gl_imports::raw::c_void;

    if fun_ptr.is_null() {
        Err(())
    } else {
        Ok(fun_ptr)
    }
}

impl FnPtr {
    #[allow(non_snake_case, dead_code)] #[inline]
    pub fn load_GetProcAddress(&mut self, function_name: &str) -> Result<(), ()> {
        if !self.is_loaded {
                self.f = unsafe { load_GetProcAddress(function_name)? };
                self.is_loaded = true;
        };
        Ok(())
    }
}

#[allow(non_snake_case, dead_code)] #[inline]
pub fn load_QueryDeviceStringEXT() -> Result<(), ()> {
    unsafe { ext_storage::QueryDeviceStringEXT.load_GetProcAddress("eglQueryDeviceStringEXT")}
}

#[allow(non_snake_case, dead_code)] #[inline]
pub fn load_QueryDevicesEXT() -> Result<(), ()> {
    unsafe { ext_storage::QueryDevicesEXT.load_GetProcAddress("eglQueryDevicesEXT")}
}

mod ext_storage {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use super::__gl_imports::raw;
    use super::FnPtr;
    pub static mut QueryDeviceStringEXT: FnPtr = FnPtr {
        f: super::missing_fn_panic as *const raw::c_void,
        is_loaded: false
    };
    pub static mut QueryDevicesEXT: FnPtr = FnPtr {
        f : super::missing_fn_panic as *const raw::c_void,
        is_loaded: false
    };
}