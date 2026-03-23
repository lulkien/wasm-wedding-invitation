use dioxus::prelude::*;

use crate::{
    components::common::{SectionTitle, Spacing},
    database::{update_location, DepartLocation, Person},
};

const FPT_QR: Asset = asset!("/assets/fpt.webp");
const LOTTE_QR: Asset = asset!("/assets/lotte.webp");

#[component]
pub(super) fn RsvpSection(get_user_data: Signal<Option<Person>>) -> Element {
    let mut dropdown_active = use_signal(|| false);
    let mut select_location =
        use_signal(|| get_user_data().map(|u| u.depart_from).unwrap_or_default());
    let mut saved_location =
        use_signal(|| get_user_data().map(|u| u.depart_from).unwrap_or_default());

    use_effect(move || {
        if let Some(user) = get_user_data() {
            select_location.set(user.depart_from);
            saved_location.set(user.depart_from);
        }
    });

    rsx! {
        section {
            id: "rsvp",
            class: "section-default rsvp-section has-text-centered has-vertically-aligned-content",
            div {
                class: "container",
                SectionTitle { name: "RSVP" }
                Message {}
                ConfirmationDropdown { dropdown_active, select_location }
                ConfirmButton { select_location, saved_location, get_user_data }
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
            br {}
            br {}
            "Shuttles will depart from FPT Tower and Lotte Mall West Lake."
            br {}
            br {}
            "Please let us know if you can attend; we look forward to celebrating!"
            br {}
            br {}
            "If you can't make it, we'll miss you and hope to see you soon."
        }
    }
}

#[component]
fn ConfirmationDropdown(
    dropdown_active: Signal<bool>,
    select_location: Signal<DepartLocation>,
) -> Element {
    let dropdown_class = if dropdown_active() {
        "dropdown is-active location-selector"
    } else {
        "dropdown location-selector"
    };

    let label = match select_location() {
        DepartLocation::None => "I want to depart from...",
        DepartLocation::Fpt => "I want to depart from FPT Tower",
        DepartLocation::Handico => "I want to depart from Handico Tower",
        DepartLocation::Lotte => "I want to depart from Lotte Mall West Lake",
        DepartLocation::MyVehicle => "I will use my own vehicle",
        DepartLocation::Nah => "I can't make it",
    };

    rsx! {
        div {
            class: "{dropdown_class}",
            div {
                class: "dropdown-trigger",
                button {
                    class: "button dropdown-button",
                    onclick: move |_| dropdown_active.set(!dropdown_active()),
                    "{label}"
                    span {
                        class: "icon is-small",
                        if dropdown_active() { "▲" } else { "▼" }
                    }
                }
            }
            div {
                class: "dropdown-menu",
                div {
                    class: "dropdown-content",
                    a {
                        class: "dropdown-item",
                        onclick: move |_| {
                            dropdown_active.set(false);
                            select_location.set(DepartLocation::Fpt);
                        },
                        "FPT Tower"
                    }
                    a {
                        class: "dropdown-item",
                        onclick: move |_| {
                            dropdown_active.set(false);
                            select_location.set(DepartLocation::Handico);
                        },
                        "Handico Tower"
                    }
                    a {
                        class: "dropdown-item",
                        onclick: move |_| {
                            dropdown_active.set(false);
                            select_location.set(DepartLocation::Lotte);
                        },
                        "Lotte Mall West Lake"
                    }
                    a {
                        class: "dropdown-item",
                        onclick: move |_| {
                            dropdown_active.set(false);
                            select_location.set(DepartLocation::MyVehicle);
                        },
                        "I will use my own vehicle"
                    }
                    a {
                        class: "dropdown-item",
                        onclick: move |_| {
                            dropdown_active.set(false);
                            select_location.set(DepartLocation::Nah);
                        },
                        "I can't make it"
                    }
                }
            }
        }
    }
}

#[component]
fn Contact(select_location: Signal<DepartLocation>) -> Element {
    rsx! {
        div {
            match select_location() {
                DepartLocation::None => rsx! {},
                DepartLocation::Fpt => rsx! {
                    ZaloGroup { src: FPT_QR, url: "https://zalo.me/g/en42usc62wqiernfxxcn".to_string() }
                },
                DepartLocation::Handico => rsx! {
                    ZaloGroup { src: FPT_QR, url: "https://zalo.me/g/en42usc62wqiernfxxcn".to_string() }
                },
                DepartLocation::Lotte => rsx! {
                    ZaloGroup { src: LOTTE_QR, url: "https://zalo.me/g/wxrlhr776".to_string() }
                },
                DepartLocation::MyVehicle => rsx! {
                    p {
                        class: "rsvp-contact-message",
                        "Awesome, can't wait to celebrate with you!"
                    }
                },
                DepartLocation::Nah => rsx! {
                    p {
                        class: "rsvp-contact-message",
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
            class: "zalo-instruction",
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

#[post("/api/updateLocation")]
pub async fn update_location_api(
    uid: String,
    location: DepartLocation,
) -> Result<(), ServerFnError> {
    update_location(uid.as_str(), location).inspect_err(|e| warn!("Update location error: {e}"));
    Ok(())
}

#[component]
fn ConfirmButton(
    select_location: Signal<DepartLocation>,
    mut saved_location: Signal<DepartLocation>,
    get_user_data: Signal<Option<Person>>,
) -> Element {
    let is_none = matches!(select_location(), DepartLocation::None);
    let is_saved = select_location() == saved_location();

    if is_none || is_saved {
        return rsx! { div {} };
    }

    let onclick = move |_| {
        let uid = get_user_data().map(|u| u.uid).unwrap_or_default();
        let location = select_location();
        spawn(async move {
            match update_location_api(uid, location).await {
                Ok(_) => {
                    info!("Location updated");
                    saved_location.set(location);
                }
                Err(e) => error!("Failed to update location: {e}"),
            }
        });
    };

    rsx! {
        button {
            class: "button confirm-button",
            onclick,
            "Confirm"
        }
    }
}
