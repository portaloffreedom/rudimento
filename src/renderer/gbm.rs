use renderer::Renderer;
use std::os::unix::io::RawFd;
use egl::types::*;

pub struct GBMRenderer {

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

impl Renderer for GBMRenderer {
    fn image_target_texture_2d(&self) {}

    fn create_image(
        &self,
        context: Option<EGLContext>, 
        target: EGLenum,
        buffer: EGLClientBuffer,
        attrib_list: &Vec<EGLint>
    ) -> EGLImageKHR
    {
        panic!("not implemented yet");
    }

    fn swap_buffers_with_damage(&self) {}
    fn create_platform_window(&self) {}
	// const has_unpack_subimage: bool;
    
    // const has_bind_display: bool;
    fn bind_display(&self) {}
    fn unbind_display(&self) {}
    fn query_buffer(&self) {}

    // const has_context_priority: bool;
	// const has_egl_image_external: bool;
	// const has_egl_buffer_age: bool;
	// const has_configless_context: bool;
	// const has_surfaceless_context: bool;

    //nvidia stuff
    // const has_egl_output_base: bool;
	// const has_egl_output_drm: bool;
	// const has_egl_output_drm_flip_event: bool;
    fn get_output_layers(&self) {}
    fn query_output_layer_attrib(&self) {}

    // const has_egl_stream: bool;
    fn create_stream(&self) {}
    fn destroy_stream(&self) {}
    fn query_stream(&self) {}

    // const has_egl_stream_producer_eglsurface: bool;
    fn create_stream_producer_surface(&self) {}

    // const has_egl_stream_consumer_egloutput: bool;
    fn stream_consumer_output(&self) {}

    //nvidia egl stream attrib
    // const has_egl_stream_attrib: bool;
    fn create_stream_attrib(&self) {}
    // const has_egl_stream_acquire_mode: bool;
    fn stream_consumer_acquire_attrib(&self) {}

    // const has_egl_stream_consumer_gltexture: bool;
    // const has_egl_wayland_eglstream: bool;
    fn stream_consumer_gltexture(&self) {}

    // const has_dmabuf_import: bool;
    // struct wl_list dmabuf_images;

    // const has_gl_texture_rg: bool;
    // struct gl_shader texture_shader_rgba;
	// struct gl_shader texture_shader_rgbx;
	// struct gl_shader texture_shader_egl_external;
	// struct gl_shader texture_shader_y_uv;
	// struct gl_shader texture_shader_y_u_v;
	// struct gl_shader texture_shader_y_xuxv;
	// struct gl_shader invert_color_shader;
	// struct gl_shader solid_shader;
	// struct gl_shader *current_shader;

	// fn destroy_signal_ref(&self) -> &wayland::signal::Signal { &self.destroy_signal }
    // fn output_destroy_listener_ref(&self) -> &wayland::listener::Listener { &self.output_destroy_listener }

    // const has_dmabuf_import_modifiers: bool;
    fn query_dmabuf_formats(&self) {}
    fn query_dmabuf_modifiers(&self) {}

    // const has_native_fence_sync: bool;
    fn create_sync(&self) {}
    fn destroy_sync(&self) {}
    fn dup_native_fence_fd(&self) {}
}

