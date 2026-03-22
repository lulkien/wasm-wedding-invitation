use dioxus::prelude::*;
use dioxus_bulma::components::Container;

const BG_IMAGE: Asset = asset!("/assets/hero-bg.jpg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            id: "home",
            class: "hero is-large",
            background_image: "url({BG_IMAGE})",
            div {
                class: "hero-body",
                Container {
                    class: "has-text-centered",
                    Subtitle {  }
                    Title {  }
                    DateAndTime {  }
                }
            }
        }
    }
}

#[component]
fn Subtitle() -> Element {
    rsx! {
        h1 {
            class: "subtitle subtitle--lead",
            "Wedding Invitation"
        }
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        h2 {
            class: "title",
            "Hoang Kien & Pham Hang"
        }
    }
}

#[component]
fn DateAndTime() -> Element {
    rsx! {
        h4 {
            class: "subtitle",
            "Saturday, March 28, 2026"
            br {  }
            "Trieu Tien, Son Nam Ward"
            br {  }
            "Hung Yen Province"
        }
    }
}
