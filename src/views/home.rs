use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="">
            <h1>"Welcome to Trade World!"</h1>
            <div class="">
                <a href="/tile-map">"Tile map"</a>
                <a href="/forestry">"Forestry"</a>
            </div>
        </div>
    }
}
