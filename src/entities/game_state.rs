use std::collections::HashMap;

use leptos::prelude::*;

#[derive(Clone, Debug, Copy)]
pub struct GameState {
    pub cash: RwSignal<f64>,
    pub logs: RwSignal<u64>,
    pub inventory: RwSignal<HashMap<String, f64>>,
    pub tiles: RwSignal<Vec<super::tile::Tile>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            cash: RwSignal::new(1000.),
            logs: RwSignal::new(30),
            inventory: RwSignal::new(HashMap::new()),
            tiles: RwSignal::new(Vec::new()),
        }
    }
}
