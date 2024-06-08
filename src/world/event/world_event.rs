use crate::world::world::World;

pub trait IWorldEvent {
    fn handle(&self, world: &mut World);
}