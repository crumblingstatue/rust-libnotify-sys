extern crate gobject_2_0_sys as gobject;
extern crate glib_sys as glib;
extern crate gtypes;

use std::os::raw::c_char;
use gtypes::gboolean;
use gobject::GObject;

use glib::{GList, GError, GVariant};

enum NotifyNotificationPrivate {}

#[repr(C)]
pub struct NotifyNotification {
    _parent_object: GObject,
    _priv_: NotifyNotificationPrivate,
}

extern "C" {
    pub fn notify_init(app_name: *const c_char) -> gboolean;
    pub fn notify_uninit();
    pub fn notify_is_initted() -> gboolean;

    pub fn notify_get_app_name() -> *const c_char;
    pub fn notify_set_app_name(app_name: *const c_char);

    pub fn notify_get_server_caps() -> *mut GList;

    pub fn notify_get_server_info(ret_name: *mut *mut c_char,
                                  ret_vendor: *mut *mut c_char,
                                  ret_version: *mut *mut c_char,
                                  ret_spec_version: *mut *mut c_char);

    pub fn notify_notification_new(summary: *const c_char,
                                   body: *const c_char,
                                   icon: *const c_char)
                                   -> *mut NotifyNotification;

    pub fn notify_notification_show(notification: *mut NotifyNotification,
                                    error: *mut *mut GError);

    pub fn notify_notification_set_hint(notification: *mut NotifyNotification,
                                        key: *const c_char,
                                        value: *mut GVariant);
}
