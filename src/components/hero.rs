use chrono::{DateTime, Duration, Utc};
use dioxus::prelude::*;
use gloo_timers::callback::Interval;

use crate::config::wedding_config;

const BG_IMAGE: Asset = asset!("/assets/hero-bg.jpg");

#[component]
pub fn Hero() -> Element {
    let config = wedding_config();
    let countdown = use_signal(|| compute_countdown(&config.ceremony.date_utc));

    let date_utc = config.ceremony.date_utc.clone();
    use_effect(move || {
        let mut countdown = countdown;
        let date_utc = date_utc.clone();
        Interval::new(1000, move || {
            countdown.set(compute_countdown(&date_utc));
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
                div {
                    class: "container has-text-centered",
                    Subtitle {  }
                    Title { title: config.couple.title.clone() }
                    DateAndTime {
                        date_display: config.ceremony.date_display.clone(),
                        location_line: config.venue.location_line.clone(),
                        province: config.venue.province.clone(),
                    }
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

fn compute_countdown(date_utc: &str) -> (u32, u32, u32, u32) {
    let target: DateTime<Utc> = date_utc.parse().unwrap_or_default();
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
fn Subtitle() -> Element {
    rsx! {
        h1 {
            class: "subtitle subtitle-lead",
            "Wedding Invitation"
        }
    }
}

#[component]
fn Title(title: String) -> Element {
    rsx! {
        h2 {
            class: "title",
            "{title}"
        }
    }
}

#[component]
fn DateAndTime(date_display: String, location_line: String, province: String) -> Element {
    rsx! {
        h4 {
            class: "subtitle",
            "{date_display}"
            br {  }
            "{location_line}"
            br {  }
            "{province}"
        }
    }
}

#[component]
fn Countdown(days: u32, hours: u32, mins: u32, secs: u32) -> Element {
    rsx! {
        div {
            class: "countdown-wrapper",
            nav {
                class: "level countdown",
                CountdownItem { value: days, label: "Days" }
                CountdownSeparator {}
                CountdownItem { value: hours, label: "Hours" }
                CountdownSeparator {}
                CountdownItem { value: mins, label: "Minutes" }
                CountdownSeparator {}
                CountdownItem { value: secs, label: "Seconds" }
            }
        }
    }
}

#[component]
fn CountdownItem(value: u32, label: &'static str) -> Element {
    rsx! {
        div {
            class: "level-item has-text-centered",
            div {
                p { class: "title", "{value:02}" }
                p { class: "heading", "{label}" }
            }
        }
    }
}

#[component]
fn CountdownSeparator() -> Element {
    rsx! {
        div {
            class: "level-item",
            p { class: "title separator", ":" }
        }
    }
}
