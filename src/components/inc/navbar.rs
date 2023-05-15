use yew::prelude::*;


#[function_component(Navbar)]
pub fn home() -> Html {
    html! { 
        

        <div class="navbar">
            <div class="navbar-link">{"Home"}</div>
            <div class="navbar-link">{"About"}</div>
            <div class="navbar-link">{"Services"}</div>
            <div class="navbar-link">{"Stylists"}</div>
            <div class="navbar-link">{"Gallery"}</div>
            <div class="navbar-link">{"Contact"}</div>
    
        </div>
    }
}
