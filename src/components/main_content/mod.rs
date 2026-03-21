use crate::database::Person;
use dioxus::prelude::*;
use dioxus_bulma::prelude::*;

mod introduce_section;
mod location_section;
mod rsvp_section;
mod time_section;

#[component]
pub fn MainContent(get_user_data: Signal<Option<Person>>) -> Element {
    rsx! {
        div {
            class: "main-content",
            introduce_section::IntroduceSection { get_user_data }
            time_section::TimeSection {  }
            location_section::LocationSection {  }
            rsvp_section::RsvpSection { get_user_data }
        }
    }
}
