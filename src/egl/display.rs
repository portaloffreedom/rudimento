use libc;
use egl;
use egl::EGLError;
use egl::ffi::types::{EGLint, EGLenum};
use egl::device::EGLDevice;
use backend::drm::DRMDevice;

pub struct EGLDisplay {
    display_raw: egl::ffi::types::EGLDisplay,
}

impl EGLDisplay {
    pub fn new(egl_device: &EGLDevice) -> Result<Self, EGLError> {
        let display_raw = unsafe{ egl::ffi::GetDisplay(egl_device.raw() as *const libc::c_void) };
        
        if display_raw == egl::ffi::NO_DISPLAY {
            Err(EGLError::from_str("EGLDisplay::new() Failed to create display"))
        } else {
            Ok(Self{
                display_raw,
            })
        }
    }

    pub fn from_platform(platform: EGLenum, drm_device: &DRMDevice, egl_device: &EGLDevice) -> Result<Self, EGLError> {
        let device_platform_attribs: Vec<EGLint> = vec![
			egl::ffi::DRM_MASTER_FD_EXT as EGLint, drm_device.rawfd(),
			egl::ffi::NONE as EGLint
        ];

        egl::ffi::load_GetPlatformDisplayEXT()
            .map_err(|_| EGLError::from_str("cannot load eglGetPlatformDisplayEXT"))?;

        let mut display_raw = unsafe {
            egl::ffi::GetPlatformDisplayEXT(platform, egl_device.raw(), device_platform_attribs.as_ptr())
        };

        if display_raw.is_null() || display_raw == egl::ffi::NO_DISPLAY {
            Err(EGLError::from_str("EGLDisplay::fromPlatform() Failed to create display"))
        } else {
            Ok(Self{
                display_raw,
            })
        }
    }

    pub fn initialize(&mut self) -> Result<(EGLint, EGLint), EGLError> {
        let mut major: EGLint = 0;
        let mut minor: EGLint = 0;

        let initialization_success = unsafe {egl::ffi::Initialize(self.display_raw, &mut major, &mut minor) == egl::ffi::TRUE };
        if !initialization_success {
            Err(EGLError::from_str("Failed to initialize display"))
        } else {
            Ok((major, minor))
        }
    }

    pub fn choose_config(&mut self, drm_device: &DRMDevice) -> Result<(), EGLError> {
        use egl::ffi::types::EGLConfig;

        let mut count: EGLint = 0;
        let r = unsafe{ egl::ffi::GetConfigs(self.display_raw, 0 as *mut EGLConfig, 0, &mut count)};
        if r != egl::ffi::TRUE {
            return Err(EGLError::from_str("eglGetConfigs error"));
        }

        if count < 1 {
            return Err(EGLError::from_str("No EGL configs to choose from"));
        }

        let mut configs: Vec<EGLConfig> = Vec::with_capacity(count as usize);
        configs.resize(count as usize, 0 as EGLConfig);
        
        let mut matched: EGLint = 0;
        let device_platform_attribs: Vec<EGLint> = vec![
			egl::ffi::DRM_MASTER_FD_EXT as EGLint, drm_device.rawfd(),
			egl::ffi::NONE as EGLint
        ];
        let r = unsafe{ egl::ffi::ChooseConfig(
            self.display_raw, 
            device_platform_attribs.as_ptr(), 
            configs.as_mut_ptr(), 
            count, 
            &mut matched)
        };

        if r != egl::ffi::TRUE {
            return Err(EGLError::from_str("eglChooseConfig error"));
        }

        if matched < 1 {
            return Err(EGLError::from_str("No EGL configs with appropriate attributes"));
        }

        // if (!visual_id || n_ids == 0)
        //     config_index = 0;

        // for (i = 0; config_index == -1 && i < n_ids; i++)
        //     config_index = match_config_to_visual(gr->egl_display,
        //                         visual_id[i],
        //                         configs,
        //                         matched);

        // if (config_index != -1)
        //     *config_out = configs[config_index];

        // out:
        // free(configs);
        // if (config_index == -1)
        //     return -1;

        // if (i > 1)
        //     weston_log("Unable to use first choice EGL config with id"
        //         " 0x%x, succeeded with alternate id 0x%x.\n",
        //         visual_id[0], visual_id[i - 1]);
        // return 0;

        Ok(())
    }
}

impl Drop for EGLDisplay {
    fn drop(&mut self) {
        let r = unsafe {egl::ffi::Terminate(self.display_raw)} == egl::ffi::TRUE;
        if !r {
            println!("FAILED TO TERMINATE DISPLAY \"{:?}\"", self.display_raw);
        }
    }
}