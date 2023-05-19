use gloo::utils::window;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(Navbar)]
pub fn home() -> Html {
    
    
    let window = web_sys::window()
        .unwrap();
    let document = window
        .document()
        .unwrap();

    // * Links between parts of the page
    // Navbar
    
    let doc = document.clone();
    let go_to_home: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        let about = doc.get_element_by_id("top")
        .expect("Should be an element with id 'top'");
        about.scroll_into_view();
    });
    let doc = document.clone();
    let go_to_about: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        let about = doc.get_element_by_id("about")
        .expect("Should be an element with id 'about'");
        about.scroll_into_view();
    });
    let doc = document.clone();
    let go_to_stylists: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        let about = doc.get_element_by_id("stylists")
        .expect("Should be an element with id 'stylists'");
        about.scroll_into_view();
    });
    let doc = document.clone();
    let go_to_gallery: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        let about = doc.get_element_by_id("gallery")
        .expect("Should be an element with id 'gallery'");
        about.scroll_into_view();
    });
    let doc = document.clone();
    let go_to_contact: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        let about = doc.get_element_by_id("contact")
        .expect("Should be an element with id 'contact'");
        about.scroll_into_view();
    });
    let doc = document.clone();
    let go_to_services: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        let about = doc.get_element_by_id("services")
        .expect("Should be an element with id 'services'");
        about.scroll_into_view();
    });


    html! { 

        
        <div class="navbar">
            <div onclick={go_to_home} class="navbar-link">{"Home"}</div>
            <div onclick={go_to_about} class="navbar-link">{"About"}</div>
            <div onclick={go_to_services} class="navbar-link">{"Services"}</div>
            <div onclick={go_to_stylists} class="navbar-link">{"Stylists"}</div>
            <div onclick={go_to_gallery} class="navbar-link">{"Gallery"}</div>
            <div onclick={go_to_contact} class="navbar-link">{"Contact"}</div>
        </div>
    }
}