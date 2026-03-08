#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::{components::ColumnSize, prelude::*};

use crate::components::common::{SectionTitle, Spacing};

#[component]
pub(super) fn TimeSection() -> Element {
    rsx! {
        section { 
            id: "time",
            class: "section-default time-section has-text-centered has-vertically-aligned-content",

            Container {
                SectionTitle { name: "Time" }
                Calendar {  }
            }
            Spacing { space: "40px" }
        }
    }
}

#[component]
fn Calendar() -> Element {
    rsx! {
        Columns {
            multiline: true,
            DateTimeColumn {  }
            ReceptionTime {  }
            CeremonyTime {  }
        }
    }
}

#[component]
fn DateTimeColumn() -> Element {
    rsx! {
        Column {
            class: "has-vertically-aligned-content",
            size: ColumnSize::Four,
            div { class: "has-text-centered", "Saturday" }
            div { class: "day-number has-text-centered", "28" }
            div { class: "has-text-centered", "March 2026" }
        }
    }
}

#[component]
fn ReceptionTime() -> Element {
    rsx! {
        Column {
            class: "has-vertically-aligned-content",
            size: ColumnSize::Four,
            p {
                class: "is-larger has-text-centered",
                "Reception:"
                br {  }
                "8:30 AM"
            }
        }
    }
}

#[component]
fn CeremonyTime() -> Element {
    rsx! {
        Column {
            class: "has-vertically-aligned-content",
            size: ColumnSize::Four,
            p {
                class: "is-larger has-text-centered",
                "Ceremony:"
                br {  }
                "1:30 PM"
            }
        }
    }
}

