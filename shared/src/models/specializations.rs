use serde::{Deserialize, Serialize};
use crate::models::specializations::chief::{ChefKind, ChefSpec};

pub mod chief;
pub mod refiner;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecLevel(pub u32);

pub trait Specialization {
    fn get_name(&self) -> &'static str;
    fn get_level(&self) -> u32;
    fn set_level(&mut self, level: u32);

    fn get_focus_efficiency(&self) -> u32;
    fn get_passive_efficiency(&self) -> u32;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpecKind {
    Chef(ChefKind),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Specializations {
    Chief(u32, Vec<ChefSpec>),
}

impl Specializations {
    pub fn default_list() -> Vec<Self> {
        vec![
            Specializations::Chief(0, vec![
                ChefSpec { kind: ChefKind::Butcher, level: 0 },
                ChefSpec { kind: ChefKind::IngredientChef, level: 0 },
                ChefSpec { kind: ChefKind::SandwichChef, level: 0 },
                ChefSpec { kind: ChefKind::StewChef, level: 0 },
                ChefSpec { kind: ChefKind::OmeletteChef, level: 0 },
                ChefSpec { kind: ChefKind::RoastChef, level: 0 },
                ChefSpec { kind: ChefKind::PieChef, level: 0 },
                ChefSpec { kind: ChefKind::SaladChef, level: 0 },
                ChefSpec { kind: ChefKind::SoupChef, level: 0 },
            ]),
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Chief(_, _)   => "Cooking",
        }
    }

    pub fn calculate_total_efficiency(&self, target: &SpecKind) -> u32 {
        match (self, target) {
            (Self::Chief(mastery_lvl, specs), SpecKind::Chef(target_kind)) => {
                let mut total = mastery_lvl * 30;
                for spec in specs {
                    if &spec.kind == target_kind {
                        total += spec.get_focus_efficiency();
                    } else {
                        total += spec.get_passive_efficiency();
                    }
                }
                total
            }
            _ => 0,
        }
    }
}

pub fn find_efficiency(all_specs: &[Specializations], target: &SpecKind) -> u32 {
    all_specs
        .iter()
        .map(|s| s.calculate_total_efficiency(target))
        .max()
        .unwrap_or(0)
}