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
    let mut get_user_data = use_signal(Person::with_default_message);

    use_effect(move || {
        let uid = uid.clone();
        spawn(async move {
            match load_data(uid).await {
                Ok(person) => get_user_data.set(person),
                Err(e) => error!("Failed to load user: {e}"),
            }
        });
    });

    rsx! {
        div {
            background_color: "ghostwhite",
            home::HeaderWrapper {  }
            main_content::MainContent { get_user_data }
        }
    }
}

#[post("/api/getUserData")]
async fn load_data(uid: String) -> Result<Person, ServerFnError> {
    let person = database::query_user(uid.as_str());
    Ok(person)
}
