use libc;
use egl;
use egl::EGLError;
use egl::ffi::types::{EGLint, EGLenum, EGLConfig};
use egl::device::EGLDevice;
use backend::drm::DRMDevice;
use egl::extensions::Extensions;

#[derive(Debug)]
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

    pub fn raw_ref(&self) -> &egl::ffi::types::EGLDisplay {
        &self.display_raw
    }

    pub fn from_platform(platform: EGLenum, drm_device: &DRMDevice, egl_device: &EGLDevice) -> Result<Self, EGLError> {
        let device_platform_attribs: Vec<EGLint> = vec![
			egl::ffi::DRM_MASTER_FD_EXT as EGLint, drm_device.rawfd(),
			egl::ffi::NONE as EGLint
        ];

        egl::ffi::load_GetPlatformDisplayEXT()
            .map_err(|_| EGLError::from_str("cannot load eglGetPlatformDisplayEXT"))?;

        let display_raw = unsafe {
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

    pub fn choose_config(&mut self, visual_ids: Option<Vec<EGLint>>) -> Result<EGLConfig, EGLError> {

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
        let gl_renderer_opaque_stream_attribs: Vec<EGLint> = {
            use egl::ffi::*;
            vec![
            SURFACE_TYPE as EGLint, STREAM_BIT_KHR as EGLint,
            RED_SIZE as EGLint, 1,
            GREEN_SIZE as EGLint, 1,
            BLUE_SIZE as EGLint, 1,
            ALPHA_SIZE as EGLint, 0,
            RENDERABLE_TYPE as EGLint, OPENGL_ES2_BIT as EGLint,
            NONE as EGLint
        ]};
        let r = unsafe{ egl::ffi::ChooseConfig(
            self.display_raw, 
            gl_renderer_opaque_stream_attribs.as_ptr(), 
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

        if let Some(visual_ids) = visual_ids {
            let mut visual_id_counter = 0;
            let config = visual_ids.iter()
                .filter_map(|visual_id| {
                    visual_id_counter += 1;
                    println!("AAAAA visual_id_counter={}", visual_id_counter);
                    self.match_config_to_visual(*visual_id, &configs)
                })
                .nth(0)
                .cloned()
                .ok_or_else(|| EGLError::from_str("could not find egl configuration"));

            if config.is_ok() && visual_id_counter > 1 {
                println!("Unable to use first choice EGL config with id 0x{:x}, succeeded with alternate id 0x{:x}.", visual_ids[0], visual_ids[visual_id_counter]);
            }

            config
        } else {
            Ok(configs[0])
        }
    }

    pub fn extensions(&self) -> Result<Extensions, EGLError>
    {
        Extensions::query(Some(self))
    }

    fn match_config_to_visual<'a>(&self, visual_id: EGLint, configs: &'a Vec<EGLConfig>) -> Option<&'a EGLConfig> {
        configs.iter()
            .find(|config| {
                let mut id: EGLint = -1;
                let r = unsafe {egl::ffi::GetConfigAttrib(self.display_raw, **config, egl::ffi::NATIVE_VISUAL_ID as EGLint, &mut id)};
                if r == egl::ffi::TRUE {
                    id == visual_id
                } else {
                    false
                }
            })
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