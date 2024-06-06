use sfml::SfBox;
use sfml::system::Clock;
use sfml::window::Key;

/// @brief A keyboard related subclass that determines if a key remains pressed.
pub struct ToggleKey {
    key: Key,
    delay_timer: SfBox<Clock>
}

impl ToggleKey {
    pub fn new(key: Key) -> Self {
        Self {
            key,
            delay_timer: Clock::start()
        }
    }

    pub fn is_key_pressed(&mut self) -> bool {
        if self.delay_timer.elapsed_time().as_seconds() > 0.2 {
            if self.key.is_pressed() {
                self.delay_timer.restart();
                return true;
            }
        }
        false
    }
}