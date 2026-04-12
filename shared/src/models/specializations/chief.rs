use serde::{Deserialize, Serialize};
use crate::models::specializations::Specialization;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChefSpec {
    pub kind: ChefKind,
    pub level: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChefKind {
    Butcher,
    IngredientChef,
    SandwichChef,
    StewChef,
    OmeletteChef,
    RoastChef,
    PieChef,
    SaladChef,
    SoupChef,
}
impl Specialization for ChefSpec {
    fn get_name(&self) -> &'static str {
        match self.kind {
            ChefKind::Butcher       => "Butcher",
            ChefKind::IngredientChef => "Ingredient Chef",
            ChefKind::SandwichChef  => "Sandwich Chef",
            ChefKind::StewChef      => "Stew Chef",
            ChefKind::OmeletteChef  => "Omelette Chef",
            ChefKind::RoastChef     => "Roast Chef",
            ChefKind::PieChef       => "Pie Chef",
            ChefKind::SaladChef     => "Salad Chef",
            ChefKind::SoupChef      => "Soup Chef",
        }
    }
    fn get_level(&self) -> u32 { self.level }
    fn set_level(&mut self, level: u32) { self.level = level; }

    fn get_focus_efficiency(&self) -> u32 {
        self.level * 250
    }

    fn get_passive_efficiency(&self) -> u32 {
        self.level * 30
    }
}