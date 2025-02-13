use crate::app::models::{AddPersonRequest, Person};
use crate::app::server_functions::persons::add_person;
use leptos::*;
use validator::Validate;


#[component]
pub fn AddPersonModal(
    set_if_show_modal: WriteSignal<bool>
) -> impl IntoView {
    const INPUT_STYLE: &str = "w-full h-23 bg-[#333333] pr-4 pl-6 py-4 text-white mt-6 outline-none focus:outline-none focus:pl-7 transition-all duration-1000 ease-in-out";

    const CANCEL_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";

    const ADD_BUTTON_STYLE: &str = "mt-10 bg-[#7734e7] px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

    const NO_ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7] px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";

    let (error_message, set_error_message) = create_signal(String::new());
    let (if_error, set_if_error) = create_signal(false);
    


    let (person_name, set_person_name) = create_signal(String::new());
    let (person_title, set_person_title) = create_signal(String::new());
    let (person_level, set_person_level) = create_signal(String::new());
    let (compensation, set_compensation) = create_signal(String::new());





}