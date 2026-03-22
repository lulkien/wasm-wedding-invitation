use dioxus::prelude::*;
use dioxus_bulma::components::Title;

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

