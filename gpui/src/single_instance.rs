use std::ffi::c_void;
use std::io;
use std::ptr;
use std::time::{Duration, Instant};

pub const WINDOW_TITLE: &str = "OpenMouse Desktop";

const ERROR_ALREADY_EXISTS: u32 = 183;
const SW_RESTORE: i32 = 9;

#[link(name = "kernel32")]
extern "system" {
    fn CreateMutexW(attributes: *const c_void, initial_owner: i32, name: *const u16) -> *mut c_void;
    fn GetLastError() -> u32;
    fn SetLastError(error: u32);
    fn CloseHandle(handle: *mut c_void) -> i32;
}

#[link(name = "user32")]
extern "system" {
    fn FindWindowW(class_name: *const u16, window_name: *const u16) -> isize;
    fn ShowWindowAsync(hwnd: isize, command: i32) -> i32;
    fn SetForegroundWindow(hwnd: isize) -> i32;
}

pub struct Instance(*mut c_void);

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

pub fn acquire() -> io::Result<Option<Instance>> {
    let name: Vec<u16> = "Local\\OpenMouseDesktop".encode_utf16().chain(Some(0)).collect();
    let (handle, error) = unsafe {
        SetLastError(0);
        let handle = CreateMutexW(ptr::null(), 0, name.as_ptr());
        (handle, GetLastError())
    };
    if handle.is_null() {
        return Err(io::Error::from_raw_os_error(error as i32));
    }

    // Keeping the named object alive reserves this desktop session for the app.
    let instance = Instance(handle);
    if error != ERROR_ALREADY_EXISTS {
        return Ok(Some(instance));
    }
    drop(instance);
    show_existing_window()?;
    Ok(None)
}

fn show_existing_window() -> io::Result<()> {
    let title: Vec<u16> = WINDOW_TITLE.encode_utf16().chain(Some(0)).collect();
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let hwnd = unsafe { FindWindowW(ptr::null(), title.as_ptr()) };
        if hwnd != 0 {
            if unsafe { ShowWindowAsync(hwnd, SW_RESTORE) } == 0 {
                return Err(io::Error::other("could not restore the existing OpenMouse Desktop window"));
            }
            unsafe {
                SetForegroundWindow(hwnd);
            }
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "OpenMouse Desktop is already running, but its window is not ready",
            ));
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
