use sfml::window::Event;
use crate::renderer::render_master::RenderMaster;

pub trait StateBase {
    fn handle_event(&mut self, event: Event);
    fn handle_input(&mut self);
    fn update(&mut self, delta_time: f32);
    fn render(&mut self, renderer: &mut RenderMaster);
    fn on_open(&mut self);
}