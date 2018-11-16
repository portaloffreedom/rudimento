pub struct GLRenderer {

}

impl GLRenderer {
    pub fn display_create(
        &self,
        // struct weston_compositor *ec,
        // EGLenum platform,
        // void *native_window,
        // const EGLint *platform_attribs,
    	// const EGLint *config_attribs,
        // const EGLint *visual_id,
        // int n_ids
        )
    {
    //     struct gl_renderer *gr;
    // 	EGLint major, minor;
    // 	int supports = 0;

    // 	if (platform) {
    // 		supports = gl_renderer_supports(
    // 			ec, platform_to_extension(platform));
    // 		if (supports < 0)
    // 			return -1;
    // 	}

    // 	gr = zalloc(sizeof *gr);
    // 	if (gr == NULL)
    // 		return -1;

    // 	gr->base.read_pixels = gl_renderer_read_pixels;
    // 	gr->base.repaint_output = gl_renderer_repaint_output;
    // 	gr->base.flush_damage = gl_renderer_flush_damage;
    // 	gr->base.attach = gl_renderer_attach;
    // 	gr->base.surface_set_color = gl_renderer_surface_set_color;
    // 	gr->base.destroy = gl_renderer_destroy;
    // 	gr->base.surface_get_content_size =
    // 		gl_renderer_surface_get_content_size;
    // 	gr->base.surface_copy_content = gl_renderer_surface_copy_content;
    // 	gr->egl_display = NULL;

    // 	/* extension_suffix is supported */
    // 	if (supports) {
    // 		if (!get_platform_display) {
    // 			get_platform_display = (void *) eglGetProcAddress(
    // 					"eglGetPlatformDisplayEXT");
    // 		}

    // 		/* also wrap this in the supports check because
    // 		 * eglGetProcAddress can return non-NULL and still not
    // 		 * support the feature at runtime, so ensure the
    // 		 * appropriate extension checks have been done. */
    // 		if (get_platform_display && platform) {
    // 			gr->egl_display = get_platform_display(platform,
    // 							       native_window,
    // 							       platform_attribs);
    //         }
    //     }

    //     if (!gr->egl_display) {
    //         weston_log("warning: either no EGL_EXT_platform_base "
    //                 "support or specific platform support; "
    //                 "falling back to eglGetDisplay.\n");
    //         gr->egl_display = eglGetDisplay(native_window);
    //     }

    //     if (gr->egl_display == EGL_NO_DISPLAY) {
    //         weston_log("failed to create display\n");
    //         goto fail;
    //     }

    //     if (!eglInitialize(gr->egl_display, &major, &minor)) {
    //         weston_log("failed to initialize display\n");
    //         goto fail_with_error;
    //     }

    //     log_egl_info(gr->egl_display);

    //     if (egl_choose_config(gr, config_attribs, visual_id,
    //                     n_ids, &gr->egl_config) < 0) {
    //         weston_log("failed to choose EGL config\n");
    //         goto fail_terminate;
    //     }

    //     ec->renderer = &gr->base;
    //     ec->capabilities |= WESTON_CAP_ROTATION_ANY;
    //     ec->capabilities |= WESTON_CAP_CAPTURE_YFLIP;
    //     ec->capabilities |= WESTON_CAP_VIEW_CLIP_MASK;

    //     if (gl_renderer_setup_egl_extensions(ec) < 0)
    //         goto fail_with_error;

    //     wl_list_init(&gr->dmabuf_images);
    //     if (gr->has_dmabuf_import) {
    //         gr->base.import_dmabuf = gl_renderer_import_dmabuf;
    //         gr->base.query_dmabuf_formats =
    //             gl_renderer_query_dmabuf_formats;
    //         gr->base.query_dmabuf_modifiers =
    //             gl_renderer_query_dmabuf_modifiers;
    //     }

    //     if (gr->has_surfaceless_context) {
    //         weston_log("EGL_KHR_surfaceless_context available\n");
    //         gr->dummy_surface = EGL_NO_SURFACE;
    //     } else {
    //         weston_log("EGL_KHR_surfaceless_context unavailable. "
    //                 "Trying PbufferSurface\n");

    //         if (gl_renderer_create_pbuffer_surface(gr) < 0)
    //             goto fail_with_error;
    //     }

    //     wl_display_add_shm_format(ec->wl_display, WL_SHM_FORMAT_RGB565);
    //     wl_display_add_shm_format(ec->wl_display, WL_SHM_FORMAT_YUV420);
    //     wl_display_add_shm_format(ec->wl_display, WL_SHM_FORMAT_NV12);
    //     wl_display_add_shm_format(ec->wl_display, WL_SHM_FORMAT_YUYV);

    //     wl_signal_init(&gr->destroy_signal);

    //     if (gl_renderer_setup(ec, gr->dummy_surface) < 0) {
    //         if (gr->dummy_surface != EGL_NO_SURFACE)
    //             weston_platform_destroy_egl_surface(gr->egl_display,
    //                                 gr->dummy_surface);
    //         goto fail_with_error;
    //     }

    //     return 0;

    // fail_with_error:
    //     gl_renderer_print_egl_error_state();
    // fail_terminate:
    //     eglTerminate(gr->egl_display);
    // fail:
    //     free(gr);
    //     return -1;
    }
}