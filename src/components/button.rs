use leptos::prelude::*;

#[derive(Clone, Copy)]
pub enum ButtonVariant {
    Green,
    Red,
}

#[component]
pub fn Button<F>(
    #[prop(default=ButtonVariant::Green)] variant: ButtonVariant,
    #[prop(optional)] disabled: Signal<bool>,
    children: Children,
    on_click: F,
) -> impl IntoView
where
    F: Fn() + Sync + Send + 'static,
{
    let base = "px-3 py-1 text-white text-lg rounded-md"; //transition-colors
    let variant = match variant {
        ButtonVariant::Green => "bg-green-600 hover:bg-green-500",
        ButtonVariant::Red => {
            "bg-red-600 hover:bg-red-500 disabled:bg-gray-700 disabled:text-gray-400"
        }
    };
    let class = format!("{base} {variant}");

    view! {
        <button class=class disabled=disabled on:click=move |_| on_click()>
            {children()}
        </button>
    }
}
