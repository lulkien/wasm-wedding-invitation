#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::components::Title;

use crate::{
    components::{home, main_content},
    database::{self, Person},
};

const FAVICON: Asset = asset!("/assets/favicon.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn FrontPage() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {  }
    }
}

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    PageNotFound,

    #[route("/invitation/:uid")]
    Invitation { uid: String },
}

#[component]
fn PageNotFound() -> Element {
    rsx! {
        div {
            display: "flex",
            justify_content: "center",
            align_items: "center",
            height: "100vh",

            Title {
                "Nothing here my friend xD"
            }
        }
    }
}

#[component]
pub fn Invitation(uid: String) -> Element {
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
            background_color: "ghostwhite",
            home::HeaderWrapper {  }
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
            style: "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background-color: rgba(255,255,255,0.95); display: flex; justify-content: center; align-items: center; z-index: 9999;",
        }
    }
}

#[component]
fn NotFoundOverlay() -> Element {
    rsx! {
        div {
            style: "position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background-color: ghostwhite; display: flex; justify-content: center; align-items: center; z-index: 9999;",
            div {
                font_size: "3rem",
                text_align: "center",
                color: "black",
                "Nothing here my friend xD"
            }
        }
    }
}

#[post("/api/getUserData")]
async fn load_data(uid: String) -> Result<Option<Person>, ServerFnError> {
    Ok(database::query_user(uid.as_str()))
}
