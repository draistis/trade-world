use leptos::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum HousingType {
    Cheap,
    Standard,
    Fancy,
}

impl HousingType {
    pub fn all() -> Vec<Self> {
        vec![Self::Cheap, Self::Standard, Self::Fancy]
    }

    pub fn details(&self) -> HousingDetails {
        match self {
            HousingType::Cheap => HousingDetails {
                name: "Cheap Housing",
                description: "The most basic shack.",
                cost: 500.0,
                accomodates: Accomodation(10, "Basic"),
                land_used: 10,
            },
            HousingType::Standard => HousingDetails {
                name: "Standard Housing",
                description: "Normal housing with some comforts.",
                cost: 750.0,
                accomodates: Accomodation(5, "Advanced"),
                land_used: 20,
            },
            HousingType::Fancy => HousingDetails {
                name: "Fancy Housing",
                description: "A fancy home fit for a king.",
                cost: 1250.0,
                accomodates: Accomodation(3, "Expert"),
                land_used: 30,
            },
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

    pub fn build(&self, housing_type: HousingType, amount: u64) {
        match housing_type {
            HousingType::Cheap => self.cheap.update(|h| *h += amount),
            HousingType::Standard => self.standard.update(|h| *h += amount),
            HousingType::Fancy => self.fancy.update(|h| *h += amount),
        }
    }

    pub fn destroy(&self, housing_type: HousingType, amount: u64) -> Result<(), String> {
        match housing_type {
            HousingType::Cheap => {
                if self.cheap.get() < amount {
                    return Err(format!(
                        "Cannot destroy {} cheap houses, only have {}.",
                        amount,
                        self.cheap.get()
                    ));
                }
                self.cheap.update(|h| *h -= 1);
            }
            HousingType::Standard => {
                if self.standard.get() < amount {
                    return Err(format!(
                        "Cannot destroy {} standard houses, only have {}.",
                        amount,
                        self.standard.get()
                    ));
                }
                self.standard.update(|h| *h -= 1);
            }
            HousingType::Fancy => {
                if self.fancy.get() < amount {
                    return Err(format!(
                        "Cannot destroy {} fancy houses, only have {}.",
                        amount,
                        self.fancy.get()
                    ));
                }
                self.fancy.update(|h| *h -= 1);
            }
        }
        Ok(())
    }
}

pub struct Accomodation(pub u64, pub &'static str);

pub struct HousingDetails {
    pub name: &'static str,
    pub description: &'static str,
    pub cost: f64,
    pub accomodates: Accomodation,
    pub land_used: u64,
}
