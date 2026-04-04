use serde::{Deserialize, Serialize};
use crate::data::sidebar::Specialization;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChiefSpecialization {
    Butcher(u32),
    IngredientChef(u32),
    SandwichChef(u32),
    StewChef(u32),
    OmeletteChef(u32),
    RoastChef(u32),
    PieChef(u32),
    SaladChef(u32),
    SoupChef(u32),
}


impl Specialization for ChiefSpecialization {
    fn get_name(&self) -> &'static str {
        match self {
            Self::Butcher(_) => "Butcher",
            Self::IngredientChef(_) => "Ingredient Chef",
            Self::SandwichChef(_) => "Sandwich Chef",
            Self::StewChef(_) => "Stew Chef",
            Self::OmeletteChef(_) => "Omelette Chef",
            Self::RoastChef(_) => "Roast Chef",
            Self::PieChef(_) => "Pie Chef",
            Self::SaladChef(_) => "Salad Chef",
            Self::SoupChef(_) => "Soup Chef",
        }
    }

    fn get_level(&self) -> u32 {
        match self {
            Self::Butcher(lv) | Self::IngredientChef(lv) | Self::SandwichChef(lv) |
            Self::StewChef(lv) | Self::OmeletteChef(lv) | Self::RoastChef(lv) |
            Self::PieChef(lv) | Self::SaladChef(lv) | Self::SoupChef(lv) => *lv,
        }
    }

    fn set_level(&mut self, new_level: u32) {
        match self {
            Self::Butcher(lv) | Self::IngredientChef(lv) | Self::SandwichChef(lv) |
            Self::StewChef(lv) | Self::OmeletteChef(lv) | Self::RoastChef(lv) |
            Self::PieChef(lv) | Self::SaladChef(lv) | Self::SoupChef(lv) => *lv = new_level,
        }
    }
}