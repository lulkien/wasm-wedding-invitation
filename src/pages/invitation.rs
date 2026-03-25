use dioxus::prelude::*;
use dioxus::document::eval;

use crate::{
    components::{hero, main_content, navigation_bar},
    database::{self, Person},
};

const FLOWER_IMAGE: Asset = asset!("/assets/favicon.webp");

// Critical images that must be fully loaded before the overlay is dismissed
const HERO_BG: Asset = asset!("/assets/hero-bg.jpg");
const FLORAL_BORDER: Asset = asset!("/assets/floral-border.webp");
const DIVIDER_LEAVES: Asset = asset!("/assets/divider-leaves.webp");
const SECTION_END: Asset = asset!("/assets/divider-flowers-leaves.webp");

#[component]
pub(super) fn Invitation(uid: String) -> Element {
    let mut get_user_data = use_signal(|| Option::<Person>::None);
    let mut loading = use_signal(|| true);
    let mut not_found = use_signal(|| false);

    use_effect(move || {
        let uid = uid.clone();

        // Build the JS preloader string using the resolved asset URLs.
        // asset! returns the correct (possibly fingerprinted) URL for each file.
        let preload_script = format!(
            r#"
            await Promise.all(["{bg}", "{floral}", "{divider}", "{section_end}"].map(src =>
                new Promise(r => {{
                    const i = new Image();
                    i.onload = i.onerror = r;
                    i.src = src;
                }})
            ));
            dioxus.send(true);
            "#,
            bg = HERO_BG,
            floral = FLORAL_BORDER,
            divider = DIVIDER_LEAVES,
            section_end = SECTION_END,
        );

        // Kick off image preloading immediately — it runs in the browser
        // concurrently with the server data fetch below.
        let mut img_eval = eval(&preload_script);

        spawn(async move {
            // 1. Fetch personalised guest data from the server
            match load_data(uid).await {
                Ok(Some(person)) => {
                    get_user_data.set(Some(person));
                }
                Ok(None) => {
                    not_found.set(true);
                    loading.set(false);
                    return;
                }
                Err(e) => {
                    error!("Failed to load user: {e}");
                    loading.set(false);
                    return;
                }
            }

            // 2. Wait for all critical images to finish loading.
            //    If they already finished while the fetch was in flight this
            //    resolves immediately; otherwise we wait the remaining time.
            let _ = img_eval.recv::<bool>().await;

            loading.set(false);
        });
    });

    rsx! {
        div {
            class: "invitation-page",
            hero::Hero {}
            navigation_bar::NavigationBar {}
            main_content::MainContent { get_user_data }

            if loading() {
                LoadingOverlay {}
            }

            if not_found() {
                NotFoundOverlay {}
            }
        }
    }
}

#[component]
fn LoadingOverlay() -> Element {
    rsx! {
        div {
            class: "loading-overlay",
            div {
                class: "loading-flower",
                img {
                    class: "flower-spin",
                    src: FLOWER_IMAGE,
                    alt: "flower"
                }
            }
            p {
                class: "loading-text",
                "Loading..."
            }
        }
    }
}

#[component]
fn NotFoundOverlay() -> Element {
    rsx! {
        div {
            class: "not-found-overlay",
            div {
                class: "message",
                "Nothing here my friend xD"
            }
        }
    }
}

#[post("/api/getUserData")]
async fn load_data(uid: String) -> Result<Option<Person>, ServerFnError> {
    Ok(database::query_user(uid.as_str()).await)
}
