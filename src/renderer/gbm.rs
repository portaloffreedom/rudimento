use renderer::Renderer;
use std::os::unix::io::RawFd;

pub struct GBMRenderer {

}

impl Renderer for GBMRenderer {

}


impl GBMRenderer {
    pub fn new(fd: RawFd) -> Result<Box<Self>, String> { 
        //TODO find gbm library in crates.io
        //TODO load libglapi.so.0
        //TODO return gbm_create_device(fd);
        Err("GBMRenderer not yet implemtended".to_string())
    }
}
