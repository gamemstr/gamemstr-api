pub mod items;
pub mod spells;
pub mod worlds;

pub trait Model {
    type Entity;
    fn new(entity: Self::Entity) -> Self;
    fn to_entity(&self) -> Self::Entity;
}
