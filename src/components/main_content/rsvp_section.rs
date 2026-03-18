#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::{components::ColumnSize, prelude::*};

use crate::components::common::{SectionTitle, Spacing};

#[component]
pub(super) fn RsvpSection() -> Element {
    rsx! {
        section { 
            id: "rsvp",
            class: "section-default rsvp-section has-text-centered has-vertically-aligned-content",

            Container {
                SectionTitle { name: "RSVP" }
                Message {  }
            }
        }
    }
}

#[component]
fn Message() -> Element {
    rsx! {
        p {
            class: "message",
            "It would be an honor to have you join us and offer your blessing."
            br { }
            br { }
            "Shuttles will depart from FPT Tower and Lotte Mall West Lake."
            br { }
            br { }
            "Please let us know if you can attend; we look forward to celebrating!"
            br { }
            br { }
            "If you can’t make it, we’ll miss you and hope to see you soon."
        }
    }
}
