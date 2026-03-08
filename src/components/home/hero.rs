use dioxus::prelude::*;
use dioxus_bulma::components::Container;

#[component]
pub(super) fn Hero() -> Element {
    rsx! {
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

#[component]
fn Subtitle() -> Element {
    rsx! {
        h1 { 
            class: "subtitle",
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
    rsx!{
        h4 {
            class: "subtitle",

            "Saturday, March 28, 2026"
            br {}
            "Trieu Tien, Son Nam Ward"
            br {}
            "Hung Yen Province"
        }
    }
}
