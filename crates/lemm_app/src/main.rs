mod app;
mod components;
mod data;
mod pages;
mod route;
mod server;

use app::App;

fn main() {
    dioxus::launch(App);
}
