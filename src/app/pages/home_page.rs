use leptos::*;
use crate::app::components::Header;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
      <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
          <Header/>
          <div class="mt-20 ml-20 text-white">"Welcome to Dashboard"</div>
         
      </div>
        
    }
}