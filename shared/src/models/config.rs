use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
#[derive(Clone, Copy, PartialEq)]
pub enum ActiveTab {
    Cooking,
    Farming,
    Gathering,
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize,EnumIter,Display)]
pub enum Cities{
    Bracilien,
    Carleon,
    Thetford,
    Lymhurst,
    Martlock,
    FortSterling,
    Bridgewatch,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize,EnumIter,Display)]
pub enum AveragePrice {
    Day,
    Week,
    Month,
}

