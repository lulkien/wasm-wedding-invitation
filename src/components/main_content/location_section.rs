#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::{components::ColumnSize, prelude::*};

use crate::components::common::{SectionTitle, Spacing};

#[component]
pub(super) fn LocationSection() -> Element {
    rsx! {
        section { 
            id: "location",
            class: "section-default location-section has-text-centered has-vertically-aligned-content",

            Container {
                SectionTitle { name: "Location" }
                Address {  }
                Map {  }
            }
        }
    }
}

#[component]
fn Address() -> Element {
    rsx! {
        p {
            class: "address has-text-centered",
            strong {
                color: "#181818",
                "Venue: "
            }
            "Near Doc Suoi Roundabout"
            br {  }
            strong {
                color: "#181818",
                "Address: "
            }
            "Trieu Tien Village, Son Nam Ward, Hung Yen Province"
        }
    }
}

#[component]
fn Map() -> Element {
    rsx! {
        div { 
            class: "map-frame",
            iframe { 
                src: "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3732.4018810793696!2d106.05648041264276!3d20.693899080793273!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x3135c74c3c5622c3%3A0xd00469807a33cd80!2zSMawxqFuZyBUcuG6p20gSHV54buBbiBBbmg!5e0!3m2!1sen!2sus!4v1772970153119!5m2!1sen!2sus",
                width: "100%",
                height: "450",
                style: "border:0;",
                allowfullscreen: "",
                referrerpolicy: "no-referrer-when-downgrade"
            }
        }
    }
}
