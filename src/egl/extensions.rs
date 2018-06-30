use egl::display::EGLDisplay;
use egl::EGLError;
use egl::ffi;
use std::collections::HashSet;
use std::ffi::CStr;

pub struct Extensions(HashSet<String>);

impl Extensions {
    pub fn query<'a>(display: Option<&EGLDisplay>) -> Result<Self, EGLError> {
        println!("EXTENSIONS FOR DISPLAY {:?}", display);
        let display = display.map(|d| d.raw_ref().clone())
            .unwrap_or(ffi::NO_DISPLAY);

        let extensions = unsafe{ffi::QueryString(display, ffi::EXTENSIONS as ffi::EGLint)};
        if extensions as usize == 0 {
            return Err(EGLError::from_str("Retrieving EGL extension string failed."));
        }

        // This sucks as perfomance
        let extensions: HashSet<String> = unsafe {CStr::from_ptr(extensions)}
            .to_string_lossy()
            .split_whitespace()
            // .map(|s| {println!("{}", s); s})
            .map(|s| s.trim().to_string())
            .collect();

        // println!("ALL EXTENSIONS: {:?}", extensions);

        Ok(Extensions(extensions))
    }

    pub fn contains(&self, extension: &str) -> bool
    {
        self.0.contains(extension)
    }
}