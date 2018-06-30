use super::*;
use super::super::EGLError;
use wayland_server::sys::server::{
    wl_display,
    wl_resource
};

macro_rules! define_enum {
    ($name:ident, $value:expr) => {
        #[allow(dead_code, non_upper_case_globals)] pub const $name: types::EGLenum = $value;
    }
}

define_enum!(STREAM_BIT_KHR,      0x0800);
define_enum!(PLATFORM_DEVICE_EXT, 0x313F);
define_enum!(PLATFORM_GBM_KHR,    0x31D7);
define_enum!(DRM_DEVICE_FILE_EXT, 0x3233);
define_enum!(DRM_MASTER_FD_EXT,   0x333C);


macro_rules! define_ext_funs {
    { $( $name:ident ( $($param:ident: $param_type:ty),* ) -> $return_type:ty)* } => {

        mashup! {
            $(
                m["load" $name] = load_ $name;
            )*
        }

        $(
            #[allow(non_snake_case, dead_code)] 
            #[inline]
            pub unsafe fn $name(
                    $( $param: $param_type,)*
                ) -> $return_type
            {
                self::__gl_imports::mem::transmute::<_, extern "system" fn($( $param_type,)*) -> $return_type >
                    (ext_storage::$name.f)($( $param,)*)
            }
        )*

        m! {
            $(
                #[allow(non_snake_case, dead_code)] 
                #[inline]
                pub fn "load" $name() -> Result<(), EGLError> {
                    unsafe { ext_storage::$name.load_with_GetProcAddress(concat!("egl", stringify!($name)))}
                }
            )*
        }

        mod ext_storage {
            #![allow(non_snake_case)]
            #![allow(non_upper_case_globals)]
            use super::__gl_imports::raw;
            use super::FnPtr;
            $(
                #[allow(non_snake_case)]
                #[allow(non_upper_case_globals)]
                pub static mut $name: FnPtr = FnPtr {
                    f: super::missing_fn_panic as *const raw::c_void,
                    is_loaded: false
                };
            )*
        }
    }
}

#[allow(non_snake_case, dead_code)] #[inline]
unsafe fn load_with_GetProcAddress(function_name: &str) -> Result<*const __gl_imports::raw::c_void, EGLError> {
    use std::ffi::CString;
    let function_name_cstring = CString::new(function_name).unwrap();
    let fun_ptr = super::GetProcAddress(function_name_cstring.as_ptr()) as *const __gl_imports::raw::c_void;

    if fun_ptr.is_null() {
        Err(EGLError::from_string(format!("could not get address of {}", function_name)))
    } else {
        Ok(fun_ptr)
    }
}

impl FnPtr {
    #[allow(non_snake_case, dead_code)] #[inline]
    pub fn load_with_GetProcAddress(&mut self, function_name: &str) -> Result<(), EGLError> {
        if !self.is_loaded {
                self.f = unsafe { load_with_GetProcAddress(function_name)? };
                self.is_loaded = true;
        };
        Ok(())
    }
}

define_ext_funs!{
    QueryDeviceStringEXT(
        device: types::EGLDeviceEXT,
        name: types::EGLint
    ) -> *const __gl_imports::raw::c_char

    QueryDevicesEXT(max_devices: types::EGLint,
        devices: *mut types::EGLDeviceEXT,
        num_devices: *mut types::EGLint
    ) -> types::EGLBoolean
    
    GetPlatformDisplayEXT(
        platform: types::EGLenum,
        native_display: *const __gl_imports::raw::c_void,
        attrib_list: *const types::EGLint
    ) -> types::EGLDisplay

    CreateImageKHR(
        dpy: types::EGLDisplay,
        ctx: types::EGLContext,
        target: types::EGLenum,
        buffer: types::EGLClientBuffer,
        attrib_list: *const types::EGLint
    ) -> types::EGLImageKHR
    
    DestroyImageKHR(
        dpy: types::EGLDisplay,
        image: types::EGLImageKHR
    ) -> types::EGLBoolean

    BindWaylandDisplayWL(
        disp: types::EGLDisplay,
        display: *mut wl_display
    ) -> types::EGLBoolean

    UnbindWaylandDisplayWL(
        disp: types::EGLDisplay,
        display: *mut wl_display
    ) -> types::EGLBoolean

    QueryWaylandBufferWL(
        displ: types::EGLDisplay,
        buffer: *mut wl_resource,
        attribute: types::EGLint,
        value: *mut types::EGLint
    ) -> types::EGLBoolean

    GetOutputLayersEXT(
        dpy: types::EGLDisplay,
        attrib_list: *const types::EGLAttrib,
        layers: *mut types::EGLOutputLayerEXT,
        max_layers: types::EGLint,
        num_layers: *mut types::EGLint
    ) -> types::EGLBoolean

    QueryOutputLayerAttribEXT(
        dpy: types::EGLDisplay,
        layer: types::EGLOutputLayerEXT,
        attribute: types::EGLint,
        value: *mut types::EGLAttrib
    ) -> types::EGLBoolean

    CreateStreamKHR(
        dpy: types::EGLDisplay,
        attrib_list: *const types::EGLint
    ) -> types::EGLStreamKHR

    DestroyStreamKHR(
        dpy: types::EGLDisplay,
        stream: types::EGLStreamKHR
    ) -> types::EGLBoolean

    QueryStreamKHR(
        dpy: types::EGLDisplay,
        stream: types::EGLStreamKHR,
        attribute: types::EGLenum,
        value: *mut types::EGLint
    ) -> types::EGLBoolean

    CreateStreamProducerSurfaceKHR(
        dpy: types::EGLDisplay,
        config: types::EGLConfig,
        stream: types::EGLStreamKHR,
        attrib_list:  *const types::EGLint
    ) -> types::EGLSurface

    StreamConsumerOutputEXT(
        dpy: types::EGLDisplay,
        stream: types::EGLStreamKHR,
        layer: types::EGLOutputLayerEXT
    ) -> types::EGLBoolean

	CreateStreamAttribNV(
        dpy: types::EGLDisplay,
        attrib_list: *const types::EGLAttrib
    ) -> types::EGLStreamKHR

    StreamConsumerAcquireAttribNV(
        dpy: types::EGLDisplay,
        stream: types::EGLStreamKHR,
        attrib_list: *const types::EGLAttrib
    ) -> types::EGLBoolean

    StreamConsumerGLTextureExternalKHR(
        dpy: types::EGLDisplay,
        stream: types::EGLStreamKHR
    ) -> types::EGLBoolean
    
    CreatePlatformWindowSurfaceEXT(
        dpy: types::EGLDisplay,
        config: types::EGLConfig,
        native_window: *mut __gl_imports::raw::c_void,
        attrib_list: *const EGLint
    ) -> types::EGLSurface 

    SwapBuffersWithDamageEXT(
        dpy: types::EGLDisplay,
        surface: types::EGLSurface,
        rects: *mut EGLint,
        n_rects: EGLint
    ) -> types::EGLBoolean

    SwapBuffersWithDamageKHR(
        dpy: types::EGLDisplay,
        surface: types::EGLSurface,
        rects: *mut EGLint,
        n_rects: EGLint
    ) -> types::EGLBoolean

    QueryDmaBufFormatsEXT(
        dpy: types::EGLDisplay,
        max_formats: types::EGLint,
        formats: *mut types::EGLint,
        num_formats: *mut types::EGLint
    ) -> types::EGLBoolean

    QueryDmaBufModifiersEXT(
        dpy: types::EGLDisplay,
        format: types::EGLint,
        max_modifiers: types::EGLint,
        modifiers: *mut types::EGLuint64KHR,
        external_only: *mut types::EGLBoolean,
        num_modifiers: *mut types::EGLint
    ) -> types::EGLBoolean

    CreateSyncKHR(
        dpy: types::EGLDisplay,
        sync_type: types::EGLenum,
        attrib_list: *const types::EGLint
    ) -> types::EGLSyncKHR

    DestroySyncKHR(
        dpy: types::EGLDisplay,
        sync: types::EGLSyncKHR 
    ) -> types::EGLBoolean

    DupNativeFenceFDANDROID(
        dpy: types::EGLDisplay,
        sync: types::EGLSyncKHR
    ) -> types::EGLint
    
}