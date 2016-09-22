use std::path::Path;
use std::ffi::CString;
use libc::stat;
use libc;
use launcher::Launcher;

use systemd::login;

use dbus;
use dbus::arg::Array;
use std::mem;

macro_rules! dbus_error_to_string_try {
    ( $dbus_result:expr, $error_string:tt) => {
            match $dbus_result {
            Ok(r) => r,
            Err(e) => return Err(format!($error_string, e)),
        }
    };
}

macro_rules! dbus_add_match {
    ( $dbus_conn:expr, $sender:tt, $interface:tt, $member:tt, $path:expr ) => {
        match $dbus_conn.add_match(format!(
			        "type='signal', sender='{}', interface='{}', member='{}', path='{}'",
				    $sender,
				    $interface,
				    $member,
				    $path).as_str()
        ) {
            Ok(r) => r,
            Err(e) => return Err(
                format!("cannot dbus match signal \"{}\" {}", $member, e)
            ),
        }
    };
}

fn my_stat(path: &Path) -> Result<libc::stat,String> {
    let mut file_stat: libc::stat = unsafe {mem::zeroed()};

    let path_string = CString::new(path.as_os_str().to_str().unwrap()).unwrap();
    let r = unsafe {stat(path_string.as_ptr(), &mut file_stat as *mut libc::stat)};

    if r < 0 {
        Err(format!("fail to stat file {}", path.as_os_str().to_string_lossy()))
    } else {
        Ok(file_stat)
    }
}

fn major(rdev: libc::dev_t) -> u32 {
    (rdev >> 8) as u32
}

fn minor(rdev: libc::dev_t) -> u32 {
    (rdev & 0xff) as u32
}

fn is_type(mode: libc::mode_t, mask: libc::mode_t) -> bool {
    (mode & mask) != 0
}

pub struct LogindLauncher {
    sync_drm: bool,
    seat_name: String,
    session_id: String,
    vt: u32,
    dbus_path: String,
    dbus_conn: dbus::Connection,
}

impl LogindLauncher {
    pub fn new(tty: Option<u32>, seat_name: String, sync_drm: bool) -> Result<LogindLauncher, String> {
        //get session
        let session_id = try!(login::get_session()).to_string_lossy().into_owned();

        //get session seat
        let seat_id = try!(login::session_get_seat(&session_id));

        //session get vt and test
        let vt = try!(login::session_get_vt(&session_id));
        match tty {
            Some(tty) => {
                if vt != tty {
                    return Err(format!("logind: requested VT --tty={} differs from real session VT {}",
                            tty, vt));
                }
            }
            None => {}
        }

        let dbus_path = format!("/org/freedesktop/login1/session/{}", &session_id);

        //TODO get wayland event loop

        //create the dbus connection
        let dbus_conn = dbus::Connection::get_private(dbus::BusType::System).unwrap();

        Ok(LogindLauncher {
            sync_drm: sync_drm,
            seat_name: seat_name,
            session_id: session_id,
            vt: vt,
            dbus_path: dbus_path,
            dbus_conn: dbus_conn,
        })
    }

    fn take_device(&self, device_path: &Path) -> Result<(), String>{
        let mut message = try!(dbus::Message::new_method_call(
            "org.freedesktop.login1",
            &self.dbus_path,
            "org.freedesktop.login1.Session",
            "TakeDevice")
        );

        let device_stat = my_stat(device_path).unwrap();

        let is_ifchr = is_type(device_stat.st_mode, libc::S_IFCHR);
        if !is_ifchr {
            panic!("file {} is not character device", device_path.as_os_str().to_string_lossy());
        }

        let major: u32 = major(device_stat.st_rdev);
        let minor: u32 = minor(device_stat.st_rdev);
        message = message.append2(major, minor);

        // send the message
        let reply = dbus_error_to_string_try!(
            self.dbus_conn.send_with_reply_and_block(message, -1),
            "Error sending message \"TakeDevice\": {}"
        );

        Ok(())
    }

    fn release_device(&self, device_major: u32, device_minor: u32) -> Result<(), String> {
        let message = try!(dbus::Message::new_method_call(
            "org.freedesktop.login1",
            &self.dbus_path,
            "org.freedesktop.login1.Session",
            "ReleaseDevice"))
        .append2(device_major, device_minor);

        match self.dbus_conn.send(message) {
            Ok(_) => Ok(()),
            Err(_) => Err("Error sending message \"Activate\"".to_string())
        }
    }

    fn setup_dbus(&self) -> Result<(),String> {
        //rust dbus API missing: add filter

        //weston_dbus_add_match_signal
        // -> SessionRemoved
        dbus_add_match!(self.dbus_conn,
            "org.freedesktop.login1",
            "org.freedesktop.login1.Manager",
            "SessionRemoved",
            "/org/freedesktop/login1");

        //weston_dbus_add_match_signal
        // -> PauseDevice
        dbus_add_match!(self.dbus_conn,
            "org.freedesktop.login1",
            "org.freedesktop.login1.Session",
            "PauseDevice",
            &self.dbus_path);

        //weston_dbus_add_match_signal
        // -> ResumeDevice
        dbus_add_match!(self.dbus_conn,
            "org.freedesktop.login1",
            "org.freedesktop.login1.Session",
            "ResumeDevice",
            &self.dbus_path);

        //weston_dbus_add_match_signal
        // -> PropertiesChanged
        dbus_add_match!(self.dbus_conn,
            "org.freedesktop.login1",
            "org.freedesktop.DBus.Properties",
            "PropertiesChanged",
            &self.dbus_path);

        Ok(())
    }

    fn take_control(&self) -> Result<(),String> {

        let message = try!(dbus::Message::new_method_call(
            "org.freedesktop.login1",
            &self.dbus_path,
            "org.freedesktop.login1.Session",
            "TakeControl")
        ).append1(false); // force

        //dbus_connection_send_with_reply_and_block
        let reply = dbus_error_to_string_try!(
            self.dbus_conn.send_with_reply_and_block(message, -1),
            "Error sending message \"TakeControl\": {}"
        );

        Ok(())
    }

    fn release_control(&self) -> Result<(), String> {
        let message = try!(dbus::Message::new_method_call(
            "org.freedesktop.login1",
            &self.dbus_path,
            "org.freedesktop.login1.Session",
            "ReleaseControl"));

        match self.dbus_conn.send(message) {
            Ok(_) => Ok(()),
            Err(_) => Err("Error sending message \"ReleaseControl\"".to_string())
        }
    }

    fn activate(&self) -> Result<(),String> {
        let message = try!(dbus::Message::new_method_call(
            "org.freedesktop.login1",
            &self.dbus_path,
            "org.freedesktop.login1.Session",
            "Activate"));

        match self.dbus_conn.send(message) {
            Ok(_) => Ok(()),
            Err(_) => Err("Error sending message \"Activate\"".to_string())
        }
    }
}

impl Launcher for LogindLauncher {
    fn connect(&self) {
        self.setup_dbus().unwrap();
        self.take_control().unwrap();
        self.activate().unwrap();
    }

    fn open(&self, device_path: &Path) {
        // logind take device
        self.take_device(device_path).unwrap();
    }

    fn close(&self) {

    }

    fn activate_vt(&self) -> Result<(), String> {
        let message = try!(dbus::Message::new_method_call(
            "org.freedesktop.login1",
            "/org/freedesktop/login1/seat/self",
            "org.freedesktop.login1.Seat",
            "SwitchTo"))
        .append1(&self.vt);

        match self.dbus_conn.send(message) {
            Ok(_) => Ok(()),
            Err(_) => Err("Error sending message \"Activate\"".to_string())
        }
    }

    fn restore(&self) {

    }
}

impl Drop for LogindLauncher {
    fn drop(&mut self) {
        self.release_control();
        //self.dbus_conn is release as soon as it's dropped
    }
}

