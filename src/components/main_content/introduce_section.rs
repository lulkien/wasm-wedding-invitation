#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::prelude::*;

use crate::{
    components::common::{DividerLeaves, Spacing, SECTION_IMAGE_END},
    database::query,
};

const SECTION_IMAGE_START: Asset = asset!("/assets/floral-border-optimized.webp");

#[component]
pub(super) fn IntroduceSection(id: String) -> Element {
    rsx! {
        section {
            id: "intro",
            class: "section-default introduce-section no-padding-top has-text-centered has-vertically-aligned-content",

            background_image: "url({SECTION_IMAGE_END})",
            background_position: "bottom -30px center",
            background_repeat: "no-repeat",
            background_size: "initial",

            FloralBorderTop {  }
            Message { id }
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
fn Message(id: String) -> Element {
    let message = query(&id);

    let lines = [
        Some(&message.line1),
        message.line2.as_ref(),
        message.line3.as_ref(),
        message.line4.as_ref(),
        message.line5.as_ref(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    rsx! {
        p {
            class: "welcome-text",

            for (i, line) in lines.iter().enumerate() {
                "{line}"
                if i < lines.len() - 1 {
                    br { }
                }
            }
            // // Content
            // "{message.line1}"
            // br { }
            //
            // {message.line2.as_ref().map(|line| rsx! {
            //     "{line}"
            //     br { }
            // })}
            //
            // {message.line3.as_ref().map(|line| rsx! {
            //     "{line}"
            //     br { }
            // })}
            //
            // {message.line4.as_ref().map(|line| rsx! {
            //     "{line}"
            //     br { }
            // })}
            //
            // {message.line5.as_ref().map(|line| rsx! {
            //     "{line}"
            // })}
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
