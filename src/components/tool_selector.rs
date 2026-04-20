use leptos::ev::keydown;
use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn ToolSelector() -> impl IntoView {
    let show = RwSignal::new(false);
    window_event_listener(keydown, move |ev| {
        let key = ev.key();
        match key.as_str() {
            "f" => show.set(!show.get()),
            _ => {}
        }
    });

    view! {
        <Show when=move || { show.get() }>
            <div class="tool-selector">"I am tool selector"</div>
        </Show>
    }
}
