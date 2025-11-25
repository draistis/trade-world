use leptos::prelude::*;

#[derive(Clone)]
struct TabValue(RwSignal<String>);

#[component]
pub fn Tabs(default_value: &'static str, children: Children) -> impl IntoView {
    provide_context::<TabValue>(TabValue(RwSignal::new(default_value.to_string())));

    view! { <div class="w-full flex flex-col">{children()}</div> }
}

#[component]
pub fn TabsList(children: Children) -> impl IntoView {
    view! { <div class="inline-flex items-center border-b border-gray-700">{children()}</div> }
}

#[component]
pub fn TabsTrigger(value: &'static str, children: Children) -> impl IntoView {
    let tab_value =
        use_context::<TabValue>().expect("TabsTrigger must be used inside of a Tabs component");
    let is_active = move || tab_value.0.get() == value;

    let on_click = move |_| {
        tab_value.0.set(value.to_string());
    };

    view! {
        <button
            on:click=on_click
            class="px-4 py-2 -mb-px transition-colors cursor-pointer"
            class=(["border-b-2", "border-white", "text-white"], is_active)
            class=(["text-gray", "hover:text-gray-200"], move || !is_active())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContent(value: &'static str, children: Children) -> impl IntoView {
    let tab_value =
        use_context::<TabValue>().expect("TabsContent must be used inside of a Tabs component");
    let is_visible = move || tab_value.0.get() == value;

    view! {
        <div class="p-2" class:hidden=move || !is_visible()>
            {children()}
        </div>
    }
}
