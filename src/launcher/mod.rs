pub mod logind;

use std::path::Path;

pub trait Launcher {
    fn connect(&self);
    //fn destroy(&self);
    fn open(&self, device_path: &Path);
    fn close(&self);
    fn activate_vt(&self) -> Result<(), String>;
    fn restore(&self);
}
