#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::{components::ColumnSize, prelude::*};

use crate::components::common::{SectionTitle, Spacing};

const FPT_QR: Asset = asset!("/assets/fpt.webp");
const LOTTE_QR: Asset = asset!("/assets/lotte.webp");

#[component]
pub(super) fn RsvpSection() -> Element {
    let dropdown_active = use_signal(|| false);
    let select_location = use_signal(|| String::from(""));

    rsx! {
        section { 
            id: "rsvp",
            class: "section-default rsvp-section has-text-centered has-vertically-aligned-content",

            Container {
                SectionTitle { name: "RSVP" }
                Message {  }
                ConfirmationDropdown { dropdown_active, select_location }
                Contact { select_location }
            }
        }
    }
}

#[component]
fn Message() -> Element {
    rsx! {
        p {
            class: "message",
            "It would be an honor to have you join us and offer your blessing."
            br { }
            br { }
            "Shuttles will depart from FPT Tower and Lotte Mall West Lake."
            br { }
            br { }
            "Please let us know if you can attend; we look forward to celebrating!"
            br { }
            br { }
            "If you can’t make it, we’ll miss you and hope to see you soon."
        }
    }
}

#[component]
fn ConfirmationDropdown(dropdown_active: Signal<bool>, select_location: Signal<String>) -> Element {
    rsx! {
        Dropdown {
            class: "location-selector",
            active: dropdown_active(),
            DropdownTrigger {
                onclick: move |_| dropdown_active.set(!dropdown_active()),
                Button {
                    class: "dropdown-button",
                    match select_location().as_str() {
                        "fpt" => "I want to depart from FPT Tower/Handico Tower",
                        "lotte"=> "I want to depart from Lotte Mall West Lake",
                        "" => "I want to depart from...",
                        _  => "I can't make it"
                    }
                    span {
                        class: "icon is-small",
                        if dropdown_active() { "▲" } else { "▼" }
                    }
                }
            }
            DropdownMenu {
                style: "width: 100%; min-width: unset;",
                DropdownItem { 
                    onclick: move |_| {
                        dropdown_active.set(false);
                        select_location.set(String::from("fpt"));
                    },
                    "FPT Tower/Handico Tower"
                }
                DropdownItem { 
                    onclick: move |_| {
                        dropdown_active.set(false);
                        select_location.set(String::from("lotte"));
                    },
                    "Lotte Mall West Lake"
                }
                DropdownItem { 
                    onclick: move |_| {
                        dropdown_active.set(false);
                        select_location.set(String::from("nil"));
                    },
                    "I can't make it"
                }
            }
        }
    }
}

#[component]
fn Contact(select_location: Signal<String>) -> Element {
    rsx! {
        div {
            match select_location().as_str() {
                "fpt" => rsx! { ZaloGroup { src: FPT_QR , url: "https://zalo.me/g/en42usc62wqiernfxxcn".to_string() } },
                "lotte" => rsx! { ZaloGroup { src: LOTTE_QR, url: "https://zalo.me/g/wxrlhr776".to_string() } },
                "" => { rsx! {  } }
                _ => rsx! {
                    p {
                        font_size: "1rem",
                        "No worries at all—we'll catch up with you soon!"
                    }
                }
            }
        }
    }
}

#[component]
fn ZaloGroup(src: Asset, url: String) -> Element {
    rsx! {
        img {
            class: "qr-code",
            src: "{src}"
        }
        p {
            font_size: "1rem",
            "If you don't mind, please join this"
            a {
                onclick: move |_| {
                    webbrowser::open(&url);
                },
                " Zalo "
            }
            "group for transport updates and coordination."
        }
    }
}
