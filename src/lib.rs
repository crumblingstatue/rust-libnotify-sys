extern crate gdk_pixbuf_sys;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate gtypes;

use gdk_pixbuf_sys::GdkPixbuf;
use glib::{GList, GError, GVariant};
use gobject::GObject;
use gtypes::gboolean;
use std::os::raw::c_char;
use std::os::raw::c_int;



enum NotifyNotificationPrivate {}


#[repr(C)]
pub struct NotifyNotification {
    _parent_object: GObject,
    _priv_: NotifyNotificationPrivate,
}

#[repr(C)]
pub enum NotifyUrgency {
    NotifyUrgencyLow,
    NotifyUrgencyNormal,
    NotifyUrgencyCritical,
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

    pub fn notify_notification_update(notification: *mut NotifyNotification,
                                      summary: *const c_char,
                                      body: *const c_char,
                                      icon: *const c_char)
                                      -> gboolean;

    pub fn notify_notification_set_timeout(notification: *mut NotifyNotification, timeout: c_int);

    pub fn notify_notification_set_category(notification: *mut NotifyNotification,
                                             category: *const c_char);

    pub fn notify_notification_set_urgency(notification: *mut NotifyNotification,
                                            urgency: NotifyUrgency);

    pub fn notify_notification_set_image_from_pixbuf(notification: *mut NotifyNotification,
                                                      pixbuf: *mut GdkPixbuf);

    pub fn notify_notification_clear_hints(notification: *mut NotifyNotification);

    pub fn notify_notification_close(notification: *mut NotifyNotification,
                                      error: *mut *mut GError);
}
