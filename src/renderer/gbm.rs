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
    gbm: gbm::Device<DRMDevice>,
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

        let gbm = gbm::Device::new(drm_device)
        // unsafe: device has to outlive file descriptor
        // let gbm = unsafe { gbm::Device::new_from_fd(drm_devide.as_raw_fd()) }
            .map_err(|e| format!("Could not create GDB Device: {}", e))?;

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
            gbm,
        }))
    }

    pub fn test(&self, drm_device: &DRMDevice) {
        use std::thread;
        use std::time::Duration;
        use gbm::{Format, BufferObjectFlags};
        use libdrm::control::{crtc, framebuffer};
        use libdrm::control::Device as libdrm_device;
        use libdrm::control::ResourceInfo;

        // Get a set of all modesetting resource handles (excluding planes):
        let res_handles = drm_device.resource_handles().unwrap();

        println!("\nConnector Informations");
        // Print all connector information
        for &con in res_handles.connectors() {
            let info: libdrm::control::connector::Info 
                = drm_device.resource_info(con).unwrap();

            println!("{:?}", info);
        }

        let con = res_handles.connectors().iter().next()
            .expect("No Card Connector found");
        let connector_info: libdrm::control::connector::Info
            = drm_device.resource_info(*con).unwrap();
        let mode = connector_info.modes()[0];
        let (hdisplay, vdisplay) = mode.size();

        println!("\nCRTCs Informations");
        // Print all CRTC information
        for &crtc in res_handles.crtcs() {
            let info: drm::control::crtc::Info 
                = drm_device.resource_info(crtc).unwrap();

            println!("{:?}", info);
        }

        //TODO maybe select a better one than the first
        let crtc_handle = res_handles.crtcs().iter().next()
            .expect("No CRTC handle found");

        let mut bo = self.gbm.create_buffer_object::<()>(
            hdisplay.into(), vdisplay.into(),
            Format::XRGB8888,
            BufferObjectFlags::SCANOUT | BufferObjectFlags::WRITE,
            )
            .expect("Could not create Buffer Object");

        // write something to it (usually use import or egl rendering instead)
        let buffer = {
            let mut buffer = Vec::<u8>::new();
            for i in 0..vdisplay {
                for j in 0..hdisplay {
                    // XRGB8888
                    buffer.push(if i % 2 == 0 { 0   } else { 0   }); // Blue
                    buffer.push(if i % 2 == 0 { 255 } else { 0   }); // Green
                    buffer.push(if i % 2 == 0 { 0   } else { 0   }); // Red
                    buffer.push(if j % 2 == 0 { 0   } else { 255 }); // Nothing
                }
            }
            buffer
        };
        bo.write(&buffer)
            .expect("Buffer Object write failed")
            .expect("Buffer Object write failed 2");

        // create a framebuffer from our buffer
        let fb_info = framebuffer::create(&self.gbm, &bo)
            .expect("framebuffer create failed");

        // display it (and get a crtc, mode and connector before)
        crtc::set(&self.gbm, *crtc_handle, fb_info.handle(), &[*con], (0, 0), Some(mode))
            .expect("display framebuffer to the crtc failed");

        thread::sleep(Duration::from_millis(100));


        let mut bo2 = self.gbm.create_buffer_object::<()>(
            hdisplay.into(), vdisplay.into(),
            Format::XRGB8888,
            BufferObjectFlags::SCANOUT | BufferObjectFlags::WRITE,
            )
            .expect("Could not create Buffer Object");

        // write something to it (usually use import or egl rendering instead)
        let buffer = {
            let mut buffer = Vec::<u8>::new();
            for i in 0..vdisplay {
                for j in 0..hdisplay {
                    // XRGB8888
                    buffer.push(255);//if i % 2 == 0 { 0   } else { 0   }); // Blue
                    buffer.push(255);//if i % 2 == 0 { 255 } else { 0   }); // Green
                    buffer.push(255);//if i % 2 == 0 { 0   } else { 255 }); // Red
                    buffer.push(if j % 2 == 0 { 0   } else { 255 }); // Nothing
                }
            }
            buffer
        };
        bo2.write(&buffer)
            .expect("Buffer Object write failed")
            .expect("Buffer Object write failed 2");


        let fb_info_2 = framebuffer::create(&self.gbm, &bo2)
            .expect("framebuffer create failed");

        crtc::set(&self.gbm, *crtc_handle, fb_info_2.handle(), &[*con], (0, 0), Some(mode))
            .expect("display framebuffer to the crtc failed");

        thread::sleep_ms(100);
        
        // https://github.com/dvdhrm/docs/commit/87d3698967a148174cdaa97a068b23ca2387c798
        // https://docs.rs/drm/0.3.4/drm/control/crtc/fn.page_flip.html
        crtc::page_flip(&self.gbm, *crtc_handle, fb_info.handle(), &[crtc::PageFlipFlags::PageFlipEvent])
            .expect("Page Flip schedule failed");

        thread::sleep_ms(100);

        { // Test VSYNC

            let vdisplay = vdisplay as usize;
            let hdisplay = hdisplay as usize;

            let threads_n = 8_usize;
            let thread_handles = (0..threads_n).into_iter().map(|thread_id| {
                thread::Builder::new().name(format!("vsync test builder {}", thread_id))
                    .spawn(move || {
                        let mut mother_buffer = Vec::<u8>::new();
                        for i in 0..vdisplay {
                            for j in 0..hdisplay {
                                // XRGB8888
                                mother_buffer.push(0); // Blue
                                mother_buffer.push(255); // Green
                                mother_buffer.push(0); // Red
                                mother_buffer.push(255); // Nothing
                            }
                        }
                        let mut buffers = Vec::new();
                        let partition = (128/threads_n) as usize;
                        for c in ((partition*thread_id)..(partition*(thread_id+1))) {
                            let mut i = 0;
                            let mut j = 0;
                            let mut quartet_counter = 0;
                            for byte in mother_buffer.iter_mut() {
                                quartet_counter = quartet_counter + 1;

                                j = j+quartet_counter/4;
                                quartet_counter = quartet_counter%4;

                                i = i + j/hdisplay;
                                j = j%hdisplay;

                                let is_black = (c*3 + j) < (hdisplay/3) || (c*3 + j) > (hdisplay*2/3);
                                let color: u8 = if is_black { 0 } else { 255 };
                                *byte = color;
                            }
                            
                            buffers.push(mother_buffer.clone());
                        }
                        buffers
                }).expect("Could not start thread ")
            }).collect::<Vec<_>>();

            let buffers = thread_handles.into_iter().flat_map(|handle| {
                handle.join().expect("Error running thread")
            }).collect::<Vec<_>>();

            let mut bos = vec![(bo, fb_info), (bo2,fb_info_2)];
            let mut counter = 0;
            for _ in 0..20 {
                for buffer in buffers.iter() {
                    counter += 1;
                    let (bo, fb_info) = &mut bos[counter%2];
                    bo.write(&buffer)
                        .expect("Buffer Object write failed")
                        .expect("Buffer Object write failed 2");
                    crtc::set(&self.gbm, *crtc_handle, fb_info.handle(), &[*con], (0, 0), Some(mode))
                        .expect("display framebuffer to the crtc failed");
                }
            }
        }
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

