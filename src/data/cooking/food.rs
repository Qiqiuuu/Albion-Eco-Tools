use strum_macros::Display;
use crate::data::cooking::stews::Stews;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AlbionItem {
    pub category: FoodCategory,
    pub tier: u8,
    pub enchant: u8,
}

impl AlbionItem {
    pub fn api_name(&self) -> String {
        let base = self.category.api_base();
        if self.enchant == 0 {
            format!("T{}_{}", self.tier, base)
        } else {
            format!("T{}_{}@{}", self.tier, base, self.enchant)
        }
    }
}

pub trait ApiTier {
    fn api_tier(&self) -> u8;

}
pub trait FoodName{
    fn food_name(&self) -> String;
}

pub trait ApiBase {
    fn api_base(&self) -> String;
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FoodCategory {
    Dish(DishCategory),
}

impl ApiBase for FoodCategory {
    fn api_base(&self) -> String {
        match self {
            FoodCategory::Dish(dish) => dish.api_base(),
        }
    }
}
impl FoodName for FoodCategory {
    fn food_name(&self) -> String {
        match self {
            FoodCategory::Dish(dish) => dish.food_name(),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DishCategory{
    Stew(Stews),
    // Salad(Salads),
}

impl ApiBase for DishCategory {
    fn api_base(&self) -> String {
        match self {
            DishCategory::Stew(stew) => format!("MEAL_{}",stew.api_base()),
            // DishCategory::Salad(salad) => format!("MEAL_{}",todo!())
        }
    }
}
impl DishCategory{
    pub fn get_all_dishes() -> Vec<Self> {
        let mut all = Vec::new();
        Stews::get_all_variants().into_iter().for_each(|stew| {all.push(Self::Stew(stew));});

        all
    }
}

impl FoodName for DishCategory {
    fn food_name(&self) -> String {
        match self {
            DishCategory::Stew(stew) => stew.food_name(),
            // DishCategory::Salad(salad) => todo!()
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Salads{
    Basic(CropTypes),
    Special()
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Display)]
pub enum MeatTypes{
    Goat,
    Mutton,
    Beef,
    Pork,
    Goose
}
impl ApiTier for MeatTypes {
    fn api_tier(&self) -> u8 {
        match self {
            MeatTypes::Goat => 4,
            MeatTypes::Mutton => 6,
            MeatTypes::Beef => 8,
            MeatTypes::Pork => 7,
            MeatTypes::Goose => 5,
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CropTypes{
    Turnip,
    Cabbage,
    Potato,
    Corn,
    Pumpkin
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HerbTypes{
    CrenellatedBurdock,
    DragonTeasel,
    ElusiveFoxglove,
    FiretouchedMullein,
    GhoulYarrow
}





#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Octopuses{
    Midwater,
    Deepwater,
}





