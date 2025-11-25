use std::collections::HashMap;

use leptos::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tile {
    pub id: &'static str,
    pub description: &'static str,
    pub resources: HashMap<String, String>,
    pub price: f64,
    pub row: u32,
    pub col: u32,
    pub is_owned: RwSignal<bool>,
    pub tile_state: TileState,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            id: "",
            description: "",
            resources: HashMap::new(),
            price: 0.,
            is_owned: RwSignal::new(false),
            row: 0,
            col: 0,
            tile_state: TileState::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TileState {
    pub total_land: RwSignal<u32>,
    pub empty_land: RwSignal<u32>,
    pub buildings: Buildings,
    pub workers: Workers,
    pub housing: Housing,
}

impl TileState {
    pub fn new() -> Self {
        Self {
            total_land: RwSignal::new(500),
            empty_land: RwSignal::new(500),
            buildings: Buildings::new(),
            workers: Workers::new(),
            housing: Housing::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Workers {
    pub basic: RwSignal<u64>,
    pub advanced: RwSignal<u64>,
    pub expert: RwSignal<u64>,
}

impl Workers {
    pub fn new() -> Self {
        Self {
            basic: RwSignal::new(0),
            advanced: RwSignal::new(0),
            expert: RwSignal::new(0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Buildings {
    pub production: Production,
    pub housing: Housing,
    pub roads: RwSignal<HashMap<String, u64>>,
}

impl Buildings {
    pub fn new() -> Self {
        Self {
            production: Production::new(),
            housing: Housing::new(),
            roads: RwSignal::new(HashMap::new()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Production {
    pub warehouse: RwSignal<u64>,
    pub sawmill: RwSignal<u64>,
    pub workshop: RwSignal<u64>,
    pub water_pump: RwSignal<u64>,
}

impl Production {
    pub fn new() -> Self {
        Self {
            warehouse: RwSignal::new(0),
            sawmill: RwSignal::new(0),
            workshop: RwSignal::new(0),
            water_pump: RwSignal::new(0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Housing {
    pub cheap: RwSignal<u64>,
    pub standard: RwSignal<u64>,
    pub fancy: RwSignal<u64>,
}

impl Housing {
    pub fn new() -> Self {
        Self {
            cheap: RwSignal::new(0),
            standard: RwSignal::new(0),
            fancy: RwSignal::new(0),
        }
    }
}
