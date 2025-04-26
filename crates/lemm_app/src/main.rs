#![windows_subsystem = "windows"]

mod app;
mod components;
mod data;
mod pages;
mod route;
mod server;

use app::App;
use dioxus::{
    desktop::{Config, WindowBuilder},
    LaunchBuilder,
};

fn main() {
    let window = WindowBuilder::new()
        .with_title("LE Mod Manager")
        .with_resizable(true);
    let cfg = Config::new().with_menu(None).with_window(window);

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}
