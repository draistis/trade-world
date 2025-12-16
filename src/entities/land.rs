use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Land {
    pub total: RwSignal<u64>,
    pub available: RwSignal<u64>,
}

impl Land {
    pub fn new(total: u64) -> Self {
        Self {
            total: RwSignal::new(total),
            available: RwSignal::new(500),
        }
    }

    pub fn used(&self) -> u64 {
        self.total.get() - self.available.get()
    }

    pub fn use_land(&self, amount: u64) -> Result<(), String> {
        if self.available.get() >= amount {
            self.available.update(|l| *l -= amount);
            return Ok(());
        }
        Err(format!(
            "Not enough land. Need: {}, available: {}.",
            amount,
            self.available.get()
        ))
    }

    pub fn free_land(&self, amount: u64) -> Result<(), String> {
        if self.used() >= amount {
            self.available.update(|l| *l += amount);
            return Ok(());
        }
        Err(format!(
            "Trying to free more land than is used. Used: {}, freeing: {}.",
            self.used(),
            amount
        ))
    }
}
