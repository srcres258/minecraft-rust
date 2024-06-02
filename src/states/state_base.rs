use sfml::window::Event;

pub trait StateBase {
    fn handle_event(&self, event: Event);
    fn handle_input(&self);
    fn update(&self, delta_time: f32);
    fn render(&self, renderer: &RenderMaster);
    fn on_open(&self);
}