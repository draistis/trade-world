use crate::components::header::Header;
use leptos::prelude::*;
use leptos::Params;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::components::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionType,
};
use crate::components::{Tabs, TabsContent, TabsList, TabsTrigger};

#[derive(Params, PartialEq)]
pub struct TileParams {
    pub id: Option<String>,
}

#[component]
pub fn TilePage() -> impl IntoView {
    let params = use_params::<TileParams>();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
            .unwrap_or_default()
    };

    view! {
        <div class="flex flex-col h-screen overflow-hidden">
            <Header />
            <div class="flex flex-1">
                <div class="flex border border-red-500 w-1/2">
                    <Tabs default_value="overview">
                        <TabsList>
                            <TabsTrigger value="overview">Overview</TabsTrigger>
                            <TabsTrigger value="buildings">Buildings</TabsTrigger>
                            <TabsTrigger value="workers">Workers</TabsTrigger>
                        </TabsList>
                        <TabsContent value="overview">"overview"</TabsContent>
                        <TabsContent value="buildings">
                            <BuildingsTab />
                        </TabsContent>
                        <TabsContent value="workers">"workers tab content"</TabsContent>
                    </Tabs>
                </div>
                <div class="flex flex-1 border border-green-500">"alekumisalami"</div>
            </div>
        </div>
    }
}

#[component]
pub fn BuildingsTab() -> impl IntoView {
    view! {
        <Accordion of_type=AccordionType::Multiple collapsible=true>
            <AccordionItem value="production-buildings">
                <AccordionTrigger>"Production Buildings"</AccordionTrigger>
                <AccordionContent>"Produce stuff here"</AccordionContent>
            </AccordionItem>
            <AccordionItem value="worker-housing">
                <AccordionTrigger>"Worker Housing"</AccordionTrigger>
                <AccordionContent>"something hoses stuff"</AccordionContent>
            </AccordionItem>
            <AccordionItem value="hire-workers">
                <AccordionTrigger>"Hire Workers"</AccordionTrigger>
                <AccordionContent>"Hire people here"</AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
