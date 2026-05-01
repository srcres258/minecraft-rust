2026-05-01: `Camera` now owns entity sync via `update_from_entity(&Entity)`; raw entity pointers were removed from camera state.
2026-05-01: `matrix::make_view_matrix(self)` still works through `Deref<Target = Entity>` on `Camera`.
2026-05-01: `sfml::SfBox<Clock>` and `ToggleKey` are not `Send`/`Sync`, so `StatePlay` globals needed `thread_local!` + `RefCell` rather than `OnceLock`.
2026-05-01: `application::TIME_ELAPSED` can be made safe with an `AtomicU32` storing `f32::to_bits()` and a small accessor API for renderers.
2026-05-01: `UnsafeCell` access sites in `Application::new`, `Application::run_loop`, and `main()` should keep the unsafe scope to the smallest possible deref/call and carry explicit `// SAFETY:` comments.
2026-05-01: Manual QA F3 confirms runtime-facing refactor checks around skybox/frustum/view-matrix passed, but naming cleanup is incomplete (`p_chunk` still appears in classic world generator).
- 2026-05-01: Replaced the classic overworld one-time static bool gate with OnceLock while preserving the existing noise seed and parameter setup.
