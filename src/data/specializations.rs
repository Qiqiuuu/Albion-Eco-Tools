mod chief;
pub use chief::ChiefSpecialization;


pub trait Specialization {
    fn get_name(&self) -> &'static str;
    fn get_level(&self) -> u32;
    fn set_level(&mut self, level: u32);
}