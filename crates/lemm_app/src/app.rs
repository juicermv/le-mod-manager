use crate::{data::AppState, route::Route};
use dioxus::prelude::*;
use lib_lemm::data::{to_ascii_array, PackageHeader};
use crate::components::ToastArea;
use crate::pages::{DS2State, SettingsState, ToastManager};

#[component]
pub fn App() -> Element {
    // Create state
    use_context_provider(|| DS2State::read());
    use_context_provider(|| SettingsState::read());
    let toast_manager = use_context_provider(|| ToastManager::new());
    let toasts = toast_manager.toasts;

    rsx! {
        document::Link {
            href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.5/dist/css/bootstrap.min.css",
            rel:"stylesheet",
            integrity:"sha384-SgOJa3DmI69IUzQ2PVdRZhwQ+dy64/BUtbMJw1MZ8t5HZApcHrRKUc4W0kG879m7",
            crossorigin: "anonymous"
        }

        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css"
        }

        document::Script {
            src: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.5/dist/js/bootstrap.bundle.min.js",
            integrity: "sha384-k6d4wzSIapyDyv1kpU366/PK5hCdSbCRGRCMv+eplOQJWyd1fbcAu9OCUj5zNLiq",
            crossorigin: "anonymous"
        }

        document::Script {
            src: asset!("assets/update_theme.js")
        }

        ToastArea {
            toasts: toasts(),
            on_remove: move |id| use_context::<ToastManager>().remove(id)
        }

        Router::<Route> { }
    }
}
