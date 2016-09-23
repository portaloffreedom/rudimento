pub mod logind;

use std::path::Path;
use std::os::unix::io::RawFd;

pub trait Launcher {
    fn connect(&self) -> Result<(), String> ;
    //fn destroy(&self);
    fn open(&mut self, device_path: &Path) -> Result<RawFd, String>;
    fn close(&mut self);
    fn activate_vt(&self) -> Result<(), String>;
    fn restore(&self);
}
