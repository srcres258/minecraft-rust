use crate::camera::Camera;
use crate::config::Config;
use crate::context::Context;
use crate::renderer::render_master::RenderMaster;
use crate::states::state_base::StateBase;

static mut TIME_ELAPSED: f32 = 0.0;

/// @brief The main game application itself.
pub struct Application {
    states: Vec<Box<dyn StateBase>>,
    context: Context,
    master_renderer: RenderMaster,
    camera: Camera,
    config: Config,
    is_pop_state: bool
}
//
// impl<'a> Application<'a> {
//     pub fn new(config: Config) -> Self {
//         BlockDatabase::get();
//
//     }
//
//     pub fn push_state(&mut self, config: Config) {
//         self.states.push(Box::new())
//     }
// }