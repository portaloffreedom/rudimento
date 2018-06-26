use super::*;

#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_DEVICE_EXT: types::EGLenum = 0x313F;
#[allow(dead_code, non_upper_case_globals)] pub const PLATFORM_GBM_KHR:    types::EGLenum = 0x31D7;
#[allow(dead_code, non_upper_case_globals)] pub const DRM_DEVICE_FILE_EXT: types::EGLenum = 0x3233;
#[allow(dead_code, non_upper_case_globals)] pub const DRM_MASTER_FD_EXT:   types::EGLenum = 0x333C;


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
pub unsafe fn GetPlatformDisplayEXT(platform: types::EGLenum, native_display: *const __gl_imports::raw::c_void, attrib_list: *const types::EGLint) -> types::EGLDisplay {
    __gl_imports::mem::transmute::<_, extern "system" fn(types::EGLenum, *const __gl_imports::raw::c_void, *const types::EGLint) -> types::EGLDisplay>
        (ext_storage::GetPlatformDisplayEXT.f)(platform, native_display, attrib_list)
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

#[allow(non_snake_case, dead_code)] #[inline]
pub fn load_GetPlatformDisplayEXT() -> Result<(), ()> {
    unsafe { ext_storage::GetPlatformDisplayEXT.load_GetProcAddress("eglGetPlatformDisplayEXT")}
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
    pub static mut GetPlatformDisplayEXT: FnPtr = FnPtr {
        f: super::missing_fn_panic as *const raw::c_void,
        is_loaded: false
    };
}