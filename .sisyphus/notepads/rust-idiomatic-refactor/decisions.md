2026-05-01: Kept camera matrix math unchanged and only restored the missing imports plus struct definition.
2026-05-01: Synced the camera from the player after `Player::update()` in `StatePlay` to preserve existing movement behavior.
2026-05-01: Replaced `StatePlay`'s `static mut` timer/toggle globals with thread-local `RefCell` storage plus `AtomicBool` for GUI visibility, preserving single-threaded input/render semantics without raw global pointers.
2026-05-01: Replaced `application.rs` `TIME_ELAPSED` `static mut f32` with an `AtomicU32` bit-pattern accumulator and `time_elapsed()` accessor so shader renderers no longer read mutable statics.
