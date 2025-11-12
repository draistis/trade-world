use std::time::Duration;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::button::Button;
use crate::components::button::ButtonVariant;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="antialiased bg-[#0a0a0a] text-[#fafafa]">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/trade-world.css" />

        // sets the document title
        <Title text="Trade World" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("/forestry") view=ForestryPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn ForestryPage() -> impl IntoView {
    let log_price = 20.;

    let cash = RwSignal::new(1000_f64);
    let logs = RwSignal::new(35_f64);
    let harvester_operators = RwSignal::new(0); // probably shouldn't be a signal
    let fell_progress = RwSignal::new(0);

    let inc_logs = move || {
        *fell_progress.write() += 1;
        if fell_progress.get() >= 10 {
            fell_progress.set(0);
            *logs.write() += 1.;
        }
    };
    let rent_forwarder = move || {
        if cash.get() >= 250. {
            *cash.write() -= 250.;
            if logs.get() >= 25. {
                *logs.write() -= 25.;
                *cash.write() += 25. * log_price;
            } else {
                *cash.write() += logs.get() * log_price;
                logs.set(0.);
            }
        }
    };
    let hire_harvester_opr = move || {
        *harvester_operators.write() += 1;
    };
    Effect::new(move |_| {
        let handle = set_interval_with_handle(
            move || {
                if harvester_operators.get() > 0 {
                    let salary_per_tick = 0.1;
                    *cash.write() -= salary_per_tick;
                    *logs.write() += harvester_operators.get() as f64;
                }
            },
            Duration::from_secs(10),
        )
        .ok();
        on_cleanup(move || {
            if let Some(h) = handle {
                h.clear();
            }
        });
    });

    view! {
        <div class="flex flex-col items-center justify-center mt-11 mx-auto gap-4 max-w-3xl px-6 sm:px-6 lg:px-8">
            <div class="text-xl">Cash: ${cash}</div>
            <h4>"Logs (1x = $20): "{logs}</h4>
            <Button variant=ButtonVariant::Green on_click=inc_logs>
                <div>"Fell tree" {fell_progress} "/10"</div>
            </Button>
            <hr class="w-full my-4 border-t border-gray-200" />
            <div class="inline-flex justify-center items-center h-8">
                <div class="pr-10">"Rent Forwarder (125m"<sup>3</sup>"/25 logs) -> Sawmill"</div>
                <Button
                    variant=ButtonVariant::Red
                    on_click=rent_forwarder
                    disabled=(cash.get() < 250.).into()
                >
                    "Pay $250"
                </Button>
            </div>
            <hr class="w-full my-4 border-t border-gray-200" />
            <div class="inline-flex justify-center items-center h-8">
                <div class="pr-10">"Hire harvester operator ($30/h) [1 tree/min]"</div>
                <Button
                    variant=ButtonVariant::Red
                    on_click=hire_harvester_opr
                    disabled=Signal::derive(move || cash.get() < 30.)
                >
                    "Hire"
                </Button>
            </div>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
