use dioxus::prelude::*;

#[component]
pub(super) fn PageNotFound() -> Element {
    rsx! {
        div {
            class: "page-not-found",
            h1 {
                "Nothing here my friend xD"
            }
        }
    }
}
