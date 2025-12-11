use crate::entities::{Housing, HousingType, Production, WorkerType};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Buildings {
    pub production: Production,
    pub housing: Housing,
}

impl Buildings {
    pub fn new() -> Self {
        Self {
            production: Production::new(),
            housing: Housing::new(),
        }
    }

    pub fn get_capacity(&self, worker_type: WorkerType) -> u64 {
        match worker_type {
            WorkerType::Basic => {
                self.housing.cheap.get() * HousingType::Cheap.details().accomodates.0
            }
            WorkerType::Advanced => {
                self.housing.standard.get() * HousingType::Standard.details().accomodates.0
            }
            WorkerType::Expert => {
                self.housing.fancy.get() * HousingType::Fancy.details().accomodates.0
            }
        }
    }
}
