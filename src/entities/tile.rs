use crate::entities::{Buildings, Housing, HousingType, ProductionType, WorkerType, Workers};
use leptos::prelude::*;

use crate::entities::Inventory;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tile {
    pub id: &'static str,
    pub description: &'static str,
    pub resources: Vec<&'static str>,
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
            resources: Vec::new(),
            price: 0.,
            is_owned: RwSignal::new(false),
            row: 0,
            col: 0,
            tile_state: TileState::new(),
        }
    }

    pub fn hired_workers(&self, worker_type: WorkerType) -> u64 {
        self.tile_state.workers.get_workers(worker_type)
    }

    pub fn available_workers(&self, worker_type: WorkerType) -> u64 {
        self.tile_state.buildings.get_capacity(worker_type)
    }

    pub fn hire_worker(&self, worker_type: WorkerType, money: RwSignal<f64>) -> Result<(), String> {
        let details = worker_type.details();

        if self.available_workers(worker_type) <= self.hired_workers(worker_type) {
            return Err("Insufficient housing space.".to_string());
        }

        if money.get() < details.cost {
            return Err(format!("Insufficient funds. Need ${}", details.cost));
        }

        money.update(|m| *m -= details.cost);

        match worker_type {
            WorkerType::Basic => {
                self.tile_state.workers.basic.update(|a| *a += 1);
            }
            WorkerType::Advanced => {
                self.tile_state.workers.advanced.update(|a| *a += 1);
            }
            WorkerType::Expert => {
                self.tile_state.workers.expert.update(|a| *a += 1);
            }
        }

        Ok(())
    }

    pub fn owned_housing(&self, housing_type: HousingType) -> u64 {
        let housing = self.tile_state.buildings.housing;
        match housing_type {
            HousingType::Cheap => housing.cheap.get(),
            HousingType::Standard => housing.standard.get(),
            HousingType::Fancy => housing.fancy.get(),
        }
    }

    pub fn build_housing(&self, housing_type: HousingType, money: RwSignal<f64>) {
        let details = housing_type.details();

        if money.get() < details.cost {
            return;
        }

        money.update(|m| *m -= details.cost);

        match housing_type {
            HousingType::Cheap => {
                self.tile_state.buildings.housing.cheap.update(|a| *a += 1);
            }
            HousingType::Standard => {
                self.tile_state
                    .buildings
                    .housing
                    .standard
                    .update(|a| *a += 1);
            }
            HousingType::Fancy => {
                self.tile_state.buildings.housing.fancy.update(|a| *a += 1);
            }
        }
    }

    pub fn owned_production_buildings(&self, production_type: ProductionType) -> u64 {
        let production = self.tile_state.buildings.production;

        match production_type {
            ProductionType::Warehouse => production.warehouse.get(),
            ProductionType::Sawmill => production.sawmill.get(),
            ProductionType::WaterPump => production.water_pump.get(),
            ProductionType::Workshop => production.workshop.get(),
        }
    }

    pub fn build_production(&self, production_type: ProductionType, money: RwSignal<f64>) {
        let details = production_type.details();

        if money.get() < details.cost {
            return;
        }

        money.update(|m| *m -= details.cost);

        match production_type {
            ProductionType::Sawmill => {
                self.tile_state
                    .buildings
                    .production
                    .sawmill
                    .update(|a| *a += 1);
            }
            ProductionType::Warehouse => {
                self.tile_state
                    .buildings
                    .production
                    .warehouse
                    .update(|a| *a += 1);
            }
            ProductionType::WaterPump => {
                self.tile_state
                    .buildings
                    .production
                    .water_pump
                    .update(|a| *a += 1);
            }
            ProductionType::Workshop => {
                self.tile_state
                    .buildings
                    .production
                    .workshop
                    .update(|a| *a += 1);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TileState {
    pub inventory: RwSignal<Inventory>,
    pub total_land: RwSignal<u32>,
    pub empty_land: RwSignal<u32>,
    pub buildings: Buildings,
    pub workers: Workers,
    pub housing: Housing,
}

impl TileState {
    pub fn new() -> Self {
        Self {
            inventory: RwSignal::new(Inventory::new()),
            total_land: RwSignal::new(500),
            empty_land: RwSignal::new(500),
            buildings: Buildings::new(),
            workers: Workers::new(),
            housing: Housing::new(),
        }
    }
}
