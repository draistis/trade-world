use leptos::prelude::*;

use crate::entities::GameState;

#[component]
pub fn Header() -> impl IntoView {
    let context = use_context::<GameState>().expect("couldn't get context");

    view! {
        <nav class="bg-white border-gray-200 px-4 lg:px-6 py-2.5 dark:bg-gray-800">
            <div class="flex flex-wrap justify-between items-center mx-auto max-w-11/12">
                <a href="/" class="flex items-center">
                    <img
                        src="https://flowbite.com/docs/images/logo.svg"
                        class="mr-3 h-6 sm:h-9"
                        alt="Flowbite Logo"
                    />
                    <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">
                        Trade World
                    </span>
                </a>
                <div class="flex items-center order-2 text-white text-xl">
                    {move || format!("{:.2}", context.cash.get())}
                </div>
            </div>
        </nav>
    }
}
