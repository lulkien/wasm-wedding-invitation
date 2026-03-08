use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
use dioxus_bulma::components::*;

mod hero;
mod hero_foot;

const BG_IMAGE: Asset = asset!("/assets/hero-bg.jpg");

#[component]
pub fn HeaderWrapper() -> Element {
    rsx! {
        div { 
            id: "home",

            section {
                class: "hero is-large",
                background_image: "url({BG_IMAGE})",

                hero::Hero { }
                hero_foot::HeroFoot {  }
            }
        }
    }
}

