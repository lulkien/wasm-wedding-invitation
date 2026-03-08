#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::prelude::*;

use crate::pages::FrontPage;

mod components;
mod pages;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Light,
            load_bulma_css: true,

            FrontPage {  }
        }
    }
}
