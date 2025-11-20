use std::time::Duration;

use leptos::prelude::*;

use crate::{
    components::{
        button::{Button, ButtonVariant},
        header::Header,
    },
    entities::GameState,
};

#[component]
pub fn ForestryPage() -> impl IntoView {
    let game_state = use_context::<GameState>().expect("couldn't get context");
    let log_price = 20.;

    let fell_progress = RwSignal::new(0);

    let inc_logs = move || {
        *fell_progress.write() += 1;
        if fell_progress.get() >= 10 {
            fell_progress.set(0);
            *game_state.logs.write() += 1;
        }
    };
    let rent_forwarder = move || {
        if game_state.cash.get() >= 250. {
            *game_state.cash.write() -= 250.;
            if game_state.logs.get() >= 25 {
                *game_state.logs.write() -= 25;
                *game_state.cash.write() += 25. * log_price;
            } else {
                *game_state.cash.write() += game_state.logs.get() as f64 * log_price;
                game_state.logs.set(0);
            }
        }
    };
    let hire_harvester_opr = move || {
        // *game_state.tiles.get().get(0)..advanced.write() += 1;
    };

    Effect::new(move |_| {
        set_interval(
            move || {
                // if game_state.workers.advanced.get() > 0 {
                //     *game_state.cash.write() -= 1. * game_state.workers.advanced.get() as f64;
                // }
            },
            Duration::from_secs(60),
        );
        let handle = set_interval_with_handle(
            move || {
                // if game_state.workers.advanced.get() > 0 {
                //     *game_state.logs.write() += game_state.workers.advanced.get();
                // }
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
        <Header />
        <div class="flex flex-col items-center justify-center mt-11 mx-auto gap-4 max-w-3xl px-6 sm:px-6 lg:px-8">
            <h4>"Logs (1x = $20): "{move || game_state.logs.get()}</h4>
            <Button variant=ButtonVariant::Green on_click=inc_logs>
                <div>"Fell tree " {fell_progress} "/10"</div>
            </Button>
            <hr class="w-full my-4 border-t border-gray-200" />
            <div class="inline-flex justify-center items-center h-8">
                <div class="pr-10">"Rent Forwarder (125m"<sup>3</sup>"/25 logs) -> Sawmill"</div>
                <Button
                    variant=ButtonVariant::Red
                    on_click=rent_forwarder
                    disabled=Signal::derive(move || game_state.cash.get() < 250.)
                >
                    "Pay $250"
                </Button>
            </div>
            <hr class="w-full my-4 border-t border-gray-200" />
            <div class="flex gap-8 justify-center items-center h-8">
                <div>"Hire harvester operator ($30/h) [1 tree/min]"</div>
                // <div>"["{move || game_state.workers.advanced.get()}"]"</div>
                <Button
                    variant=ButtonVariant::Red
                    on_click=hire_harvester_opr
                    disabled=Signal::derive(move || game_state.cash.get() < 30.)
                >
                    "Hire"
                </Button>
            </div>
        </div>
    }
}
