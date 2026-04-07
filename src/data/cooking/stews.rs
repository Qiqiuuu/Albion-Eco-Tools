use strum_macros::Display;
use crate::data::cooking::food::{ApiBase, ApiTier, FoodName, MeatTypes};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Stews{
    Basic(MeatTypes),
    Eel(Eels),
    Avalonian(MeatTypes),
}

impl Stews {
    pub fn get_all_variants() -> Vec<Self> {
        let mut v = Vec::new();
        let meats = [MeatTypes::Goat, MeatTypes::Mutton, MeatTypes::Beef];
        let eels = [Eels::Greenriver, Eels::Redspring, Eels::Deadwater];

        for m in meats { v.push(Self::Basic(m)); }
        for e in eels { v.push(Self::Eel(e)); }
        for m in meats { v.push(Self::Avalonian(m)); }
        v
    }
}

impl ApiBase for Stews {
    fn api_base(&self) -> String {
        match &self {
            Stews::Basic(_) => "STEW".to_string(),
            Stews::Eel(_) => "STEW_FISH".to_string(),
            Stews::Avalonian(_) => "STEW_AVALON".to_string(),
        }
    }
}

impl ApiTier for Stews {
    fn api_tier(&self) -> u8 {
        match self {
            Stews::Basic(meat) => meat.api_tier(),
            Stews::Eel(eel) => eel.api_tier(),
            Stews::Avalonian(meat) => meat.api_tier(),
        }
    }
}

impl FoodName for Stews {
    fn food_name(&self) -> String {
        match self {
            Stews::Basic(meat) => format!("{} Stew", meat.to_string()),
            Stews::Avalonian(meat) => format!("Avalonian {} Stew", meat.to_string()),
            Stews::Eel(eel) => format!("{} Eel Stew", eel.to_string()),
        }
    }

}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq,Display)]
pub enum Eels{
    Greenriver,
    Redspring,
    Deadwater
}

impl ApiTier for Eels{
    fn api_tier(&self) -> u8{
        match &self {
            Eels::Greenriver => 3,
            Eels::Redspring => 5,
            Eels::Deadwater => 7
        }
    }
}