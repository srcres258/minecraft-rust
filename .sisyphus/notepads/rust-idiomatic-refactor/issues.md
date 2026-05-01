2026-05-01: `cargo build` emits pre-existing unsafe-code warnings across the project; none were introduced by this fix.
2026-05-01: The task brief's suggested `OnceLock<Clock>`/`OnceLock<ToggleKey>` pattern is not buildable here because SFML `Clock`/`SfBox` and `ToggleKey` are not `Sync`, and `Clock::restart()` requires mutable access.
