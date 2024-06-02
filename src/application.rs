use crate::camera::Camera;
use crate::config::Config;
use crate::context::Context;
use crate::renderer::render_master::RenderMaster;
use crate::states::state_base::StateBase;

static mut TIME_ELAPSED: f32 = 0.0;

/// @brief The main game application itself.
pub struct Application<'a> {
    states: Vec<Box<dyn StateBase>>,
    context: Context,
    master_renderer: RenderMaster,
    camera: Camera<'a>,
    config: Config,
    is_pop_state: bool
}