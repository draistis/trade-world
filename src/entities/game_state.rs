use std::collections::HashMap;

use leptos::prelude::*;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct Buildings {
    pub warehouse: RwSignal<u64>,
    pub sawmill: RwSignal<u64>,
    pub workshop: RwSignal<u64>,
    pub housing: Housing,
    pub roads: RwSignal<HashMap<String, u64>>,
}

impl Buildings {
    pub fn new() -> Self {
        Self {
            warehouse: RwSignal::new(0),
            sawmill: RwSignal::new(0),
            workshop: RwSignal::new(0),
            housing: Housing::new(),
            roads: RwSignal::new(HashMap::new()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Housing {
    pub small: RwSignal<u64>,
    pub medium: RwSignal<u64>,
    pub large: RwSignal<u64>,
}

impl Housing {
    pub fn new() -> Self {
        Self {
            small: RwSignal::new(0),
            medium: RwSignal::new(0),
            large: RwSignal::new(0),
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct GameState {
    pub cash: RwSignal<f64>,
    pub logs: RwSignal<u64>,
    pub workers: Workers,
    pub buildings: Buildings,
    pub inventory: RwSignal<HashMap<String, f64>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            cash: RwSignal::new(1000.),
            logs: RwSignal::new(30),
            workers: Workers::new(),
            buildings: Buildings::new(),
            inventory: RwSignal::new(HashMap::new()),
        }
    }
}
