use dioxus::prelude::*;
use dioxus_bulma::prelude::*;

mod hero;

const BG_IMAGE: Asset = asset!("/assets/hero-bg.jpg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            id: "home",
            class: "hero is-large",
            background_image: "url({BG_IMAGE})",
            hero::Hero { }
        }
    }
}

