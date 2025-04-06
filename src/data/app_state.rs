use dioxus::signals::Signal;

#[derive(Clone)]
pub struct AppState {
    pub filepath: Signal<String>,
}
