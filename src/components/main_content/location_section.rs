#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::{components::ColumnSize, prelude::*};

use crate::components::common::SectionTitle;
use crate::config::wedding_config;

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
                MapButton {  }
            }
        }
    }
}

#[component]
fn Address() -> Element {
    let config = wedding_config();
    rsx! {
        p {
            class: "address has-text-centered",
            strong { "Venue: " }
            "{config.venue.name}"
            br {  }
            strong { "Address: " }
            "{config.venue.address}"
        }
    }
}

#[component]
fn Map() -> Element {
    let config = wedding_config();
    rsx! {
        div {
            class: "map-frame",
            iframe {
                src: "{config.venue.maps_embed_url}",
                allowfullscreen: "",
                referrerpolicy: "no-referrer-when-downgrade"
            }
        }
    }
}

#[component]
fn MapButton() -> Element {
    let config = wedding_config();
    let url = config.venue.maps_directions_url.clone();
    rsx! {
        Button {
            onclick: move |_| {
                webbrowser::open(&url);
            },
            "Go to Google Map"
        }
    }
}
