extern crate libnotify_sys as libnotify;
extern crate glib_2_0_sys as glib;

fn main() {
    unsafe {
        use libnotify::*;
        use std::mem::transmute as tm;

        if notify_init(tm("hello\0".as_ptr())) == 0 {
            panic!("Failed to init libnotify");
        }

        let notif = notify_notification_new(tm("Hello\0".as_ptr()),
                                            tm("World\0".as_ptr()),
                                            0 as *const i8);
        let mut error = 0 as *mut glib::GError;
        notify_notification_show(notif, &mut error);
        if error != 0 as *mut glib::GError {
            glib::g_error_free(error);
        }

        notify_uninit();
    }
}
