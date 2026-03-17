#![allow(unused)]

use dioxus::prelude::*;
use dioxus_bulma::prelude::*;

use crate::{database::init_database, pages::FrontPage};

mod components;
mod pages;
mod database;

fn main() {
    init_database();
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
