use chrono::{DateTime, Duration, Utc};
use dioxus::prelude::*;
use dioxus_bulma::components::Container;
use gloo_timers::callback::Interval;

const BG_IMAGE: Asset = asset!("/assets/hero-bg.jpg");

// Ceremony: March 28, 2026 1:30 PM Vietnam time (UTC+7) = 06:30 UTC
const CEREMONY_UTC: &str = "2026-03-28T06:30:00Z";

#[component]
pub fn Hero() -> Element {
    let countdown = use_signal(|| compute_countdown());

    use_effect(move || {
        let mut countdown = countdown;
        Interval::new(1000, move || {
            countdown.set(compute_countdown());
        })
        .forget();
    });

    let (days, hours, mins, secs) = countdown();

    rsx! {
        section {
            id: "home",
            class: "hero is-large",
            background_image: "url({BG_IMAGE})",
            div {
                class: "hero-body",
                Container {
                    class: "has-text-centered",
                    Subtitle {  }
                    Title {  }
                    DateAndTime {  }
                    Countdown {
                        days: days,
                        hours: hours,
                        mins: mins,
                        secs: secs,
                    }
                }
            }
        }
    }
}

fn compute_countdown() -> (u32, u32, u32, u32) {
    let target: DateTime<Utc> = CEREMONY_UTC.parse().unwrap_or_default();
    let now = Utc::now();
    let duration = target.signed_duration_since(now);

    if duration <= Duration::zero() {
        return (0, 0, 0, 0);
    }

    let total_secs = duration.num_seconds() as u64;
    let days = (total_secs / 86400) as u32;
    let hours = ((total_secs % 86400) / 3600) as u32;
    let mins = ((total_secs % 3600) / 60) as u32;
    let secs = (total_secs % 60) as u32;

    (days, hours, mins, secs)
}

#[component]
fn Countdown(days: u32, hours: u32, mins: u32, secs: u32) -> Element {
    rsx! {
        div {
            class: "countdown",
            ul {
                class: "countdown__list",
                li {
                    span { class: "countdown__number", "{days:02}" }
                    p { class: "countdown__label", "Days" }
                }
                li { class: "countdown__separator", ":" }
                li {
                    span { class: "countdown__number", "{hours:02}" }
                    p { class: "countdown__label", "Hours" }
                }
                li { class: "countdown__separator", ":" }
                li {
                    span { class: "countdown__number", "{mins:02}" }
                    p { class: "countdown__label", "Minutes" }
                }
                li { class: "countdown__separator", ":" }
                li {
                    span { class: "countdown__number", "{secs:02}" }
                    p { class: "countdown__label", "Seconds" }
                }
            }
        }
    }
}

#[component]
fn Subtitle() -> Element {
    rsx! {
        h1 {
            class: "subtitle subtitle--lead",
            "Wedding Invitation"
        }
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        h2 {
            class: "title",
            "Hoang Kien & Pham Hang"
        }
    }
}

#[component]
fn DateAndTime() -> Element {
    rsx! {
        h4 {
            class: "subtitle",
            "Saturday, March 28, 2026"
            br {  }
            "Trieu Tien, Son Nam Ward"
            br {  }
            "Hung Yen Province"
        }
    }
}
