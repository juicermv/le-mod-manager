use dioxus::prelude::*;
use strum_macros::Display;

#[derive(Display, Debug, Clone, PartialEq, Eq)]
pub enum ToastType {
    #[strum(serialize = "text-bg-info")]
    Info,

    #[strum(serialize = "text-bg-success")]
    Success,

    #[strum(serialize = "text-bg-warning")]
    Warning,

    #[strum(serialize = "text-bg-danger")]
    Error,
}

#[derive(Default, Clone, PartialEq)]
pub struct ToastManager {
    pub toasts: Signal<Vec<(String, ToastType)>>,
}


impl ToastManager {
    pub fn new() -> Self {
        Self {
            toasts: Signal::new(vec![]),
        }
    }

    pub fn add(&mut self, message: String, toast_type: ToastType) {
        let mut toasts = self.toasts.read().clone();
        toasts.push((message, toast_type));
        self.toasts.set(toasts);
    }

    pub fn remove(&mut self, index: usize) {
        let mut toasts = self.toasts.read().clone();
        if index < toasts.len() {
            toasts.remove(index);
            self.toasts.set(toasts);
        }
    }
}