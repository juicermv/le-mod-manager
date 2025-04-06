mod app;
mod components;
mod data;
mod pages;
mod route;

use app::App;

fn main() {
    dioxus::launch(App);
}
