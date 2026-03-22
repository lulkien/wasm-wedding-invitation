#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::{components::ColumnSize, prelude::*};

use crate::components::common::{SectionTitle, Spacing};
use crate::config::wedding_config;

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
    let config = wedding_config();
    rsx! {
        Column {
            class: "has-vertically-aligned-content",
            size: ColumnSize::Four,
            div { class: "has-text-centered", "{config.ceremony.day_of_week}" }
            div { class: "day-number has-text-centered", "{config.ceremony.day_number}" }
            div { class: "has-text-centered", "{config.ceremony.month_year}" }
        }
    }
}

#[component]
fn ReceptionTime() -> Element {
    let config = wedding_config();
    rsx! {
        Column {
            class: "has-vertically-aligned-content",
            size: ColumnSize::Four,
            p {
                class: "is-larger has-text-centered",
                "Reception:"
                br {  }
                "{config.ceremony.reception_time}"
            }
        }
    }
}

#[component]
fn CeremonyTime() -> Element {
    let config = wedding_config();
    rsx! {
        Column {
            class: "has-vertically-aligned-content",
            size: ColumnSize::Four,
            p {
                class: "is-larger has-text-centered",
                "Ceremony:"
                br {  }
                "{config.ceremony.ceremony_time}"
            }
        }
    }
}

