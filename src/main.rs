#![allow(unused)]

use dioxus::{logger::{self, tracing::Level}, prelude::*};

use crate::pages::PageLoader;

mod components;
mod config;
mod database;
mod pages;

fn main() {
    logger::init(Level::INFO).expect("Can't init dioxus logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        PageLoader {}
    }
}
