use dioxus::prelude::*;
//use dioxus_motion::prelude::*;
use strum_macros::EnumIter;

use crate::components::*;
use crate::pages::*;

#[derive(Routable, Clone, PartialEq, EnumIter)]
pub enum Route {
    #[layout(Navbar)]
    //#[transition(Fade)]
    #[route("/")]
    Home {},

    //#[transition(Fade)]
    #[route("/ds2")]
    DS2 {},

    //#[transition(Fade)]
    #[route("/create")]
    Create {},

    //#[transition(Fade)]
    #[route("/settings")]
    Settings {},

    //#[transition(Fade)]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

impl Route {
    pub fn get_name(&self) -> String {
        use Route::*;

        match self {
            Home {} => "Home",
            Create {} => "Create",
            DS2 {} => "Dark Souls II",
            Settings {} => "Settings",
            NotFound { segments } => "404",
        }
        .into()
    }
}
