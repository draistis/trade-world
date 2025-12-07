use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col m-auto w-screen h-screen content-center justify-center items-center text-4xl text-white/80">
            <h1 class="flex flex-1 m-auto">"Welcome to Trade World!"</h1>
            <div class="flex flex-1 flex-col">
                <a href="/tile-map">"Tile map"</a>
                <a class="text-gray-400" href="/forestry">
                    "Forestry"
                </a>
            </div>
        </div>
    }
}
