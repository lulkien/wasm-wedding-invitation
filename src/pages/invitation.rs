use dioxus::prelude::*;
use dioxus_bulma::components::Title;

use crate::{
    components::{hero, main_content, navigation_bar},
    database::{self, Person},
};

#[component]
pub(super) fn Invitation(uid: String) -> Element {
    let mut get_user_data = use_signal(|| Option::<Person>::None);
    let mut loading = use_signal(|| true);
    let mut not_found = use_signal(|| false);

    use_effect(move || {
        let uid = uid.clone();
        spawn(async move {
            match load_data(uid).await {
                Ok(Some(person)) => {
                    get_user_data.set(Some(person));
                    loading.set(false);
                }
                Ok(None) => {
                    not_found.set(true);
                    loading.set(false);
                }
                Err(e) => {
                    error!("Failed to load user: {e}");
                    loading.set(false);
                }
            }
        });
    });

    rsx! {
        div {
            class: "invitation-page",
            hero::Hero {  }
            navigation_bar::NavigationBar {  }
            main_content::MainContent { get_user_data }

            if loading() {
                LoadingOverlay {  }
            }

            if not_found() {
                NotFoundOverlay {  }
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
                class: "message",
                "Invitation is loading..."
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
    Ok(database::query_user(uid.as_str()))
}
