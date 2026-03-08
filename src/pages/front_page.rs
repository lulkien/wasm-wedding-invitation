#![allow(unused)]

use dioxus::prelude::*;

use crate::components::{home, main_content};

const FAVICON: Asset = asset!("/assets/favicon.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn FrontPage() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        FrontPageContainer {  }
    }
}

#[component]
pub fn FrontPageContainer() -> Element {
    rsx! {
        div {
            background_color: "ghostwhite",

            home::HeaderWrapper {  }
            main_content::MainContent {  }
        }
    }
}
