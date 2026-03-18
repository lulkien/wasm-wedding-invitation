#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::{components::ColumnSize, prelude::*};

const SECTION_IMAGE_DIVIDER: Asset = asset!("/assets/divider-leaves-optimized.webp");
pub const SECTION_IMAGE_END: Asset = asset!("/assets/divider-flowers-leaves-optimized.webp");

#[derive(Props, Clone, PartialEq)]
pub struct SpacingProps {
    pub space: String,
}

#[component]
pub fn Spacing(props: SpacingProps) -> Element {
    rsx! {
        span {
            width: "100%",
            height: "{props.space}"
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
        Column {
            size: ColumnSize::Twelve,

            h1 {
                class: "section-title has-text-centered title",
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
