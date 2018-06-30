use libc;
use egl::EGLError;
use std::os;
use std::ffi::{CStr, CString};

static mut EGL_LOADER: Option<EglLoader> = None;

#[allow(non_snake_case)]
pub fn load_EGL(lib_path: &str) -> Result<(), EGLError> {
    let egl_loader = unsafe { &mut EGL_LOADER };
    if egl_loader.is_none() {
        let _egl_loader = EglLoader::new(lib_path).map_err(|e| {
            EGLError::from_string(format!("Impossible to load EGL Library: {:?}", e))
        })?;

        ::egl::ffi::load_with(|s| _egl_loader.load_fn(s) as *const os::raw::c_void);

        *egl_loader = Some(_egl_loader);
    }

    Ok(())
}

struct EglLoader {
    lib: *mut libc::c_void
}

impl EglLoader {
    fn new(lib_path: &str) -> Result<EglLoader, &CStr> {
        let path_cstring = CString::new(lib_path).unwrap();
        let flags: libc::c_int = libc::RTLD_NOW;

        let lib = unsafe{ libc::dlopen(path_cstring.as_ptr(), flags) };
        if lib.is_null() {
            let err_string = unsafe{ CStr::from_ptr( libc::dlerror() )};
            return Err( err_string );
        }

        Ok(EglLoader {
            lib: lib,
        })
    }

    fn load_fn(&self, sym_name: &str) -> *const libc::c_void {
        let sym_name = CString::new(sym_name).unwrap();
        unsafe{ libc::dlsym(self.lib, sym_name.as_ptr()) }
    }
}