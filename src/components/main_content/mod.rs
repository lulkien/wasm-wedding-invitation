#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::prelude::*;

mod introduce_section;
mod time_section;
mod location_section;
mod rsvp_section;

#[component]
pub fn MainContent(id: String) -> Element {
    rsx! {
        div {
            class: "main-content",

            introduce_section::IntroduceSection { id }
            time_section::TimeSection {  }
            location_section::LocationSection {  }
            rsvp_section::RsvpSection {  }
        }
    }
}

