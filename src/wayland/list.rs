use wayland_server::sys::common::wl_list;
use wayland_server::sys::server::{
    wl_list_init,
    wl_list_insert,
    wl_list_remove,
    wl_list_length,
    wl_list_empty, 
    wl_list_insert_list,
};

pub struct List {
    raw: wl_list,
}

impl Drop for List {
    fn drop(&mut self) {
        self.remove()
    }
}

impl List {
    pub fn new() -> Self {
        let mut raw = wl_list {
            prev: 0 as *mut wl_list,
            next: 0 as *mut wl_list,
        };
        unsafe {wl_list_init(&mut raw)};
        Self {
            raw,
        }
    }

    pub fn insert(&mut self, elem: &mut Self) {
        // elem does not need to be initialized here
        unsafe {wl_list_insert(&mut self.raw, &mut elem.raw)};
    }

    pub fn remove(&mut self) {
        unsafe {wl_list_remove(&mut self.raw)};
    }

    pub fn length(&self) -> usize {
        unsafe { wl_list_length(&self.raw) as usize }
    }
    pub fn empty(&self) -> bool {
        // returns 1 if empty, 0 if not empty
        unsafe { wl_list_empty(&self.raw) == 1 }
    }

    pub fn insert_list(&mut self, other: &mut Self) {
        unsafe {wl_list_insert_list(&mut self.raw, &mut other.raw)};
    }

 
}