use egl;
use egl::display::EGLDisplay;
use egl::EGLError;
use egl::ffi::types::*;
use egl::device::{EGLDevice, get_egl_devices};
use backend::drm::DRMDevice;
use renderer::Renderer;
use std::path::Path;
use libc;
use renderer;
use wayland::list::List as WLlist;
use wayland::display::Display as WaylandDisplay;
use wayland::signal::Signal as WaylandSignal;

#[derive(Debug)]
pub struct EGLRenderer {
    egl_device: EGLDevice,
    display: EGLDisplay,
    egl_config: EGLConfig,
    use_eglstream: bool,
//    opaque_attribs: egl::EGLint,
//    alpha_attribs: egl::EGLint,
//    opaque_stream_attribs: egl::EGLint,

    // FLAGS SUPPORTED OPERATIONS
    has_context_priority: bool,
    has_bind_display: bool,
    has_egl_buffer_age: bool,
    has_egl_ext_swap_buffers_with_damage: bool,
    has_egl_khr_swap_buffers_with_damage: bool,
    has_configless_context: bool,
    has_surfaceless_context: bool,
    has_dmabuf_import: bool,
    has_dmabuf_import_modifiers: bool,
    has_native_fence_sync: bool,
    has_egl_output_base: bool,
    has_egl_output_drm: bool,
    has_egl_output_drm_flip_event: bool,
    has_egl_stream: bool,
    has_egl_stream_producer_eglsurface: bool,
    has_egl_stream_consumer_egloutput: bool,
    has_egl_stream_attrib: bool,
    has_egl_stream_acquire_mode: bool,
    has_egl_stream_consumer_gltexture: bool,
    has_egl_wayland_eglstream: bool,
    has_egl_platform_base: bool,
}

impl EGLRenderer {
    fn new(egl_device: EGLDevice, drm_device: &DRMDevice) -> Result<Box<EGLRenderer>, EGLError> {
        let (display, egl_config) = Self::display_create(drm_device, &egl_device)?;

        // ec->capabilities |= WESTON_CAP_ROTATION_ANY;
        // ec->capabilities |= WESTON_CAP_CAPTURE_YFLIP;
        // ec->capabilities |= WESTON_CAP_VIEW_CLIP_MASK;
        
        let mut renderer = Box::new(EGLRenderer {
            egl_device,
            display,
            egl_config,
            use_eglstream: true,
            // support stuff, will be overwritten by setup_egl_extensions
            has_context_priority: false,
            has_bind_display: false,
            has_egl_buffer_age: false,
            has_egl_ext_swap_buffers_with_damage: false,
            has_egl_khr_swap_buffers_with_damage: false,
            has_configless_context: false,
            has_surfaceless_context: false,
            has_dmabuf_import: false,
            has_dmabuf_import_modifiers: false,
            has_native_fence_sync: false,
            has_egl_output_base: false,
            has_egl_output_drm: false,
            has_egl_output_drm_flip_event: false,
            has_egl_stream: false,
            has_egl_stream_producer_eglsurface: false,
            has_egl_stream_consumer_egloutput: false,
            has_egl_stream_attrib: false,
            has_egl_stream_acquire_mode: false,
            has_egl_stream_consumer_gltexture: false,
            has_egl_wayland_eglstream: false,
            has_egl_platform_base: false,
        });

        renderer.setup_egl_extensions()?;

        println!("EGLRenderer: {:?}", renderer);

        // check support for extensions:
        if !renderer.has_egl_output_base
        || !renderer.has_egl_output_drm
        || !renderer.has_egl_stream
        || !renderer.has_egl_stream_producer_eglsurface
        || !renderer.has_egl_stream_consumer_egloutput
        || !renderer.has_egl_stream_attrib
        || !renderer.has_egl_stream_acquire_mode
        {
            return Err(EGLError::from_string(format!(
                "following required extensions not supported:
                has_egl_output_base: {}
                has_egl_output_drm: {}
                has_egl_stream: {}
                has_egl_stream_producer_eglsurface: {}
                has_egl_stream_consumer_egloutput: {}
                has_egl_stream_attrib: {}
                has_egl_stream_acquire_mode: {}",
                renderer.has_egl_output_base,
                renderer.has_egl_output_drm,
                renderer.has_egl_stream,
                renderer.has_egl_stream_producer_eglsurface,
                renderer.has_egl_stream_consumer_egloutput,
                renderer.has_egl_stream_attrib,
                renderer.has_egl_stream_acquire_mode,
            )));
        }

        if !renderer.has_egl_stream_consumer_gltexture
        || !renderer.has_egl_wayland_eglstream
        {
            println!(
                "WARNING! following required extensions for EGL client frame presentation through EGLDevice not supported:
                has_egl_stream_consumer_gltexture: {}
                has_egl_wayland_eglstream: {}",
                renderer.has_egl_stream_consumer_gltexture,
                renderer.has_egl_wayland_eglstream,
            )
        }
        
        if !renderer.has_egl_output_drm_flip_event
        {
            println!(
                "WARNING! EGL page flip event notification not supported:
                has_egl_output_drm_flip_event: {}",
                renderer.has_egl_output_drm_flip_event,
            )
        }

        // let dmabuf_images = WLlist::new();

        // if (gr->has_dmabuf_import) {
        //     gr->base.import_dmabuf = gl_renderer_import_dmabuf;
        //     gr->base.query_dmabuf_formats =
        //         gl_renderer_query_dmabuf_formats;
        //     gr->base.query_dmabuf_modifiers =
        //         gl_renderer_query_dmabuf_modifiers;
        // }

        if renderer.has_surfaceless_context {
            println!("EGL_KHR_surfaceless_context available\n");
            // gr->dummy_surface = EGL_NO_SURFACE;
        } else {
            return Err(EGLError::from_str("renderer.has_surfaceless_context is mandatory for the moment"))
        //     weston_log("EGL_KHR_surfaceless_context unavailable. "
        //         "Trying PbufferSurface\n");

        //     if (gl_renderer_create_pbuffer_surface(gr) < 0)
        //         goto fail_with_error;
        }

        // use wayland_server::protocol::wl_shm::Format;

        // let mut wl_display = unsafe {
        //     use wayland_server::sys::server::wl_display;
        //     let raw: wl_display;
        //     WaylandDisplay::from_raw(raw)
        // };
        // wl_display.add_shm_format(Format::Rgb565);
        // wl_display.add_shm_format(Format::Yuv420);
        // wl_display.add_shm_format(Format::Nv12);
        // wl_display.add_shm_format(Format::Yuyv);

	    // wl_signal_init(&gr->destroy_signal);
        // let destroy_signal = WaylandSignal::new();

        // if (gl_renderer_setup(ec, gr->dummy_surface) < 0) {
        //     if (gr->dummy_surface != EGL_NO_SURFACE)
        //         weston_platform_destroy_egl_surface(gr->egl_display,
        //                             gr->dummy_surface);
        //     goto fail_with_error;
        // }

        Ok(renderer)
    }

    pub fn from_drm_device_file(drm_device: &DRMDevice) -> Result<Box<EGLRenderer>, EGLError> {
        load_EGL()?;
        println!("Creating EGL Renderer at {:?}", drm_device.dev_path());
        
        let egl_device = find_egldevice(drm_device.dev_path())?;
        Self::new(egl_device, drm_device)
    }

    pub fn first_drm_device_available(drm_device: &DRMDevice) -> Result<Box<EGLRenderer>, EGLError> {
        let devices = get_egl_devices()?;

        if devices.len() > 0 {
            let egl_device = devices.into_iter().nth(0).unwrap();
            Self::new(egl_device, drm_device)
        } else {
            Err(EGLError::from_str("No EGLDevice found"))
        }
    }

    pub fn display_create(drm_device: &DRMDevice, egl_device: &EGLDevice) -> Result<(EGLDisplay, EGLConfig), EGLError> {
        // let platform = egl::ffi::PLATFORM_GBM_KHR; // GBM
        let platform = egl::ffi::PLATFORM_DEVICE_EXT; // EGL

        // if (platform) {
        //     supports = gl_renderer_supports(
        //         ec, platform_to_extension(platform));
        //     if (supports < 0)
        //         return -1;
        // }

        let mut display = EGLDisplay::from_platform(platform, drm_device, egl_device)
            .or_else(|_| EGLDisplay::new(egl_device))?;

        let (egl_major, egl_minor) = display.initialize()?;
        println!("Initialized Display with EGL {}.{}", egl_major, egl_minor);

        // egl_choose_config(gr, config_attribs, visual_id, n_ids, &gr->egl_config)
        //weston_log("failed to choose EGL config\n");
        let egl_config = display.choose_config(None)?;

        Ok((display, egl_config))
    }


    fn setup_egl_extensions(&mut self) -> Result<(), EGLError> {
        egl::ffi::load_CreateImageKHR()?;
        egl::ffi::load_CreateImageKHR()?;
        egl::ffi::load_DestroyImageKHR()?;

        egl::ffi::load_BindWaylandDisplayWL()?;
        egl::ffi::load_UnbindWaylandDisplayWL()?;
        egl::ffi::load_QueryWaylandBufferWL()?;
        egl::ffi::load_GetOutputLayersEXT()?;
        egl::ffi::load_QueryOutputLayerAttribEXT()?;
        egl::ffi::load_CreateStreamKHR()?;
        egl::ffi::load_DestroyStreamKHR()?;
        egl::ffi::load_QueryStreamKHR()?;
        egl::ffi::load_CreateStreamProducerSurfaceKHR()?;
        egl::ffi::load_StreamConsumerOutputEXT()?;
        // #ifdef EGL_NV_stream_attrib
        egl::ffi::load_CreateStreamAttribNV()?;
        egl::ffi::load_StreamConsumerAcquireAttribNV()?;
        // #endif
        egl::ffi::load_StreamConsumerGLTextureExternalKHR()?;

        let extensions = self.display.extensions()?;

        self.has_context_priority = extensions.contains("EGL_IMG_context_priority");
        self.has_bind_display = extensions.contains("EGL_WL_bind_wayland_display");

        if self.has_bind_display {
    //         ret = gr->bind_display(gr->egl_display, ec->wl_display);
    //         if (!ret)
    //             self.has_bind_display = 0;
        }

        self.has_egl_buffer_age = extensions.contains("EGL_EXT_buffer_age");
        if !self.has_egl_buffer_age {
            println!("warning: EGL_EXT_buffer_age not supported. Performance could be affected");
        }

        self.has_egl_ext_swap_buffers_with_damage = extensions.contains("EGL_EXT_swap_buffers_with_damage");
        self.has_egl_khr_swap_buffers_with_damage = extensions.contains("EGL_KHR_swap_buffers_with_damage");

        if self.has_egl_ext_swap_buffers_with_damage {
            egl::ffi::load_SwapBuffersWithDamageEXT()?;
        } else if self.has_egl_khr_swap_buffers_with_damage {
            egl::ffi::load_SwapBuffersWithDamageKHR()?;
        } else {
            println!("warning: neither EGL_EXT_swap_buffers_with_damage or EGL_KHR_swap_buffers_with_damage is supported. Performance could be affected.");
        }
        //TODO memorize which one is loaded
   
        self.has_configless_context = extensions.contains("EGL_KHR_no_config_context") 
            || extensions.contains("EGL_MESA_configless_context");

        self.has_surfaceless_context = extensions.contains("EGL_KHR_surfaceless_context");

        self.has_dmabuf_import = extensions.contains("EGL_EXT_image_dma_buf_import");

        self.has_dmabuf_import_modifiers = extensions.contains("EGL_EXT_image_dma_buf_import_modifiers");
        if self.has_dmabuf_import_modifiers {
            egl::ffi::load_QueryDmaBufFormatsEXT()?;
            egl::ffi::load_QueryDmaBufModifiersEXT()?;
        }

        self.has_native_fence_sync = extensions.contains("EGL_KHR_fence_sync") 
            && extensions.contains("EGL_ANDROID_native_fence_sync");
        
        if self.has_native_fence_sync {
                egl::ffi::load_CreateSyncKHR()?;
                egl::ffi::load_DestroySyncKHR()?;
                egl::ffi::load_DupNativeFenceFDANDROID()?;
        } else {
            println!("warning: Disabling render GPU timeline due to missing EGL_KHR_fence_sync or EGL_ANDROID_native_fence_sync extension");
        }

        self.has_egl_output_base = extensions.contains("EGL_EXT_output_base");
        self.has_egl_output_drm = extensions.contains("EGL_EXT_output_drm");
        self.has_egl_output_drm_flip_event = extensions.contains("EGL_NV_output_drm_flip_event");

        self.has_egl_stream = extensions.contains("EGL_KHR_stream");
        self.has_egl_stream_producer_eglsurface = extensions.contains("EGL_KHR_stream_producer_eglsurface");
        self.has_egl_stream_consumer_egloutput = extensions.contains("EGL_EXT_stream_consumer_egloutput");
        self.has_egl_stream_attrib = extensions.contains("EGL_NV_stream_attrib");
        self.has_egl_stream_acquire_mode = extensions.contains("EGL_EXT_stream_acquire_mode");
        self.has_egl_stream_consumer_gltexture = extensions.contains("EGL_KHR_stream_consumer_gltexture");
        self.has_egl_wayland_eglstream = extensions.contains("EGL_WL_wayland_eglstream");

        self.setup_egl_client_extensions()?;

        Ok(())
    }

    fn setup_egl_client_extensions(&mut self) -> Result<bool, EGLError> {
        use egl::extensions::Extensions;
        let extensions = Extensions::query(None)?;

        self.has_egl_platform_base = extensions.contains("EGL_EXT_platform_base");

        if self.has_egl_platform_base {
            egl::ffi::load_CreatePlatformWindowSurfaceEXT()?;
        } else {
            println!("warning: EGL_EXT_platform_base not supported.");
        }

        Ok(self.has_egl_platform_base)
    }

    pub fn display(&self) /*-> egl::EGLDisplay*/ {

    }

    pub fn output_window_create() {

    }

    pub fn output_destroy() {

    }

    pub fn output_surface() /*-> egl::EGLSurface*/ {

    }

    pub fn output_set_border() {

    }

    pub fn print_egl_error_state() {

    }

    pub fn get_devices() {

    }

    pub fn get_drm_device_file() {

    }

    pub fn output_stream_flip() {

    }
}

#[allow(non_snake_case)]
fn load_EGL() -> Result<(), EGLError> {
    let library_path = "libEGL.so";
    println!("Loading EGL Library from {:?}", library_path);
    egl::loader::load_EGL(library_path)
}

fn find_egldevice(filename: &Path) -> Result<EGLDevice, EGLError> {
    let devices = get_egl_devices()?;

    if devices.len() == 0 {
        //HACK BECAUSE WE GOT INTEL EGL PROBLEMS
        Ok(devices.into_iter().find(|_| true ).unwrap())
    } else {
        devices.into_iter()
            .find(|device| {
                match device.get_drm_device_file() {
                    Ok(drm_device) => 
                    {
                        let filename = filename.to_string_lossy();
                        filename.eq(&drm_device.to_string_lossy())
                    },
                    Err(e) => {
                        println!("ERROR READING DRM FILE FOR EGLDEVICE: {:?}", e);
                        false
                    }
                }
            })
            .ok_or_else(|| {
                EGLError::from_string(format!("Device {:?} not found", filename))
            })
    }
}

impl EGLRenderer {

    fn image_target_texture_2d(&self) {}

    fn create_image<'display>(
        &'display self,
        context: Option<EGLContext>, 
        target: EGLenum,
        buffer: EGLClientBuffer,
        attrib_list: &Vec<EGLint>
    ) -> Result<Box<egl::image::Image<'display>>, EGLError>
    {
        let display: &'display EGLDisplay = &self.display;

        egl::image::Image::new(display, context, target, buffer, attrib_list)
    }

    fn swap_buffers_with_damage(&self) {
    //         {
    //             .extension = "EGL_EXT_swap_buffers_with_damage",
    //             .entrypoint = "eglSwapBuffersWithDamageEXT",
    //         },
    //         {
    //             .extension = "EGL_KHR_swap_buffers_with_damage",
    //             .entrypoint = "eglSwapBuffersWithDamageKHR",
    //         },
    }

    // extension required EGL_EXT_platform_base
    fn create_platform_window(&self) {
        // egl::ffi::CreatePlatformWindowSurfaceEXT()
    }
	// const has_unpack_subimage: bool;
    
    // const has_bind_display: bool;
    fn bind_display(&self, wl_display: &mut WaylandDisplay) -> Result<(), EGLError>{
        let r = unsafe {egl::ffi::BindWaylandDisplayWL(
            self.display.raw_ref().clone(), 
            wl_display.raw_mut_ref(),
        )};
        if r != egl::ffi::TRUE {
            Err(EGLError::from_str("could not bind egl display to wayland display"))
        } else {
            Ok(())
        }
    }
    fn unbind_display(&self, wl_display: &mut WaylandDisplay) -> Result<(), EGLError> {
        let r = unsafe { egl::ffi::UnbindWaylandDisplayWL(
            self.display.raw_ref().clone(),
            wl_display.raw_mut_ref(),
        )};
        if r != egl::ffi::TRUE {
            Err(EGLError::from_str("could not unbindegl display to wayland display"))
        } else {
            Ok(())
        }
    }
    fn query_buffer(&self) -> Result<(), EGLError> {
        Err(EGLError::from_str("EGLRenderer::query_buffer() not implemented"))
        // let r = egl::ffi::QueryWaylandBufferWL(
        //     self.display.raw_ref().clone(),
        //     buffer,
        //     attribute,
        //     value,
        // );
        // if r != egl::ffi::TRUE {
        //     Err(EGLError::from_str("could not unbindegl display to wayland display"))
        // } else {
        //     Ok(())
        // }
    }

    // const has_context_priority: bool;
	// const has_egl_image_external: bool;
	// const has_egl_buffer_age: bool;
	// const has_configless_context: bool;
	// const has_surfaceless_context: bool;

    //nvidia stuff
    // const has_egl_output_base: bool;
	// const has_egl_output_drm: bool;
	// const has_egl_output_drm_flip_event: bool;
    fn get_output_layers(&self) {
        // egl::ffi::GetOutputLayersEXT()
    }
    fn query_output_layer_attrib(&self) {
        // egl::ffi::QueryOutputLayerAttribEXT()
    }

    // const has_egl_stream: bool;
    fn create_stream(&self) {
        // egl::ffi::CreateStreamKHR()
    }
    fn destroy_stream(&self) {
        // egl::ffi::DestroyStreamKHR()
    }
    fn query_stream(&self) {
        // egl::ffi::QueryStreamKHR()
    }

    // const has_egl_stream_producer_eglsurface: bool;
    fn create_stream_producer_surface(&self) {
        // egl::ffi::CreateStreamProducerSurfaceKHR()
    }

    // const has_egl_stream_consumer_egloutput: bool;
    fn stream_consumer_output(&self) {
        // egl::ffi::StreamConsumerOutputEXT()
    }

    //nvidia egl stream attrib
    // const has_egl_stream_attrib: bool;
    fn create_stream_attrib(&self) {
        // egl::ffi::CreateStreamAttribNV()
    }
    // const has_egl_stream_acquire_mode: bool;
    fn stream_consumer_acquire_attrib(&self) {
        // egl::ffi::StreamConsumerAcquireAttribNV()
    }

    // const has_egl_stream_consumer_gltexture: bool;
    // const has_egl_wayland_eglstream: bool;
    fn stream_consumer_gltexture(&self) {
        // egl::ffi::StreamConsumerGLTextureExternalKHR()
    }

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
    fn query_dmabuf_formats(&self) {
        // egl::ffi::QueryDmaBufFormatsEXT()
    }
    fn query_dmabuf_modifiers(&self) {
        // egl::ffi::QueryDmaBufModifiersEXT()
    }

    // const has_native_fence_sync: bool;
    fn create_sync(&self) {
        // egl::ffi::CreateSyncKHR()
    }
    fn destroy_sync(&self) {
        // egl::ffi::DestroySyncKHR()
    }
    fn dup_native_fence_fd(&self) {
        // egl::ffi::DupNativeFenceFDANDROID()
    }
}
