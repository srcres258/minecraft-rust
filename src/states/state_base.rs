use sfml::window::Event;

pub trait StateBase {
    fn handle_event(&mut self, e: Event);
    fn handle_input(&mut self);

    fn update(&mut self, delta_time: f32);

    // todo: render

    fn on_open(&mut self);
}