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
            },
            HousingType::Standard => HousingDetails {
                name: "Standard Housing",
                description: "Normal housing with some comforts.",
                cost: 750.0,
                accomodates: Accomodation(5, "Advanced"),
            },
            HousingType::Fancy => HousingDetails {
                name: "Fancy Housing",
                description: "A fancy home fit for a king.",
                cost: 1250.0,
                accomodates: Accomodation(3, "Expert"),
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
}

pub struct Accomodation(pub u64, pub &'static str);

pub struct HousingDetails {
    pub name: &'static str,
    pub description: &'static str,
    pub cost: f64,
    pub accomodates: Accomodation,
}
