use backend;
use backend::Backend;
use launcher::Launcher;
use compositor::Compositor;

use libudev;
use libc;

use std::borrow::Borrow;
use std::error::Error as StdError;
use std::fmt;
use std::os::unix::io::RawFd;
use std::path::{Path, PathBuf};
use std::str;
use std::string::String;
use renderer::Renderer;
use renderer::egl::EGLRenderer;
// use renderer::gbm::GBMRenderer;
// use renderer::pixman::PixmanRenderer;

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
    renderer: Box<EGLRenderer>,
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

    pub fn rawfd(&self) -> RawFd {
        self.fd
    }

    pub fn dev_path(&self) -> &Path {
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
}

impl DRMBackend {
    pub fn new(tty: u32, use_pixman: bool, use_egldevice: bool) -> backend::Result<Box<Self>> {
        let udev_context = libudev::Context::new().unwrap();
        let device_devnode_path = try!(DRMBackend::find_primary_gpu(&udev_context, "seat0"));

        use launcher::logind::LogindLauncher;
        let mut launcher = match LogindLauncher::new(Some(tty), "".to_string(), false) {
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
        let clock_type = match get_cap(drm_device.rawfd(), Capability::TimestampMonotonic) {
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


        let cursor_with = match get_cap(drm_device.rawfd(), Capability::CursorWidth) {
            Ok(cap) => cap,
            Err(_) => 64,
        };

        let cursor_height = match get_cap(drm_device.rawfd(), Capability::CursorHeight) {
            Ok(cap) => cap,
            Err(_) => 64,
        };

        let renderer = DRMBackend::init_egl_renderer(&drm_device, use_pixman, use_egldevice)?;

        // b->base.destroy = drm_destroy;
        // b->base.repaint_begin = drm_repaint_begin;
        // b->base.repaint_flush = drm_repaint_flush;
        // b->base.repaint_cancel = drm_repaint_cancel;

        // weston_setup_vt_switch_bindings(compositor);

        // wl_list_init(&b->plane_list);
        // create_sprites(b);

        // if (udev_input_init(&b->input,
        //             compositor, b->udev, seat_id,
        //             config->configure_device) < 0) {
        //     weston_log("failed to create input devices\n");
        //     goto err_sprite;
        // }

        // if (create_outputs(b, drm_device) < 0) {
        //     weston_log("failed to create output for %s\n", b->drm.filename);
        //     goto err_udev_input;
        // }

        // /* A this point we have some idea of whether or not we have a working
        // * cursor plane. */
        // if (!b->cursors_are_broken)
        //     compositor->capabilities |= WESTON_CAP_CURSOR_PLANE;

        // loop = wl_display_get_event_loop(compositor->wl_display);
        // b->drm_source =
        //     wl_event_loop_add_fd(loop, b->drm.fd,
        //                 WL_EVENT_READABLE, on_drm_input, b);

        // b->udev_monitor = udev_monitor_new_from_netlink(b->udev, "udev");
        // if (b->udev_monitor == NULL) {
        //     weston_log("failed to initialize udev monitor\n");
        //     goto err_drm_source;
        // }
        // udev_monitor_filter_add_match_subsystem_devtype(b->udev_monitor,
        //                         "drm", NULL);
        // b->udev_drm_source =
        //     wl_event_loop_add_fd(loop,
        //                 udev_monitor_get_fd(b->udev_monitor),
        //                 WL_EVENT_READABLE, udev_drm_event, b);

        // if (udev_monitor_enable_receiving(b->udev_monitor) < 0) {
        //     weston_log("failed to enable udev-monitor receiving\n");
        //     goto err_udev_monitor;
        // }

        // udev_device_unref(drm_device);

        // weston_compositor_add_debug_binding(compositor, KEY_O,
        //                     planes_binding, b);
        // weston_compositor_add_debug_binding(compositor, KEY_C,
        //                     planes_binding, b);
        // weston_compositor_add_debug_binding(compositor, KEY_V,
        //                     planes_binding, b);
        // weston_compositor_add_debug_binding(compositor, KEY_Q,
        //                     recorder_binding, b);
        // weston_compositor_add_debug_binding(compositor, KEY_W,
        //                     renderer_switch_binding, b);

        // if (compositor->renderer->import_dmabuf) {
        //     if (linux_dmabuf_setup(compositor) < 0)
        //         weston_log("Error: initializing dmabuf "
        //             "support failed.\n");
        // }

        // ret = weston_plugin_api_register(compositor, WESTON_DRM_OUTPUT_API_NAME,
        //                 &api, sizeof(api));

        // if (ret < 0) {
        //     weston_log("Failed to register output API.\n");
        //     goto err_udev_monitor;
        // }

        // return b;

        // err_udev_monitor:
        // wl_event_source_remove(b->udev_drm_source);
        // udev_monitor_unref(b->udev_monitor);
        // err_drm_source:
        // wl_event_source_remove(b->drm_source);
        // err_udev_input:
        // udev_input_destroy(&b->input);
        // err_sprite:
        // if (b->gbm)
        //     gbm_device_destroy(b->gbm);
        // destroy_sprites(b);
        // err_udev_dev:
        // udev_device_unref(drm_device);
        // err_launcher:
        // weston_launcher_destroy(compositor->launcher);
        // err_udev:
        // udev_unref(b->udev);
        // err_compositor:
        // weston_compositor_shutdown(compositor);
        // free(b);
        // return NULL;

        Ok(Box::new(DRMBackend {
            use_pixman,
            use_egldevice,
            //udev_context: udev_context,
            drm_device,
            interface: launcher,
            cursor_with,
            cursor_height,
            compositor,
            renderer,
        }))
    }

    fn find_primary_gpu(udev_context: &libudev::Context, seat: &str) -> backend::Result<PathBuf> {
        let mut enumerator = match libudev::Enumerator::new(&udev_context) {
            Ok(enumerator) => enumerator,
            Err(e) => return Err(Box::new(e)),
        };

        enumerator.match_subsystem("drm")?;
        enumerator.match_sysname("card[0-9]*")?;

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

    fn init_egl_renderer(drm_device: &DRMDevice, use_pixman: bool, use_egldevice: bool) -> Result<Box<EGLRenderer>, DRMBackendError> {
        let renderer_result =
            if use_pixman {
                Err("PixmanRenderer not supported yet".to_string())
                // PixmanRenderer::new()
                //     .map(|renderer| renderer as Box<Renderer>)
            } else { // use egl
                if use_egldevice { // use eglstream (NVIDIA)
                    EGLRenderer::from_drm_device_file(drm_device)
                        // .map(|renderer| renderer as Box<Renderer>)
                        .map_err(|e| format!("{}", e))
                } else {  // use GBM (mesa)
                    Err("GBMRenderer not supported yet".to_string())
                    // GBMRenderer::new(drm_device.rawfd())
                    //     .map(|renderer| renderer as Box<Renderer>)
                }
            };

        renderer_result
            .map_err(|e| DRMBackendError {
                description: format!("{}", e)
            })
    }
}

#[allow(non_snake_case)]
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

    println!("  [device.properties]");
    for property in device.properties() {
        println!("    - {:?} \t{:?}", property.name(), property.value());
    }

    println!("  [device.attributes]");
    for attribute in device.attributes() {
        println!("    - {:?} \t{:?}", attribute.name(), attribute.value());
    }

    println!("\n##########################################################");
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
