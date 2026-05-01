2026-05-01: `cargo build` emits pre-existing unsafe-code warnings across the project; none were introduced by this fix.
2026-05-01: The task brief's suggested `OnceLock<Clock>`/`OnceLock<ToggleKey>` pattern is not buildable here because SFML `Clock`/`SfBox` and `ToggleKey` are not `Sync`, and `Clock::restart()` requires mutable access.
2026-05-01: Manual QA found one remaining `static mut` in `src/world/generation/classic_over_world_generator.rs` and remaining `p_chunk` naming in the same file, so strict cleanup goals are not fully met.

## 2026-05-01 F1 compliance audit
- REJECT: remaining non-FFI `static mut` in `src/world/generation/classic_over_world_generator.rs`.
- `nix develop --command cargo build` and `nix develop --command cargo test` pass, but plain `cargo test` outside Nix still fails due missing SFML headers.
- Incomplete plan compliance remains in `src/world/event/player_dig_event.rs` (`PtrMutPlayer` + `unsafe impl Send`), `src/player/player.rs`/`src/util/fps_counter.rs` (`Box::leak` font singletons), `src/camera.rs` (remaining Doxygen), and `src/application.rs`/`src/world/world.rs` (unsafe still present; not all blocks documented).
- Evidence directories `wave-4/` and `wave-5/` exist but are empty.
- 2026-05-01: `cargo build` succeeded under `nix develop`, but the project still emits many pre-existing `unsafe_code` warnings outside this fix scope.
