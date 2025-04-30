#![windows_subsystem = "windows"]
mod app;
mod components;
mod data;
mod pages;
mod panic;
mod route;
mod server;

use crate::panic::capture_panic;
use crate::server::try_pick_random_port;
use app::App;
use dioxus::logger::tracing;
use dioxus::logger::tracing::Level;
use dioxus::prelude::server_fn::client;
use dioxus::prelude::server_only;
use dioxus::{
    desktop::{Config, WindowBuilder},
    LaunchBuilder,
};
use std::convert::Into;
use std::ops::Range;
use std::string::ToString;

fn main() {
    dioxus::logger::init(Level::DEBUG).expect("failed to init logger");
    capture_panic();

    if let Ok(port) = try_pick_random_port(1000..9999) {
        std::env::set_var("PORT", port.to_string());
        std::env::set_var("IP", "127.0.0.1");
        client::set_server_url(Box::leak(
            format!("http://127.0.0.1:{}", port).into_boxed_str(),
        ));

        tracing::info!(
            "Using server url {}\nwith client server url {}",
            dioxus::cli_config::fullstack_address_or_localhost(),
            client::get_server_url()
        );
    }

    let window = WindowBuilder::new()
        .with_title("LE Mod Manager")
        .with_resizable(true);
    let cli_cfg = Config::new().with_menu(None).with_window(window);

    LaunchBuilder::new().with_cfg(cli_cfg).launch(App);
}
