use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display)]
pub enum SpecId {
    Butcher, IngredientChef, SandwichChef, StewChef,
    OmeletteChef, RoastChef, PieChef, SaladChef, SoupChef,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Spec {
    pub id: SpecId,
    pub level: u32,
}

impl Spec {
    pub fn new(id: SpecId) -> Self {
        Self { id, level: 0 }
    }

    pub fn focus_efficiency(&self) -> u32 { self.level * 250 }
    pub fn passive_efficiency(&self) -> u32 { self.level * 30 }
    
    pub fn get_name(&self) -> String {self.id.to_string()}

    pub fn get_level(&self) -> u32 { self.level }
    pub fn set_level(&mut self, level: u32) { self.level = level; }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub id: CategoryId,
    pub mastery_level: u32,
    pub specs: Vec<Spec>,
}

impl Category {
    pub fn new(id: CategoryId, specs: Vec<Spec>) -> Self {
        Self { id, mastery_level: 0, specs }
    }
    pub fn set_mastery_level(&mut self, mastery_level: u32) { self.mastery_level = mastery_level; }
    pub fn get_mastery_level(&self) -> u32 { self.mastery_level }

    pub fn get_label(&self) -> String {
        self.id.to_string()
    }

    pub fn get_specs(&self) -> &Vec<Spec> { &self.specs }

    pub fn set_spec_level(&mut self, spec_id: SpecId, level: u32) -> bool{
        if let Some(spec) = self.specs.iter_mut().find(|s| s.id == spec_id) {
            spec.set_level(level);
            return true;
        }
        false
    }


}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, Display, PartialEq, Eq)]
pub enum CategoryId {
    Chief,
}

impl CategoryId {
    pub fn get_tab(&self) -> &'static str {
        match self {
            Self::Chief => "Cooking",
        }
    }
}
