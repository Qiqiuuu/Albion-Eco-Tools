use strum_macros::{Display, EnumIter};

#[derive(EnumIter, Copy, Clone, Display, PartialEq, Debug)]
pub enum FishSauce {
    Basic,
    Fancy,
    Special,
}

impl FishSauce {

    pub fn requirements(&self) -> (u32, u32) {
        match self {
            FishSauce::Basic   => (15, 1),
            FishSauce::Fancy   => (45, 3),
            FishSauce::Special => (135, 9),
        }
    }

    pub fn calculate_craft_cost(&self, price_fish: u32, price_seaweed: u32) -> u32 {
        let (fish, seaweed) = self.requirements();
        fish * price_fish + seaweed * price_seaweed
    }


}