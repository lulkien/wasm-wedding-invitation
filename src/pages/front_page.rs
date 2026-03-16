#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::components::Title;

use crate::components::{home, main_content};

const FAVICON: Asset = asset!("/assets/favicon.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn FrontPage() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {  }
    }
}

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    PageNotFound,

    #[route("/invitation/:id")]
    Invitation { id: String },
}

#[component]
fn PageNotFound() -> Element {
    rsx! {
        div { 
            display: "flex",
            justify_content: "center",
            align_items: "center",
            height: "100vh",

            Title {
                "Nothing here my friend xD"
            }
        }
    }
}


#[component]
pub fn Invitation(id: String) -> Element {
    rsx! {
        div {
            background_color: "ghostwhite",

            home::HeaderWrapper {  }
            main_content::MainContent {  }
        }
    }
}
