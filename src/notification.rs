use glib::object::GObject;
use libc::c_char;
use glib::GError;

pub type NotifyNotification = _NotifyNotification;

#[repr(C)]
struct NotifyNotificationPrivate;

#[repr(C)]
struct _NotifyNotification {
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
