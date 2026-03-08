#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::{components::ColumnSize, prelude::*};

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
