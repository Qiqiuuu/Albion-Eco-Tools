use serde::{Deserialize, Serialize};
pub(crate) use crate::data::specializations::{ChiefSpecialization, Specialization};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CategoryOfSpecializations {
    Chief(Vec<ChiefSpecialization>),
}

impl CategoryOfSpecializations {
    pub fn default_list() -> Vec<Self> {
        vec![
            CategoryOfSpecializations::Chief(vec![
                ChiefSpecialization::Butcher(0),
                ChiefSpecialization::IngredientChef(0),
                ChiefSpecialization::SandwichChef(0),
                ChiefSpecialization::StewChef(0),
                ChiefSpecialization::OmeletteChef(0),
                ChiefSpecialization::RoastChef(0),
                ChiefSpecialization::PieChef(0),
                ChiefSpecialization::SaladChef(0),
                ChiefSpecialization::SoupChef(0)
            ])
        ]
    }


    pub fn label(&self) -> &'static str {
        match self {
            Self::Chief(_) => "Cooking",
        }
    }
    pub fn get_specs(&self) -> &Vec<ChiefSpecialization> {
        match self {
            Self::Chief(specs) => specs,
        }
    }
}












