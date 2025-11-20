use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccordionType {
    Single,
    Multiple,
}

#[derive(Clone, Copy)]
struct AccordionCtx {
    acc_type: AccordionType,
    collapsible: bool,
    open_items: RwSignal<Vec<String>>,
}

impl AccordionCtx {
    fn is_open(&self, value: &str) -> bool {
        self.open_items.get().contains(&value.to_string())
    }
}

#[derive(Clone)]
struct AccordionValue(&'static str);

#[component]
pub fn Accordion(
    #[prop(default=AccordionType::Single)] of_type: AccordionType,
    #[prop(default = false)] collapsible: bool,
    children: Children,
) -> impl IntoView {
    provide_context::<AccordionCtx>(AccordionCtx {
        acc_type: of_type,
        collapsible,
        open_items: RwSignal::new(Vec::new()),
    });

    view! { <div class="w-full">{children()}</div> }
}

#[component]
pub fn AccordionItem(value: &'static str, children: Children) -> impl IntoView {
    provide_context::<AccordionValue>(AccordionValue(value));

    view! { <div class="border-b border-gray-700">{children()}</div> }
}

#[component]
pub fn AccordionTrigger(children: Children) -> impl IntoView {
    let ctx = use_context::<AccordionCtx>()
        .expect("AccordionTrigger must be used inside of an Accordion");
    let value = use_context::<AccordionValue>()
        .expect("AccordionTrigger must be used inside of an AccordionItem")
        .0;

    let on_click = move |_| {
        let open_items = ctx.open_items;
        if !ctx.is_open(value) {
            open_items.write().push(value.to_string());
            if ctx.acc_type == AccordionType::Single && open_items.get().len() > 1 {
                open_items.write().remove(0);
            }
        } else if ctx.collapsible {
            let idx = open_items.get().iter().position(|v| *v == value);
            if let Some(idx) = idx {
                open_items.write().remove(idx);
            }
        }
    };

    let is_open = move || ctx.is_open(value);

    view! {
        <button
            class="flex w-full items-center justify-between py-4 text-left font-medium transition-all hover:underline hover:cursor-pointer"
            on:click=on_click
        >
            {children()}
            <svg
                class="h-4 w-4 shrink-0 transition-transform duration-200"
                class:rotate-180=is_open
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M19 9l-7 7-7-7"
                />
            </svg>
        </button>
    }
}

#[component]
pub fn AccordionContent(children: Children) -> impl IntoView {
    let ctx = use_context::<AccordionCtx>()
        .expect("AccordionContent must be used inside of an Accordion");
    let value = use_context::<AccordionValue>()
        .expect("AccordionTrigger must be used inside of an AccordionItem")
        .0;

    let is_hidden = move || !ctx.is_open(value);

    view! {
        <div
            class="overflow-hidden transition-all duration-200"
            class:max-h-0=is_hidden
            class:max-h-96=move || !is_hidden()
        >
            <div class="pb-4 pt-0 text-gray-400">{children()}</div>
        </div>
    }
}
