use dioxus::prelude::*;
use dioxus_bulma::components::Title;

use crate::{
    components::{home, main_content},
    database::{self, Person},
};

#[component]
pub(super) fn PageNotFound() -> Element {
    rsx! {
        div {
            class: "page-not-found",
            Title {
                "Nothing here my friend xD"
            }
        }
    }
}

