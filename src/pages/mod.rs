use dioxus::prelude::*;
use dioxus_bulma::components::Title;

use invitation::Invitation;
use page_not_found::PageNotFound;

mod admin;
mod invitation;
mod page_not_found;

const FAVICON: Asset = asset!("/assets/favicon.webp");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn PageLoader() -> Element {
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
