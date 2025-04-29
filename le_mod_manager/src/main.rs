#![windows_subsystem = "windows"]
mod app;
mod components;
mod data;
mod pages;
mod panic;
mod route;
mod server;

use crate::panic::capture_panic;
use app::App;
use dioxus::{
    desktop::{Config, WindowBuilder},
    LaunchBuilder,
};

fn main() {
    capture_panic();
    
    let window = WindowBuilder::new()
        .with_title("LE Mod Manager")
        .with_resizable(true);
    let cfg = Config::new().with_menu(None).with_window(window);

    LaunchBuilder::new().with_cfg(cfg).launch(App);
}
