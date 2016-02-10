extern crate libnotify_sys as libnotify;
extern crate glib_2_0_sys as glib;

fn main() {
    unsafe {
        use libnotify::*;
        use std::os::raw::c_char;
        use std::ptr;

        if notify_init("hello\0".as_ptr() as *const c_char) == 0 {
            panic!("Failed to init libnotify");
        }

        let notif = notify_notification_new("Hello\0".as_ptr() as *const c_char,
                                            "World\0".as_ptr() as *const c_char,
                                            ptr::null());
        let mut error = ptr::null_mut();
        notify_notification_show(notif, &mut error);

        if error != ptr::null_mut() {
            glib::g_error_free(error);
        }

        notify_uninit();
    }
}
