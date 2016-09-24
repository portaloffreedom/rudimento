use backend;
use backend::Backend;
use launcher::Launcher;
use compositor::Compositor;

use egl::types::EGLDeviceEXT;
use libudev;
use libc;

use std::borrow::Borrow;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::os::unix::io::RawFd;
use std::path::{Path, PathBuf};
use std::str;
use std::string::String;
use renderer::Renderer;
use renderer::egl::EglRenderer;
use renderer::pixman::PixmanRenderer;

pub struct DRMBackend {
    //compositor
    use_pixman: bool,
    use_egldevice: bool,
    //egl_device: EGLDeviceEXT,
    //udev_context: libudev::Context,
    drm_device: DRMDevice,
    interface: Box<Launcher>,
    cursor_with: u64,
    cursor_height: u64,
    compositor: Compositor,
    renderer: Box<Renderer>,
}

pub struct DRMDevice {
    fd: RawFd,
    filename: PathBuf,
}

impl DRMDevice {
    fn new(fd: RawFd, filename: PathBuf) -> DRMDevice {
        DRMDevice {
            fd: fd,
            filename: filename,
        }
    }

    fn as_rawfd(&self) -> &RawFd {
        &self.fd
    }

    fn dev_path(&self) -> &Path {
        &self.filename
    }
}

#[derive(Debug)]
struct DRMBackendError {
    description: String
}

impl StdError for DRMBackendError {
    fn description(&self) -> &str {
        return self.description.borrow();
    }
}

impl fmt::Display for DRMBackendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.description)
    }
}

impl Backend for DRMBackend {
    fn load_backend(use_pixman: bool) -> backend::Result<Box<Self>> {
        let mut udev_context = libudev::Context::new().unwrap();
        let device_devnode_path = try!(DRMBackend::find_primary_gpu(&udev_context, "seat0"));

        use launcher::logind::LogindLauncher;
        let mut launcher = match LogindLauncher::new(Some(2), "".to_string(), false) {
            Ok(l) => Box::new(l),
            Err(e) => return Err(Box::new(DRMBackendError {
                description: e
            })),
        };

        match launcher.connect().err() {
            Some(e) =>  return Err(Box::new(DRMBackendError {
                description: e
            })),
            None => {},
        }

        use libc::O_RDWR;
        let fd = match launcher.open(&device_devnode_path, O_RDWR) {
            Ok(fd) => fd,
            Err(e) => return Err(Box::new(DRMBackendError {
                description: e
            })),
        };

        let drm_device = DRMDevice::new(fd, device_devnode_path);

        use libdrm::drm::{get_cap,Capability};
        let clock_type = match get_cap(drm_device.as_rawfd(), Capability::TimestampMonotonic) {
            Ok(cap) => {
                if cap == 1 {
                    libc::CLOCK_MONOTONIC
                } else {
                    libc::CLOCK_REALTIME
                }
            },
            Err(e) => {
                println!("drm get capabilities failed with return code {}", e);
                libc::CLOCK_REALTIME
            },
        };

        let compositor = match Compositor::new(clock_type) {
            Ok(c) => c,
            Err(e) => return Err(Box::new(DRMBackendError {
                description: e
            })),
        };


        let cursor_with = match get_cap(drm_device.as_rawfd(), Capability::CursorWidth) {
            Ok(cap) => cap,
            Err(_) => 64,
        };

        let cursor_height = match get_cap(drm_device.as_rawfd(), Capability::CursorHeight) {
            Ok(cap) => cap,
            Err(_) => 64,
        };

        let renderer_result =
            if use_pixman {
                return Err(Box::new(DRMBackendError {
                    description: "Pixman not supported yet, only egl is supported".to_string()
                }));
                PixmanRenderer::new()
            } else { // use egl
                EglRenderer::new(drm_device.dev_path())
            };

        let renderer = match renderer_result {
            Ok(r) => Box::new(r),
            Err(e) => return Err(Box::new(DRMBackendError {
                description: e
            })),
        };

        let mut backend = DRMBackend {
            use_pixman: false,
            use_egldevice: true,
            //udev_context: udev_context,
            drm_device: drm_device,
            interface: launcher,
            cursor_with: cursor_with,
            cursor_height: cursor_height,
            compositor: compositor,
            renderer: renderer
        };

        Ok(Box::new(backend))
    }
}

impl DRMBackend {
    fn find_primary_gpu(udev_context: &libudev::Context, seat: &str) -> backend::Result<PathBuf> {
        let mut enumerator = match libudev::Enumerator::new(&udev_context) {
            Ok(enumerator) => enumerator,
            Err(e) => return Err(Box::new(e)),
        };

        enumerator.match_subsystem("drm");
        enumerator.match_sysname("card[0-9]*");

        let default_seat = "seat0";
        let device_list = match enumerator.scan_devices() {
            Ok(device_list) => device_list,
            Err(e) => return Err(Box::new(e)),
        };

        let device_option = device_list.into_iter().find(|device| {

            let device_seat = match device.property_value("ID_SEAT") {
                Some(seat_str) => seat_str.to_str().unwrap_or(default_seat),
                None => default_seat,
            };

            if device_seat != seat {
                return false;
            }

            if parent_with_subsystem_and_test(device, "pci", |pci| {
                match pci.attribute_value("boot_vga") {
                    Some(boot_vga_id) => {
                        if boot_vga_id.to_str() == Some("1") {
                            //return Ok(Some(device));
                            return true
                        } else {
                            return false
                        }
                    },
                    None => false,
                }
            }) {
                return true;
            }

            return false;
        });

        let device =  match device_option {
            Some(dev) => dev,
            None => return Err(Box::new(DRMBackendError {
                description: "No device found!".to_string(),
            })),
        };

        let devnode = match device.devnode() {
            Some(devnode) => devnode,
            None => return Err(Box::new(DRMBackendError {
                description: "Error getting device devnode!".to_string(),
            })),
        };

        PrintUDEVDeviceInfo(&device);

        Ok(devnode.to_path_buf())
    }
}

fn PrintUDEVDeviceInfo(device: &libudev::Device) {
    println!("\n##########################################################");
    println!("initialized: {:?}", device.is_initialized());
    println!("     devnum: {:?}", device.devnum());
    println!("    syspath: {:?}", device.syspath());
    println!("    devpath: {:?}", device.devpath());
    println!("  subsystem: {:?}", device.subsystem());
    println!("    sysname: {:?}", device.sysname());
    println!("     sysnum: {:?}", device.sysnum());
    println!("    devtype: {:?}", device.devtype());
    println!("     driver: {:?}", device.driver());
    println!("    devnode: {:?}", device.devnode());

    if let Some(parent) = device.parent() {
        println!("     parent: {:?}", parent.syspath());
    } else {
        println!("     parent: None");
    }

    println!("  [properties]");
    for property in device.properties() {
        println!("    - {:?} {:?}", property.name(), property.value());
    }

    println!("  [attributes]");
    for attribute in device.attributes() {
        println!("    - {:?} {:?}", attribute.name(), attribute.value());
    }
}

//trait SubSystemSearchable {
//    fn parent_with_subsystem(&self, subsystem: &str) -> Option<libudev::Device>;
//}

//impl<'a> SubSystemSearchable for libudev::Device<'a> {
//    fn parent_with_subsystem(&self, subsystem: &str) -> Option<libudev::Device> {
//        let mut parent: libudev::Device;
//        let mut parent_option = self.parent();
//
//        while match parent_option {
//            Some(_) => true,
//            None => false
//        }{
//            parent = parent_option.unwrap();
//            let parent_subsystem = parent.subsystem();
//
//            if parent_subsystem == subsystem {
//                return Some(parent);
//            }
//
//            parent_option = parent.parent();
//        }
//
//        None
//    }
//}

//fn parent_with_subsystem<'context>(device: &libudev::Device, subsystem: &str) -> Option<Rc<libudev::Device<'context>>> {
//
//    match device.parent() {
//        None => return None,
//        Some(parent) => {
//            let is_parent_subsistem =
//            parent.subsystem() == subsystem;
//
//            let p = Rc::new(parent);
//
//            if is_parent_subsistem {
//                return Some(p.clone())
//            } else {
//                return parent_with_subsystem(&p, subsystem)
//            }
//        }
//    };
//}

fn parent_with_subsystem_and_test<F>(device: &libudev::Device, subsystem: &str, test: F) -> bool
    where F : Fn(&libudev::Device) -> bool {

    match device.parent() {
        None => return false,
        Some(parent) => {
            let is_parent_subsistem =
                parent.subsystem() == subsystem;

            if is_parent_subsistem {
                return test(&parent);
            } else {
                return parent_with_subsystem_and_test(&parent, subsystem, test);
            }
        }
    };
}
