#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::prelude::*;

mod introduce_section;
mod time_section;
mod location_section;

#[component]
pub fn MainContent() -> Element {
    rsx! {
        div {
            class: "main-content",

            introduce_section::IntroduceSection {  }
            time_section::TimeSection {  }
            location_section::LocationSection {  }
        }
    }
}

