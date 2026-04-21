use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <div class="logo">
                <span class="logo-1">"Aeon"</span>
                <span class="logo-2">"Toolzard"</span>
            </div>

            <div class="tool-info">
                <p>Glass Finder</p>
            </div>
        </header>
    }
}