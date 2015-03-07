extern crate "pkg-config" as pkg_config;

use pkg_config::Config;

fn main() {
    Config::new().find("libnotify").unwrap();
}
