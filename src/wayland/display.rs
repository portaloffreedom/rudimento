use wayland_server;
use wayland_server::sys::server::wl_display;
use wayland_server::protocol::wl_shm;
use wayland::WaylandError;

pub struct Display {
    raw: wl_display,
}

impl Display {
    pub unsafe fn from_raw(raw: wl_display) -> Self {
        Self {
            raw,
        }
    }

    /**
     * Add support for a wl_shm pixel format.
     * @param format The wl_shm pixel format to advertise 
     * @returns A pointer to the wl_shm format that was added to the list or NULL if adding it to the list failed.
     */
    pub fn add_shm_format(&mut self, format: wl_shm::Format) -> Result<(), WaylandError> {
        let r = unsafe { wayland_server::sys::server::wl_display_add_shm_format(&mut self.raw, format.to_raw()) };
        if r.is_null() {
            Err(WaylandError::from_str("wl_display_add_shm_format error"))
        } else {
            Ok(()) // Ok((r))
        }
    }

    pub fn raw_ref(&self) -> &wl_display {
        &self.raw
    }

    pub fn raw_mut_ref(&mut self) -> &mut wl_display {
        &mut self.raw
    }
}