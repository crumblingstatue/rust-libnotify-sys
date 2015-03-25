#![feature(libc)]

extern crate libc;
extern crate glib_2_0_sys as glib;
extern crate gobject_2_0_sys as gobject;

pub mod notification;

pub use notification::*;

use libc::c_char;
use glib::types::gboolean;

use glib::GList;

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
}
