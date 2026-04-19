use serde::{Deserialize, Serialize};
use crate::models::config::{ActiveTab, AveragePrice, Cities};
use crate::models::items::TrackedFood;
use crate::models::specializations::{Category, CategoryId, Spec, SpecId};
use crate::models::specializations::CategoryId::Chief;

#[derive(Debug, Clone, Serialize, Deserialize,PartialEq)]
pub struct UserData {
    pub specializations: Vec<Category>,
    pub tracked_foods: Vec<TrackedFood>,
    pub active_tab: ActiveTab,
    pub active_category: CategoryId,
    pub use_premium: bool,
    pub use_focus: bool,
    pub silver_fee: u32,
    pub avg: AveragePrice,
    pub city: Cities
}

impl UserData {
    pub fn default_spec() -> Vec<Category> {
        vec![
            Category::new(
                Chief,
                vec![
                    Spec::new(SpecId::Butcher),
                    Spec::new(SpecId::IngredientChef),
                    Spec::new(SpecId::SandwichChef),
                    Spec::new(SpecId::StewChef),
                    Spec::new(SpecId::OmeletteChef),
                    Spec::new(SpecId::RoastChef),
                    Spec::new(SpecId::PieChef),
                    Spec::new(SpecId::SaladChef),
                    Spec::new(SpecId::SoupChef),
                ]
            )
        ]
    }

    pub fn set_spec_level(&mut self, spec_id: SpecId,level: u32) {
        self.specializations.iter_mut().any(|cat| {cat.set_spec_level(spec_id,level)});
    }

    pub fn set_mastery_level(&mut self, cat_id: CategoryId,level: u32) {
        if let Some(cat) = self.specializations.iter_mut().find(|c| c.id == cat_id) {
            cat.set_mastery_level(level);
        }
    }

    pub fn add_tracked_food(&mut self, food: TrackedFood) {
        if let Some(existing) = self.tracked_foods.iter_mut().find(|f|
            f.item.unique_name == food.item.unique_name && f.item.enchantment == food.item.enchantment
        ) {
            existing.quantity += food.quantity;
        } else {
            self.tracked_foods.push(food);
        }
    }

    pub fn remove_tracked_food(&mut self, index: usize) {
        if index < self.tracked_foods.len() {
            self.tracked_foods.remove(index);
        }
    }

    pub fn update_tracked_food(&mut self, index: usize, food: TrackedFood) {
        if let Some(entry) = self.tracked_foods.get_mut(index) {
            *entry = food;
        }
    }
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            specializations: Self::default_spec(),
            active_tab: ActiveTab::Cooking,
            tracked_foods: Vec::new(),
            active_category: Chief,
            use_premium: false,
            use_focus: false,
            silver_fee: 0,
            avg: AveragePrice::Day,
            city: Cities::Bracilien
        }
    }

}


