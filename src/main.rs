#![allow(unused)]

use dioxus::{logger::{self, tracing::Level}, prelude::*};
use dioxus_bulma::prelude::*;

use crate::pages::PageLoader;

mod components;
mod database;
mod pages;

fn main() {
    logger::init(Level::INFO).expect("Can't init dioxus logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        BulmaProvider {
            theme: BulmaTheme::Light,
            load_bulma_css: true,

            PageLoader {  }
        }
    }
}
