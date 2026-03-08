#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::prelude::*;

use crate::components::common::Spacing;

const SECTION_IMAGE_END: Asset = asset!("/assets/divider-flowers-leaves-optimized.webp");
const SECTION_IMAGE_DIVIDER: Asset = asset!("/assets/divider-leaves-optimized.webp");
const SECTION_IMAGE_START: Asset = asset!("/assets/floral-border-optimized.webp");

#[component]
pub(super) fn IntroduceSection() -> Element {
    rsx! {
        section {
            id: "introduce-section",
            class: "section-default introduce-section no-padding-top has-text-centered has-vertically-aligned-content",

            background_image: "url({SECTION_IMAGE_END})",
            background_position: "bottom -30px center",
            background_repeat: "no-repeat",
            background_size: "initial",

            FloralBorderTop {  }
            Message {  }
            Spacing { space: "40px" }
            Spacing { space: "40px" }
            BrideAndGroomItem {  }
            Spacing { space: "40px" }
            DividerLeaves {  }
            Spacing { space: "40px" }
        }
    }
}

#[component]
fn FloralBorderTop() -> Element {
    rsx! {
        div {
            img {
                class: "floral-border has-text-centered",
                src: SECTION_IMAGE_START,
                alt: "floralborder"
            }
        }
    }
}

#[component]
fn Message() -> Element {
    rsx! {
        p {
            class: "welcome-text",
            // Content
            "My dearest friend"
            br {  }
            "The big day is officially happening, and you're one of the first people I wanted to tell."
            br {  }
            "Would be so happy to have you by my side."
        }
    }
}

#[component]
fn BrideAndGroomItem() -> Element {
    rsx! {
        div {
            BrideAndGroom { name: "Luu Hoang Kien" }
            div { class: "ampersand", "&" }
            BrideAndGroom { name: "Pham Thi Hang" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct BrideAndGroomProps {
    name: String,
}

#[component]
fn BrideAndGroom(props: BrideAndGroomProps) -> Element {
    rsx! {
        div {
            class: "bride-groom-name",
            "{props.name}"
        }
    }
}

#[component]
fn DividerLeaves() -> Element {
    rsx! {
        div {
            img {
                class: "divider has-text-centered",
                src: SECTION_IMAGE_DIVIDER,
                alt: "~~~"
            }
        }
    }
}
