use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use strum_macros::EnumIter;

use crate::components::*;
use crate::pages::*;

#[derive(Routable, Clone, PartialEq, EnumIter, MotionTransitions)]
pub enum Route {
    #[layout(Navbar)]
    #[transition(Fade)]
    #[route("/")]
    Home {},

    #[transition(Fade)]
    #[route("/settings")]
    Settings {},

    #[transition(Fade)]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

impl Route {
    pub fn get_name(&self) -> String {
        use Route::*;

        return match self {
            Home {} => "Home",
            Settings {} => "Settings",
            NotFound { segments } => "404",
        }
        .into();
    }
}
