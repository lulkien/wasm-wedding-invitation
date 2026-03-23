#![allow(unused)]

use dioxus::prelude::*;

const SECTION_IMAGE_DIVIDER: Asset = asset!("/assets/divider-leaves.webp");
pub const SECTION_IMAGE_END: Asset = asset!("/assets/divider-flowers-leaves.webp");

#[derive(Props, Clone, PartialEq)]
pub struct SpacingProps {
    pub space: String,
}

#[component]
pub fn Spacing(props: SpacingProps) -> Element {
    rsx! {
        span {
            class: "spacing",
            style: "--spacing-height: {props.space}",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SectionTitleProps {
    pub name: String,
}

#[component]
pub fn SectionTitle(props: SectionTitleProps) -> Element {
    rsx! {
        div {
            class: "column is-12",
            h1 {
                class: "section-title has-text-centered",
                "{props.name}"
            }
        }
    }
}

#[component]
pub fn DividerLeaves() -> Element {
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
