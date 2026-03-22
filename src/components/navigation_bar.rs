use dioxus::prelude::*;

#[component]
pub fn NavigationBar() -> Element {
    rsx! {
        nav {
            class: "navigation-bar",
            div {
                class: "navigation-bar__wrapper",
                ul {
                    class: "navigation-bar__menu",
                    li { a { href: "#intro", "Home" } }
                    li { a { href: "#time", "Time" } }
                    li { a { href: "#location", "Location" } }
                    li { a { href: "#rsvp", "RSVP" } }
                }
            }
        }
    }
}
