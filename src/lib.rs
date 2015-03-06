#![feature(libc)]

extern crate libc;
extern crate "glib-2_0-sys" as glib;

use libc::{
    c_char
};

use glib::types::{
    gboolean,
};

use glib::list::GList;

extern "C" {
    pub fn notify_init(app_name: *const c_char) -> gboolean;
    pub fn notify_uninit();
    pub fn notify_is_initted() -> gboolean;

    pub fn notify_get_app_name() -> *const c_char;
    pub fn notify_set_app_name(app_name: *const c_char);

    pub fn notify_get_server_caps() -> *mut GList;
}
