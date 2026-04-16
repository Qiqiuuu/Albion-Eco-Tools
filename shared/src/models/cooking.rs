use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, Display};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter,Display)]
pub enum FishSauce{
    Basic,
    Fancy,
    Special,
}

impl FishSauce {
    pub fn get_unique_name(&self) -> &str {
        match self {
            FishSauce::Basic => "T1_FISHSAUCE_LEVEL1",
            FishSauce::Fancy => "T1_FISHSAUCE_LEVEL2",
            FishSauce::Special => "T1_FISHSAUCE_LEVEL3",

        }
    }
}
pub const CHOPPED_FISH: &str = "T1_FISHCHOPS";
pub const SEAWEED: &str = "T1_SEAWEED";
