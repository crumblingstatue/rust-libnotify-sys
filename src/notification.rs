use gobject::GObject;
use libc::c_char;
use glib::GError;

enum NotifyNotificationPrivate {}

#[repr(C)]
pub struct NotifyNotification {
    _parent_object: GObject,
    _priv_: NotifyNotificationPrivate
}

extern "C" {
    pub fn notify_notification_new(summary: *const c_char,
                                   body: *const c_char,
                                   icon: *const c_char)
                                   -> *mut NotifyNotification;
    pub fn notify_notification_show(notification: *mut NotifyNotification,
                                    error: *mut *mut GError);
}
