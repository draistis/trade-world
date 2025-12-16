use std::collections::HashMap;

use leptos::prelude::*;

use crate::entities::Tile;

#[derive(Clone, Debug)]
pub struct GameState {
    pub cash: RwSignal<f64>,
    pub inventory: RwSignal<HashMap<String, f64>>,
    pub tiles: Vec<RwSignal<Tile>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            cash: RwSignal::new(10000.),
            inventory: RwSignal::new(HashMap::new()),
            tiles: Vec::new(),
        }
    }
}
