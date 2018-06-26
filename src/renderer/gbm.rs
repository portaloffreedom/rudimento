use renderer::Renderer;
use std::os::unix::io::RawFd;

pub struct GBMRenderer {

}

impl Renderer for GBMRenderer {

}


impl GBMRenderer {
    pub fn new(fd: RawFd) -> Result<Box<Self>, String> { 
        //TODO find gbm library in crates.io
        //TODO load libglapi.so.0

        //TODO device = gbm_create_device(fd);

        //TODO init renderer
        //		EGLint format[3] = {
		// 	b->gbm_format,
		// 	fallback_format_for(b->gbm_format),
		// 	0,
		// };
		// int n_formats = 2;
        //
		// if (format[1])
		// 	n_formats = 3;
        //
		// return gl_renderer->display_create(b->compositor,
		// 				   EGL_PLATFORM_GBM_KHR,
		// 				   (void *)b->gbm,
		// 				   NULL,
		// 				   gl_renderer->opaque_attribs,
		// 				   format,
		// 				   n_formats);

        Err("GBMRenderer not yet implemtended".to_string())
    }
}
