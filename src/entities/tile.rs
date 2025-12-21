use leptos::prelude::*;

use crate::entities::Inventory;
use crate::entities::{
    Buildings, HousingType, Land, ProductionSlot, ProductionType, WorkerType, Workers,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Tile<'a> {
    pub id: &'a str,
    pub description: &'a str,
    pub resources: &'a [&'a str],
    pub price: f64,
    pub row: u32,
    pub col: u32,
    pub is_owned: RwSignal<bool>,
    pub tile_state: TileState,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TileState {
    pub inventory: RwSignal<Inventory>,
    pub buildings: Buildings,
    pub land: Land,
    pub workers: Workers,
    pub production_queue: RwSignal<&'static [ProductionSlot]>,
}

impl TileState {
    pub fn new() -> Self {
        Self {
            inventory: RwSignal::new(Inventory::new()),
            buildings: Buildings::new(),
            land: Land::new(500),
            workers: Workers::new(),
            production_queue: RwSignal::new(&[]),
        }
    }
}

impl Tile<'static> {
    pub fn new() -> Self {
        Self {
            id: "",
            description: "",
            resources: &[],
            price: 0.,
            is_owned: RwSignal::new(false),
            row: 0,
            col: 0,
            tile_state: TileState::new(),
        }
    }

    pub fn hired_workers(&self, worker_type: WorkerType) -> u64 {
        self.tile_state.workers.get_total(worker_type)
    }

    pub fn workers_can_accommodate(&self, worker_type: WorkerType) -> u64 {
        self.tile_state.buildings.get_capacity(worker_type)
    }

    pub fn hire_workers(
        &self,
        worker_type: WorkerType,
        money: RwSignal<f64>,
        amount: u64,
    ) -> Result<(), String> {
        let cost = worker_type.details().cost;
        let total_cost = cost * amount as f64;

        if self.workers_can_accommodate(worker_type) <= amount {
            return Err(format!(
                "Cannot hire {} workers, only space for {}.",
                amount,
                self.workers_can_accommodate(worker_type)
            ));
        }

        if money.get() < total_cost {
            return Err(format!("Insufficient funds. Need ${}.", total_cost));
        }

        money.update(|m| *m -= total_cost);
        self.tile_state.workers.hire(worker_type, amount);
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

    pub fn build_housing(
        &self,
        housing_type: HousingType,
        money: RwSignal<f64>,
        amount: u64,
    ) -> Result<(), String> {
        let details = housing_type.details();
        let total_cost = details.cost * amount as f64;
        let total_land = details.land_used * amount;

        if money.get() < total_cost {
            return Err(format!("Insufficient funds. Need ${}.", total_cost));
        }

        self.tile_state.land.use_land(total_land)?;

        money.update(|m| *m -= total_cost);
        self.tile_state
            .buildings
            .housing
            .build(housing_type, amount);
        Ok(())
    }

    pub fn destroy_housing(&self, housing_type: HousingType, amount: u64) -> Result<(), String> {
        let details = housing_type.details();
        let total_land = details.land_used * amount;

        // Should only fail on bad implementation, not in client, surely
        self.tile_state.land.free_land(total_land)?;
        self.tile_state
            .buildings
            .housing
            .destroy(housing_type, amount)?;
        Ok(())
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

    // pub fn get_production(&self) -> impl Iterator {
    //     self.tile_state.buildings.production.get_all()
    // }

    pub fn build_production(
        &self,
        production_type: ProductionType,
        money: RwSignal<f64>,
        amount: u64,
    ) -> Result<(), String> {
        let details = production_type.details();
        let total_cost = details.cost * amount as f64;
        let total_land = details.land * amount;

        if money.get() < total_cost {
            return Err(format!("Insufficient funds. Need ${:.2}.", total_cost));
        }

        if self.tile_state.land.available.get() < total_land {
            return Err(format!(
                "Insufficient land. Need {}, have {}.",
                total_land,
                self.tile_state.land.available.get()
            ));
        }

        self.tile_state.workers.check_assign(&details.workers)?;

        self.tile_state.land.use_land(total_land).unwrap();

        self.tile_state
            .buildings
            .production
            .build(production_type, amount);
        money.update(|m| *m -= total_cost);
        Ok(())
    }
}
