use renderer::Renderer;
use backend::drm::DRMDevice;
use std::os::unix::io::{AsRawFd, RawFd};
use egl::types::*;
use libc;
use std::ffi::{CString, CStr};
use super::image;
use gbm;
use libdrm;

pub struct GBMRenderer {
    device: gbm::Device<DRMDevice>,
}

impl GBMRenderer {
    pub fn new(drm_device: DRMDevice) -> Result<Box<Self>, String> {

        let libname = CString::new("libglapi.so.0").expect("CString::new failed");;
        let r: *mut libc::c_void = unsafe { 
            libc::dlopen(libname.as_ptr(), libc::RTLD_LAZY | libc::RTLD_GLOBAL)
        };

        if r.is_null() {
            let error = unsafe { CStr::from_ptr(libc::dlerror()) };
            return Err(format!("Error loading \"libglapi.so.0\" dlerror:\n{:?}", error))
        }

        // Get a set of all modesetting resource handles (excluding planes):
        use libdrm::control::Device as libdrm_device;
        let res_handles = drm_device.resource_handles().unwrap();

        println!("\nConnector Informations");
        // Print all connector information
        for &con in res_handles.connectors() {
            let info: libdrm::control::connector::Info 
                = drm_device.resource_info(con).unwrap();

            println!("{:?}", info);
        }

        let con = res_handles.connectors()[0];
        let connector_info: libdrm::control::connector::Info
            = drm_device.resource_info(con).unwrap();
        let mode = connector_info.modes()[0];
        let (hdisplay, vdisplay) = mode.size();

        println!("\nCRTCs Informations");
        // Print all CRTC information
        for &crtc in res_handles.crtcs() {
            let info: drm::control::crtc::Info 
                = drm_device.resource_info(crtc).unwrap();

            println!("{:?}", info);
        }

        let crtc_handle = res_handles.crtcs()[0];
    
        // Create a DB of size 1920x1080
        use libdrm::control::dumbbuffer;
        use libdrm::buffer::PixelFormat;
        let mut db = dumbbuffer::DumbBuffer::create_from_device(&drm_device, (hdisplay.into(), vdisplay.into()), PixelFormat::XRGB8888)
            .expect("Could not create dumb buffer");

        // Map it and grey it out.
        {
            let mut map = db.map(&drm_device).expect("Could not map dumbbuffer");
            for mut b in map.as_mut() {
                *b = 128; // Grey
            }
        }

        // create a framebuffer from our buffer
        use libdrm::control::{crtc, framebuffer};
        let fb_info = framebuffer::create(&drm_device, &db)
            .expect("could not create FrameBuffer");

        println!("\n\nMode, fbinfo, db");
        println!("{:#?}", mode);
        println!("{:#?}", fb_info);
        println!("{:#?}", db);

        use libdrm::control::ResourceInfo;
        crtc::set(
            &drm_device,
            crtc_handle,
            fb_info.handle(),
            &[con],
            (0,0),
            Some(mode),
        )
        .expect("Could not set CRTC");

        use std::thread;
        thread::sleep_ms(2000);

        framebuffer::destroy(&drm_device, fb_info.handle()).unwrap();
        db.destroy(&drm_device).unwrap();

        let gbm = gbm::Device::new(drm_device)
        // unsafe: device has to outlive file descriptor
        // let gbm = unsafe { gbm::Device::new_from_fd(drm_devide.as_raw_fd()) }
            .map_err(|e| format!("Could not create GDB Device: {}", e))?;

        // use libdrm::control::{crtc, framebuffer};
        // use gbm::{Device, Format, BufferObjectFlags};
        // let mut bo = gbm.create_buffer_object::<()>(
        //     1920, 1080,
        //     Format::ARGB8888,
        //     BufferObjectFlags::SCANOUT,
        //     ).unwrap();

        // for i in 0..10 {
        //     // write something to it (usually use import or egl rendering instead)
        //     let buffer = {
        //         let mut buffer = Vec::new();
        //         for i in 0..1920 {
        //             for _ in 0..1080 {
        //                 buffer.push(if i % 2 == 0 { 0 } else { 255 });
        //             }
        //         }
        //         buffer
        //     };
        //     bo.write(&buffer).unwrap();

        //     // create a framebuffer from our buffer
        //     let fb_info = framebuffer::create(&gbm, &bo).unwrap();

        //     // display it (and get a crtc, mode and connector before)
        //     use libdrm::control::ResourceInfo;
        //     crtc::set(&gbm, crtc_handle, fb_info.handle(), &[con], (0, 0), Some(mode)).unwrap();
        // }

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

        

        // Err("GBMRenderer not yet implemtended".to_string())

        Ok(Box::new(Self {
            device: gbm,
        }))
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
    ) -> Result<Box<image::Image>, ::egl::EGLError>
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

