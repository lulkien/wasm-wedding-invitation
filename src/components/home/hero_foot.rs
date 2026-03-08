use dioxus::prelude::*;
use dioxus_bulma::components::{
    ColumnSize, Help, ImageSize, InputType, Label, PaginationAlignment, Subtitle, TabsAlignment,
    TabsStyle, Title, TitleSize,
};
use dioxus_bulma::prelude::*;

#[component]
pub(super) fn HeroFoot() -> Element {
    rsx! {
        div {
            class: "hero-foot",

            HeroFootWrapper {  }
        }
    }
}

#[component]
fn HeroFootWrapper() -> Element {
    rsx! {
        div {
            class: "hero-foot-wrapper",
            Columns {
                Column {
                    class: "hero-menu-desktop has-text-centered",
                    size: ColumnSize::Twelve,
                    NavigationList {  }
                }
            }
        }
    }
}

#[component]
fn NavigationList() -> Element {
    rsx! {
        ul { 
            li { class: "is-active", a { href: "#home", "Home" } }
            li { a { href: "#time", "Time" } }
            li { a { href: "#location", "Location" } }
            li { a { href: "#happy-hour", "Happy Hour" } }
            li { a { href: "#rspv", "RSPV" } }

        }
    }
}
