use dioxus::prelude::*;
//use dioxus_motion::transitions::page_transitions::AnimatedOutlet;
use strum::IntoEnumIterator;

use crate::route::Route;

#[component]
pub fn Navbar() -> Element {
    let current_route: Route = use_route();

    rsx! {
        nav {
            class: "sticky-top navbar navbar-expand bg-body-tertiary",
            div {
                class: "container-fluid" ,
                a {
                    href: "#",
                    class: "navbar-brand position-relative",
                    i {
                        class: "bi bi-cone-striped me-2 text-warning"
                    }
                    "LE Mod Manager (Preview)"
                }
                div {
                    class: "collapse navbar-collapse",
                    ul {
                        class: "navbar-nav",
                        for route in Route::iter() {
                            {
                                match route {
                                    Route::NotFound { segments: _ } => {
                                        rsx!()
                                    }
                                    _ => {
                                        let active = if route == current_route {
                                            "active"
                                        } else {
                                            ""
                                        };

                                        rsx! {
                                            li {
                                                class: "nav-item",
                                                Link { class: "nav-link {active}",
                                                    to: route.clone(),
                                                    {route.get_name()}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

            }
        }

        Outlet::<Route> {}
    }
}
