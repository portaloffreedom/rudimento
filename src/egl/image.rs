use egl;
use egl::ffi::types;
use egl::ffi::types::EGLImageKHR;
use egl::display::EGLDisplay;
use egl::EGLError;
use renderer::image::Image as RendererImage;

#[derive(Debug)]
pub struct Image<'display> {
    display: &'display EGLDisplay,
    raw: EGLImageKHR,
}

impl<'a> RendererImage<'a> for Image<'a> {}

impl<'display> Image<'display> {
    pub fn new (
        display: &'display EGLDisplay,
        context: Option<types::EGLContext>,
        target: types::EGLenum,
        buffer: types::EGLClientBuffer,
        attrib_list: &Vec<types::EGLint>
    ) -> Result<Box<Self>, EGLError>
    {
        let context = context.unwrap_or(egl::ffi::NO_CONTEXT);
        let raw = unsafe { egl::ffi::CreateImageKHR(
            display.raw_ref().clone(),
            context,
            target, 
            buffer, 
            attrib_list.as_ptr())
        };

        if raw.is_null() {
            Err(EGLError::from_string(format!("Failed to create image for display {:?}", display)))
        } else {
            Ok(Box::new(Self {
                display,
                raw,
            }))
        }
    }

    pub fn raw_ref(&self) -> &EGLImageKHR {
        &self.raw
    }

    pub fn raw_mut_ref(&mut self) -> &mut EGLImageKHR {
        &mut self.raw
    }

    pub fn raw(&self) -> EGLImageKHR {
        self.raw
    }
}

impl<'a> Drop for Image<'a> {
    fn drop(&mut self) {
        let r = unsafe { egl::ffi::DestroyImageKHR(
            *self.display.raw_ref(),
            self.raw,
        )};

        if r != egl::ffi::TRUE {
            println!("FAILED TO DESTROY IMAGE {:?}", self)
        }
    }
}