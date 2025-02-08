use crate::world::world::World;

pub trait IWorldEvent : Sync {
    fn handle(&mut self, world: &mut World);
}