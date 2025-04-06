use crate::{data::AppState, route::Route};
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let app_state = use_context_provider(|| AppState {
        filepath: Signal::new("".into())
    });

    rsx! {
        document::Link {
            href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.5/dist/css/bootstrap.min.css",
            rel:"stylesheet",
            integrity:"sha384-SgOJa3DmI69IUzQ2PVdRZhwQ+dy64/BUtbMJw1MZ8t5HZApcHrRKUc4W0kG879m7",
            crossorigin: "anonymous"
        }
        document::Script {
            src: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.5/dist/js/bootstrap.bundle.min.js",
            integrity: "sha384-k6d4wzSIapyDyv1kpU366/PK5hCdSbCRGRCMv+eplOQJWyd1fbcAu9OCUj5zNLiq",
            crossorigin: "anonymous"
        }

        document::Script {
            src: asset!("assets/update_theme.js")
        }

        Router::<Route> { }
    }
}
