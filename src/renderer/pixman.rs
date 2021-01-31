// use renderer::Renderer;
// use egl::types::*;
use wayland;
// use wayland_server;

// Capabilites:
//  WESTON_CAP_ROTATION_ANY
//  WESTON_CAP_CAPTURE_YFLIP
//  WESTON_CAP_VIEW_CLIP_MASK


pub struct PixmanRenderer {
    // base: weston_renderer,
    // repaint_debug: bool,
    // debug_color: pixman_image_t,
    // debug_binding: weston_binding,
    destroy_signal: wayland::signal::Signal,
}

impl PixmanRenderer {
    pub fn new(display: &mut wayland::display::Display) 
        -> Self
    {
        use wayland_server::protocol::wl_shm;
        display.add_shm_format(wl_shm::Format::Rgb565).unwrap();


        Self {
            destroy_signal: wayland::signal::Signal::new(),
        }
    }

    pub fn read_pixels(&self) { unimplemented!() }
    pub fn repaint_output(&self) { unimplemented!() }
    pub fn flush_damage(&self) { unimplemented!() }
    pub fn attach(&self) { unimplemented!() }
    pub fn surface_set_color(&self) { unimplemented!() }
    pub fn destroy(&self) { unimplemented!() } //TODO drop?
    pub fn surface_get_content_size(&self) { unimplemented!() }
    pub fn surface_copy_content(&self) { unimplemented!() }

}

// impl Renderer for PixmanRenderer {
//     fn image_target_texture_2d(&self) {}

//     fn create_image(
//         &self,
//         context: Option<EGLContext>, 
//         target: EGLenum,
//         buffer: EGLClientBuffer,
//         attrib_list: &Vec<EGLint>
//     ) -> EGLImageKHR
//     {
//         panic!("not implemented yet");
//     }

//     fn swap_buffers_with_damage(&self) {}
//     fn create_platform_window(&self) {}
// 	// const has_unpack_subimage: bool;
    
//     // const has_bind_display: bool;
//     fn bind_display(&self) {}
//     fn unbind_display(&self) {}
//     fn query_buffer(&self) {}

//     // const has_context_priority: bool;
// 	// const has_egl_image_external: bool;
// 	// const has_egl_buffer_age: bool;
// 	// const has_configless_context: bool;
// 	// const has_surfaceless_context: bool;

//     //nvidia stuff
//     // const has_egl_output_base: bool;
// 	// const has_egl_output_drm: bool;
// 	// const has_egl_output_drm_flip_event: bool;
//     fn get_output_layers(&self) {}
//     fn query_output_layer_attrib(&self) {}

//     // const has_egl_stream: bool;
//     fn create_stream(&self) {}
//     fn destroy_stream(&self) {}
//     fn query_stream(&self) {}

//     // const has_egl_stream_producer_eglsurface: bool;
//     fn create_stream_producer_surface(&self) {}

//     // const has_egl_stream_consumer_egloutput: bool;
//     fn stream_consumer_output(&self) {}

//     //nvidia egl stream attrib
//     // const has_egl_stream_attrib: bool;
//     fn create_stream_attrib(&self) {}
//     // const has_egl_stream_acquire_mode: bool;
//     fn stream_consumer_acquire_attrib(&self) {}

//     // const has_egl_stream_consumer_gltexture: bool;
//     // const has_egl_wayland_eglstream: bool;
//     fn stream_consumer_gltexture(&self) {}

//     // const has_dmabuf_import: bool;
//     // struct wl_list dmabuf_images;

//     // const has_gl_texture_rg: bool;
//     // struct gl_shader texture_shader_rgba;
// 	// struct gl_shader texture_shader_rgbx;
// 	// struct gl_shader texture_shader_egl_external;
// 	// struct gl_shader texture_shader_y_uv;
// 	// struct gl_shader texture_shader_y_u_v;
// 	// struct gl_shader texture_shader_y_xuxv;
// 	// struct gl_shader invert_color_shader;
// 	// struct gl_shader solid_shader;
// 	// struct gl_shader *current_shader;

// 	// fn destroy_signal_ref(&self) -> &wayland::signal::Signal { &self.destroy_signal }
//     // fn output_destroy_listener_ref(&self) -> &wayland::listener::Listener { &self.output_destroy_listener }

//     // const has_dmabuf_import_modifiers: bool;
//     fn query_dmabuf_formats(&self) {}
//     fn query_dmabuf_modifiers(&self) {}

//     // const has_native_fence_sync: bool;
//     fn create_sync(&self) {}
//     fn destroy_sync(&self) {}
//     fn dup_native_fence_fd(&self) {}
// }
