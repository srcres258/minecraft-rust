2026-05-01: `Camera` now owns entity sync via `update_from_entity(&Entity)`; raw entity pointers were removed from camera state.
2026-05-01: `matrix::make_view_matrix(self)` still works through `Deref<Target = Entity>` on `Camera`.
