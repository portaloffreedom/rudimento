pub mod drm;

use std::error::Error as StdError;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T,Box<StdError>>;

pub trait Backend {
    // fn load_backend(use_pixman: bool, use_egldevice: bool) -> Result<Box<Self>>;
}
