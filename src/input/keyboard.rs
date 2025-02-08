use std::mem;
use sfml::window::{Event, Key};

/// @brief Handles keyboard inputs and events.
pub struct Keyboard {
    keys: [bool; mem::variant_count::<Key>()],
    recently_released: Key
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; mem::variant_count::<Key>()],
            recently_released: Key::Unknown
        }
    }

    pub fn update(&mut self, e: Event) {
        self.recently_released = Key::Unknown;
        match e {
            Event::KeyReleased { code, .. } => {
                if code != Key::Unknown {
                    self.keys[code as usize] = false;
                }
            }
            Event::KeyPressed { code, .. } => {
                if code != Key::Unknown {
                    self.keys[code as usize] = true;
                }
            }
            _ => {}
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        let ord = key as isize;
        if ord < 0 {
            false
        } else {
            self.keys[ord as usize]
        }
    }
    pub fn key_released(&self, key: Key) -> bool {
        self.recently_released == key
    }
}

fn try_ordinal_to_key(ord: isize) -> Option<Key> {
    match ord {
         -1 => Some(Key::Unknown),
          0 => Some(Key::A),
          1 => Some(Key::B),
          2 => Some(Key::C),
          3 => Some(Key::D),
          4 => Some(Key::E),
          5 => Some(Key::F),
          6 => Some(Key::G),
          7 => Some(Key::H),
          8 => Some(Key::I),
          9 => Some(Key::J),
         10 => Some(Key::K),
         11 => Some(Key::L),
         12 => Some(Key::M),
         13 => Some(Key::N),
         14 => Some(Key::O),
         15 => Some(Key::P),
         16 => Some(Key::Q),
         17 => Some(Key::R),
         18 => Some(Key::S),
         19 => Some(Key::T),
         20 => Some(Key::U),
         21 => Some(Key::V),
         22 => Some(Key::W),
         23 => Some(Key::X),
         24 => Some(Key::Y),
         25 => Some(Key::Z),
         26 => Some(Key::Num0),
         27 => Some(Key::Num1),
         28 => Some(Key::Num2),
         29 => Some(Key::Num3),
         30 => Some(Key::Num4),
         31 => Some(Key::Num5),
         32 => Some(Key::Num6),
         33 => Some(Key::Num7),
         34 => Some(Key::Num8),
         35 => Some(Key::Num9),
         36 => Some(Key::Escape),
         37 => Some(Key::LControl),
         38 => Some(Key::LShift),
         39 => Some(Key::LAlt),
         40 => Some(Key::LSystem),
         41 => Some(Key::RControl),
         42 => Some(Key::RShift),
         43 => Some(Key::RAlt),
         44 => Some(Key::RSystem),
         45 => Some(Key::Menu),
         46 => Some(Key::LBracket),
         47 => Some(Key::RBracket),
         48 => Some(Key::Semicolon),
         49 => Some(Key::Comma),
         50 => Some(Key::Period),
         51 => Some(Key::Quote),
         52 => Some(Key::Slash),
         53 => Some(Key::Backslash),
         54 => Some(Key::Tilde),
         55 => Some(Key::Equal),
         56 => Some(Key::Hyphen),
         57 => Some(Key::Space),
         58 => Some(Key::Enter),
         59 => Some(Key::Backspace),
         60 => Some(Key::Tab),
         61 => Some(Key::PageUp),
         62 => Some(Key::PageDown),
         63 => Some(Key::End),
         64 => Some(Key::Home),
         65 => Some(Key::Insert),
         66 => Some(Key::Delete),
         67 => Some(Key::Add),
         68 => Some(Key::Subtract),
         69 => Some(Key::Multiply),
         70 => Some(Key::Divide),
         71 => Some(Key::Left),
         72 => Some(Key::Right),
         73 => Some(Key::Up),
         74 => Some(Key::Down),
         75 => Some(Key::Numpad0),
         76 => Some(Key::Numpad1),
         77 => Some(Key::Numpad2),
         78 => Some(Key::Numpad3),
         79 => Some(Key::Numpad4),
         80 => Some(Key::Numpad5),
         81 => Some(Key::Numpad6),
         82 => Some(Key::Numpad7),
         83 => Some(Key::Numpad8),
         84 => Some(Key::Numpad9),
         85 => Some(Key::F1),
         86 => Some(Key::F2),
         87 => Some(Key::F3),
         88 => Some(Key::F4),
         89 => Some(Key::F5),
         90 => Some(Key::F6),
         91 => Some(Key::F7),
         92 => Some(Key::F8),
         93 => Some(Key::F9),
         94 => Some(Key::F10),
         95 => Some(Key::F11),
         96 => Some(Key::F12),
         97 => Some(Key::F13),
         98 => Some(Key::F14),
         99 => Some(Key::F15),
        100 => Some(Key::Pause),
          _ => None
    }
}