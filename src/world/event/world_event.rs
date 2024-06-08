use crate::world::world::World;

pub trait IWorldEvent: Send {
    fn handle(&self, world: &mut World);
}