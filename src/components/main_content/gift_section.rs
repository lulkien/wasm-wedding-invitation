use dioxus::document::eval;
use dioxus::prelude::*;

use crate::{components::common::SectionTitle, config::wedding_config};

const VCB_QR: Asset = asset!("/assets/VCB.webp");

#[component]
pub(super) fn GiftSection() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div {
            class: "gift-section-wrapper",

            // ── Toggle button ─────────────────────────────────────────────────
            div {
                class: "gift-toggle",
                button {
                    class: "gift-toggle__btn",
                    title: if visible() { "Hide gift info" } else { "Show gift info" },
                    onclick: move |_| {
                        let opening = !visible();
                        visible.set(opening);
                        // requestAnimationFrame ensures the browser has committed
                        // the DOM change before we ask it to scroll.
                        if opening {
                            let _ = eval("requestAnimationFrame(() => document.getElementById('gift')?.scrollIntoView({ behavior: 'smooth', block: 'start' }));");
                        } else {
                            let _ = eval("requestAnimationFrame(() => document.querySelector('.gift-toggle')?.scrollIntoView({ behavior: 'smooth', block: 'center' }));");
                        }
                    },
                    if visible() { "▲" } else { "▼" }
                }
            }

            // ── Gift section (revealed on toggle) ─────────────────────────────
            if visible() {
                section {
                    id: "gift",
                    class: "section-default gift-section has-text-centered",
                    div {
                        class: "container",
                        SectionTitle { name: "Wedding Gift" }
                        GiftMessage {}
                        GiftQrCode {}
                    }
                }
            }
        }
    }
}

#[component]
fn GiftMessage() -> Element {
    let config = wedding_config();
    rsx! {
        p {
            class: "gift-message",
            "{config.gift.message}"
        }
    }
}

#[component]
fn GiftQrCode() -> Element {
    let config = wedding_config();
    rsx! {
        img {
            class: "qr-code",
            src: VCB_QR,
            alt: "Bank transfer QR code"
        }
        p {
            class: "gift-account-info",
            strong { "{config.gift.bank}" }
            br {}
            "{config.gift.account_number}"
            br {}
            "{config.gift.account_holder}"
        }
    }
}
