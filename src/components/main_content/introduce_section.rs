#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::prelude::*;

use crate::{
    components::common::{DividerLeaves, Spacing, SECTION_IMAGE_END},
    database::Person,
};

const SECTION_IMAGE_START: Asset = asset!("/assets/floral-border-optimized.webp");

#[component]
pub(super) fn IntroduceSection(get_user_data: Signal<Option<Person>>) -> Element {
    let Some(user) = get_user_data() else {
        return rsx! { div { } };
    };

    rsx! {
        section {
            id: "intro",
            class: "section-default introduce-section no-padding-top has-text-centered has-vertically-aligned-content",
            background_image: "url({SECTION_IMAGE_END})",
            FloralBorderTop {  }
            Message {
                greeting: user.greeting,
                name: user.name,
                line1: user.line1,
                line2: user.line2,
                line3: user.line3,
            }
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

#[derive(Clone, Props, PartialEq)]
pub struct MessageProps {
    pub greeting: String,
    pub name: String,
    pub line1: String,
    pub line2: Option<String>,
    pub line3: Option<String>,
}

#[component]
fn Message(props: MessageProps) -> Element {
    let lines = [
        Some(format!("{} {}", props.greeting, props.name)),
        Some(props.line1),
        props.line2,
        props.line3,
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
