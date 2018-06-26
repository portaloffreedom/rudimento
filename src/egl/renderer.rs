use egl;
use egl::display::EGLDisplay;
use egl::EGLError;
use egl::device::{EGLDevice, get_egl_devices};
use backend::drm::DRMDevice;
use renderer::Renderer;
use std::path::Path;
use libc;

pub struct EGLRenderer {
    egl_device: EGLDevice,
    display: EGLDisplay,
//    opaque_attribs: egl::EGLint,
//    alpha_attribs: egl::EGLint,
//    opaque_stream_attribs: egl::EGLint,
}

impl Renderer for EGLRenderer {
}

impl EGLRenderer {
    pub fn from_drm_device_file(drm_device: &DRMDevice) -> Result<Box<EGLRenderer>, EGLError> {
        load_EGL()?;
        println!("Creating EGL Renderer at {:?}", drm_device.dev_path());
        
        let egl_device = find_egldevice(drm_device.dev_path())?;

        let display = Self::display_create(drm_device, &egl_device)?;

        Ok(Box::new(EGLRenderer {
            egl_device,
            display,
        }))
    }

    pub fn first_drm_device_available(drm_device: &DRMDevice) -> Result<Box<EGLRenderer>, EGLError> {
        let devices = get_egl_devices()?;

        if devices.len() > 0 {
            let egl_device = devices.into_iter().nth(0).unwrap();
            let display = Self::display_create(drm_device, &egl_device)?;
            Ok(Box::new(EGLRenderer {
                egl_device,
                display,
            }))
        } else {
            Err(EGLError::from_str("No EGLDevice found"))
        }
    }

    pub fn display_create(drm_device: &DRMDevice, egl_device: &EGLDevice) -> Result<EGLDisplay, EGLError> {
        // let platform = egl::ffi::PLATFORM_GBM_KHR; // GBM
        let platform = egl::ffi::PLATFORM_DEVICE_EXT; // EGL

        let mut display = EGLDisplay::from_platform(platform, drm_device, egl_device)
            .or_else(|_| EGLDisplay::new(egl_device))?;

        let (egl_major, egl_minor) = display.initialize()?;
        println!("Initialized Display with EGL {}.{}", egl_major, egl_minor);

        // egl_choose_config(gr, config_attribs, visual_id, n_ids, &gr->egl_config)
        //weston_log("failed to choose EGL config\n");
        display.choose_config(drm_device)?;

        // ec->capabilities |= WESTON_CAP_ROTATION_ANY;
        // ec->capabilities |= WESTON_CAP_CAPTURE_YFLIP;
        // ec->capabilities |= WESTON_CAP_VIEW_CLIP_MASK;

        // gl_renderer_setup_egl_extensions()

        // check support for extensions:
        //TODO FAIL
        // gr->has_egl_output_base
        // gr->has_egl_output_drm
        // gr->has_egl_stream
        // gr->has_egl_stream_producer_eglsurface
        // gr->has_egl_stream_consumer_egloutput
        // gr->has_egl_stream_attrib
        // gr->has_egl_stream_acquire_mode

        //TODO WARNING: 
        // following required extensions for
		// EGL client frame presentation through
		// EGLDevice not supported:
        // gr->has_egl_stream_consumer_gltexture
        // gr->has_egl_wayland_eglstream

        //TODO WARNING: 
        // EGL page flip event notification
        // not supported
        // gr->has_egl_output_drm_flip_event

        // wl_list_init()

        // if (gr->has_dmabuf_import) {
        //     gr->base.import_dmabuf = gl_renderer_import_dmabuf;
        //     gr->base.query_dmabuf_formats =
        //         gl_renderer_query_dmabuf_formats;
        //     gr->base.query_dmabuf_modifiers =
        //         gl_renderer_query_dmabuf_modifiers;
        // }

        // if (gr->has_surfaceless_context) {
        //     weston_log("EGL_KHR_surfaceless_context available\n");
        //     gr->dummy_surface = EGL_NO_SURFACE;
        // } else {
        //     weston_log("EGL_KHR_surfaceless_context unavailable. "
        //         "Trying PbufferSurface\n");

        //     if (gl_renderer_create_pbuffer_surface(gr) < 0)
        //         goto fail_with_error;
        // }

        // wl_display_add_shm_format(ec->wl_display, WL_SHM_FORMAT_RGB565);
        // wl_display_add_shm_format(ec->wl_display, WL_SHM_FORMAT_YUV420);
        // wl_display_add_shm_format(ec->wl_display, WL_SHM_FORMAT_NV12);
        // wl_display_add_shm_format(ec->wl_display, WL_SHM_FORMAT_YUYV);

	    // wl_signal_init(&gr->destroy_signal);

        // if (gl_renderer_setup(ec, gr->dummy_surface) < 0) {
        //     if (gr->dummy_surface != EGL_NO_SURFACE)
        //         weston_platform_destroy_egl_surface(gr->egl_display,
        //                             gr->dummy_surface);
        //     goto fail_with_error;
        // }

        Ok(display)

        // Err(EGLError::from_str("YEAH WE ARRIVED HERE!"))
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

fn load_EGL() -> Result<(), EGLError> {
    let library_path = "libEGL.so";
    println!("Loading EGL Library from {:?}", library_path);
    egl::loader::Load_EGL(library_path)
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
