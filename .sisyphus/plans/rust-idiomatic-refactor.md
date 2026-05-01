# Rust Idiomatic Refactoring of minecraft-rust

## TL;DR

> **Quick Summary**: Comprehensive refactoring of an 88-file Minecraft clone from C++-style Rust to idiomatic modern Rust — reducing `unsafe`, migrating `mod.rs` conventions, fixing 2 rendering bugs (skybox + frustum culling), and establishing TDD infrastructure. Every module refactored incrementally with `cargo build` verification after each step.

> **Deliverables**:
> - Fixed skybox rendering (black background → visible sky)
> - Fixed frustum culling (over-aggressive → correct visibility)
> - 19 `mod.rs` files migrated to modern module convention
> - ~35 non-FFI `unsafe` sites eliminated or properly scoped
> - Rust-idiomatic patterns: `OnceLock`/`LazyLock` for singletons, `&[T]` over `&Vec<T>`, `enumerate()` over index loops, `From` over `as` casts, standard Rust docs over Doxygen
> - Full test infrastructure (`#[test]` + TDD workflow) where none existed
> - `cargo build` passes in Nix dev shell after every change

> **Estimated Effort**: Large (50+ tasks across 6 waves)
> **Parallel Execution**: YES — 3 parallel tracks in Waves 1-3, merging from Wave 4 onward
> **Critical Path**: Test infra → Bug fixes → Foundation refactoring → Core modules → Final verification

---

## Context

### Original Request
Refactor the `minecraft-rust` codebase (C++→Rust port of [MineCraft-One-Week-Challenge](https://github.com/Hopson97/MineCraft-One-Week-Challenge)) to:
1. Reduce `unsafe` usage
2. Adopt modern Rust patterns (not C++ porting patterns)
3. Migrate from `mod.rs` convention to modern `module_name.rs`
4. Fix skybox rendering bug (black background)
5. Fix frustum culling bug (over-aggressive culling)
6. Ensure `cargo build` works in Nix dev shell

### Interview Summary
**Key Discussions**:
- **Test Strategy**: TDD — add `#[test]` infrastructure before refactoring each module (user chose TDD over tests-after or no-tests)
- **Unsafe Reduction**: Aggressive — eliminate ALL removable `unsafe`, keep only GL/SFML FFI calls (user chose maximum reduction)
- **Refactoring Approach**: Incremental — one module at a time, `cargo build` after each (user chose incremental over cross-cutting)
- **Verification**: Agent-Executed QA Scenarios (Playwright UI / tmux CLI / curl API) alongside TDD tests for every task

**Research Findings**:

**Unsafe Inventory** (~42 unsafe sites across 28 files):
- **CAN_REMOVE** (5 sites): `main.rs:110`, `application.rs:57`, `play_state.rs:68,88,203`, `chunk_manager.rs:102` — BUT these require architectural pre-work (see Metis gap analysis)
- **SINGLETON** (11 sites): Global timers, fonts, input state, block database, noise generator — replaceable with `OnceLock`/`LazyLock`
- **THREADING** (17 sites): Shared Camera/World via `UnsafeCellWrapper`, `PtrConstEntity`, `PtrMutPlayer` — safest to scope `unsafe` tightly rather than redesign threading now
- **GL_FFI** (10+ sites): OpenGL calls — must stay `unsafe`, but wrap better

**Bug Root Causes Confirmed**:
- **Skybox**: ① `SkyboxShader::default()` and `::new()` never call `get_uniforms()` (uniform locations stay 0) ② Skybox drawn last with default `glDepthFunc(LESS)` — fragments behind geometry fail depth test
- **Frustum Culling**: ① `matrix.rs:53` — rotation.z uses wrong axis `(1,0,1)` instead of `(0,0,1)` ② `frustum.rs:74` — left plane uses `mat[(0,2)]` instead of `mat[(0,3)]` ③ `frustum.rs:120-121` — returns `true` on first plane intersection instead of checking all 6 planes

**C++ Anti-Patterns** (10 categories, all to be addressed):
1. Raw pointers / unsafe singletons → `OnceLock`/`LazyLock` + `Arc<Mutex<T>>`
2. `Box::leak` for globals → `OnceLock`/`LazyLock`
3. C-style casts (`as _`) → `From`/`TryFrom` + newtypes
4. `&Vec<T>` params → `&[T]`
5. Index-based loops → iterators with `enumerate()`
6. Large match blocks → methods on enums
7. Doxygen comments → standard `///` Rust docs
8. `unwrap()`/`expect()` everywhere → scoped, documented
9. Integer fields where enums/newtypes belong → domain newtypes
10. Non-idiomatic naming (`p_world`, `p_chunk`) → standard snake_case

### Metis Review
**Identified Gaps** (addressed in plan):
- **`CAN_REMOVE` wasn't trivially removable**: `main.rs:110` and `application.rs:57` require architectural pre-work (separate `Application` init from `Rc<UnsafeCell>` wrapping) — added as dedicated tasks in Wave 3
- **Thread safety has deeper issues**: `World::load_chunks` reads shared fields without synchronization, `Drop` uses non-atomic `get_mut()` — scoped as `unsafe` tightening rather than full threading redesign
- **`extern crate nalgebra_glm as glm;`** in 14 files → replace with `use nalgebra_glm as glm;` — added as Wave 2 task
- **`edition = "2024"`** needs validation — first task in Wave 0 verifies build
- **`static mut NOISE_GEN`** in `classic_over_world_generator.rs` was missed in initial inventory — included in Wave 3 singleton removal
- **`matrix.rs:53` bug affects ALL rendering**, not just culling — the view matrix itself is wrong — highest priority fix
- **`Water.frag` doesn't exist** — intentional (Water.vert feeds Chunk.frag's inputs), documented as-is
- **SFML `Text<'a>` lifetime propagation** — `StatePlay<'a>` and `Player<'a>` carry unnecessary lifetimes from leaked Font — fix by using `OnceLock<Font>` instead of `static mut`

---

## Work Objectives

### Core Objective
Refactor the minecraft-rust codebase into idiomatic, safe Rust while preserving all gameplay behavior and fixing two rendering bugs.

### Concrete Deliverables
- Skybox renders correctly (visible sky instead of black)
- Frustum culling correctly shows/hides geometry
- 19 `mod.rs` → `module_name.rs` migrated
- `cargo build` succeeds in Nix dev shell
- 35+ non-FFI `unsafe` blocks eliminated or properly scoped
- Test infrastructure established with passing tests
- All modules follow modern Rust conventions

### Definition of Done
- [x] `cargo build` passes with zero errors in Nix dev shell
- [ ] `cargo test` passes all tests
- [ ] `cargo clippy` reports zero warnings (or existing warnings only, no new ones)
- [ ] Skybox visible on launch (not black)
- [ ] Frustum culling: chunks at screen edges render correctly (no visible pop-in/out at wrong positions)
- [x] No `mod.rs` files remain in `src/` (except possibly if blocked by tooling)
- [x] All non-FFI `unsafe` blocks documented with `// SAFETY:` comments

### Must Have
- Skybox fix applied
- Frustum culling fix applied
- mod.rs migration complete
- Unsafe reduction for all SINGLETON sites
- Thread-unsafe unsafes tightly scoped with `// SAFETY:` docs
- Test infrastructure + tests for all refactored modules
- `cargo build` in Nix dev shell

### Must NOT Have (Guardrails)
- **DO NOT** change the threading architecture (thread count, spawn points, `ChunkManager` structure)
- **DO NOT** change GL FFI calls (they must stay `unsafe`)
- **DO NOT** add new gameplay features
- **DO NOT** refactor `chunk_mesh_builder.rs` algorithmically (only unsafe/style changes)
- **DO NOT** change skybox textures or add new visual effects
- **DO NOT** change player position clamping logic
- **DO NOT** replace `nalgebra-glm` or `sfml` crate dependencies
- **DO NOT** add comprehensive error handling (keep `unwrap()` where it was, scope new errors)
- **DO NOT** create `Water.frag` (the Water.vert→Chunk.frag reuse is intentional)

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (zero tests in project)
- **Automated tests**: TDD (write tests first, then refactor)
- **Framework**: `cargo test` (Rust's built-in test framework — no additional deps needed)
- **TDD Flow**: Each task follows RED (failing test) → GREEN (minimal impl) → REFACTOR

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Frontend/UI**: Playwright — browser-based verification of the running game
- **CLI/Build**: Bash — `cargo build`, `cargo test`, `cargo clippy`
- **API/Library**: Bash — `cargo test` unit tests
- **Shader/GL**: Bash — static code verification + build test

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 0 (Prerequisites — sequential foundation):
├── Task 0.1: Validate cargo build in Nix dev shell [quick]
├── Task 0.2: Establish test infrastructure [quick]
└── Task 0.3: Create .sisyphus/evidence directory [quick]

Wave 1 (Bug Fixes — parallel tracks, high value, low risk):
Track A — Skybox Fix:
├── Task 1A.1: Fix SkyboxShader::default() + ::new() missing get_uniforms() [quick]
├── Task 1A.2: Fix skybox depth test (GL_LEQUAL before draw) [quick]
├── Task 1A.3: Write unit test for skybox uniform initialization [quick]
Track B — Frustum Fix:
├── Task 1B.1: Fix matrix.rs:53 wrong Z rotation axis [quick]
├── Task 1B.2: Fix frustum.rs:74 wrong left-plane matrix element [quick]
├── Task 1B.3: Fix frustum.rs:120-121 premature intersection return [quick]
├── Task 1B.4: Write frustum plane extraction unit tests [unspecified-high]
├── Task 1B.5: Write frustum AABB culling unit tests [unspecified-high]

Wave 2 (Mechanical Cleanup — MAX PARALLEL, independent tracks):
Track C — extern crate removal:
├── Task 2C.1: Replace extern crate nalgebra_glm in all 14 files [quick]
Track D — mod.rs migration (19 files, parallelizable):
├── Task 2D.1-19: Rename each mod.rs to sibling module_name.rs [quick each]
Track E — Dependency modernization:
├── Task 2E.1: Replace lazy_static with std::sync::LazyLock [quick]

Wave 3 (Foundation Refactoring — low-dependency modules first):
├── Task 3.1: Refactor util/unsafe_cell_wrapper.rs — tight scoping [quick]
├── Task 3.2: Refactor util/random.rs — OnceLock singleton [quick]
├── Task 3.3: Refactor util/array2d.rs — idiomatic iterators [quick]
├── Task 3.4: Refactor util/fps_counter.rs — remove lifetime propagation [quick]
├── Task 3.5: Refactor maths/frustum.rs — idiomatic + docs [quick]
├── Task 3.6: Refactor maths/matrix.rs — idiomatic + docs [quick]
├── Task 3.7: Refactor maths/ray.rs — idiomatic [quick]
├── Task 3.8: Refactor maths/noise_generator.rs — OnceLock singleton [quick]
├── Task 3.9: Refactor config.rs + physics/aabb.rs — misc cleanup [quick]

Wave 4 (Core Module Refactoring — sequential per module, parallel across independent):
├── Task 4.1: Refactor world/block/block_id.rs — From impls [quick]
├── Task 4.2: Refactor world/block/block_data.rs — enum methods [unspecified-high]
├── Task 4.3: Refactor world/block/block_database.rs — OnceLock singleton [quick]
├── Task 4.4: Refactor world/block/chunk_block.rs — newtype [quick]
├── Task 4.5: Refactor player/player.rs — replace static mut globals [deep]
├── Task 4.6: Refactor camera.rs — PtrConstEntity → safe entity tracking [unspecified-high]
├── Task 4.7: Refactor world/event/player_dig_event.rs — safe player ref [quick]
├── Task 4.8: Refactor world/chunk/chunk_section.rs — scope unsafe [unspecified-high]
├── Task 4.9: Refactor world/chunk/chunk_manager.rs — remove raw ptr cast [quick]
├── Task 4.10: Refactor world/world.rs — replace raw ptr updates [deep]
├── Task 4.11: Refactor states/play_state.rs — eliminate static mut [deep]
├── Task 4.12: Refactor application.rs — separate Rc<UnsafeCell> init [unspecified-high]
├── Task 4.13: Refactor main.rs — safe Application::run_loop() [quick]

Wave 5 (Polish — idiomatic style across remaining files):
├── Task 5.1: Replace &Vec<T> with &[T] in model.rs + others [quick]
├── Task 5.2: Convert index loops to iterators (chunk_mesh, player, noise) [quick]
├── Task 5.3: Convert Doxygen comments to standard Rust docs (all files) [writing]
├── Task 5.4: Fix non-idiomatic naming (p_world → world, etc.) [quick]
├── Task 5.5: Add #![warn(unsafe_code)] attribute with FFI exceptions [quick]

Wave FINAL (After ALL tasks — 4 parallel reviews, then user okay):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
└── Task F4: Scope fidelity check (deep)
-> Present results -> Get explicit user okay
```

### Agent Dispatch Summary

- **Wave 0**: 3 × `quick`
- **Wave 1**: 6 × `quick`, 2 × `unspecified-high`
- **Wave 2**: 21 × `quick` (mechanical changes)
- **Wave 3**: 9 × `quick`
- **Wave 4**: 5 × `quick`, 3 × `unspecified-high`, 3 × `deep`
- **Wave 5**: 4 × `quick`, 1 × `writing`
- **FINAL**: 1 × `oracle`, 2 × `unspecified-high`, 1 × `deep`

---

## TODOs

### Wave 0 — Prerequisites

- [x] 0.1. Validate `cargo build` in Nix dev shell

  **What to do**:
  - Enter Nix dev shell: `nix develop`
  - Run `cargo build` and capture output
  - Verify zero errors (warnings acceptable)
  - Document the Rust toolchain version (`rustc --version`)
  - If build fails, diagnose and report before any other work

  **Must NOT do**:
  - Do NOT change any source code
  - Do NOT modify `Cargo.toml` or `flake.nix`

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single verification command, no code changes
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 0 — sequential
  - **Blocks**: 0.2, 0.3

  **References**:
  - `flake.nix` — Nix dev shell definition with all dependencies
  - `Cargo.toml` — Edition 2024, crate dependencies

  **Acceptance Criteria**:
  - [ ] `cargo build` completes with zero errors in Nix dev shell
  - [ ] Rust toolchain version recorded

  **QA Scenarios**:
  ```
  Scenario: Build succeeds in Nix dev shell
    Tool: Bash
    Preconditions: Nix installed, flakes enabled
    Steps:
      1. nix develop --command cargo build 2>&1
      2. Assert exit code is 0
      3. Assert no lines contain "error["
    Expected Result: Build completes successfully, binary at target/debug/minecraft-rust
    Failure Indicators: Exit code != 0, or lines containing "error[" in output
    Evidence: .sisyphus/evidence/task-0.1-build.log
  ```

  **Commit**: NO (verification only)

- [x] 0.2. Establish test infrastructure

  **What to do**:
  - Create `src/test_utils.rs` with shared test helpers (if needed later)
  - Add a smoke test in `src/main.rs` or a new test module: `#[test] fn smoke_build() { assert!(true); }`
  - Run `cargo test` to verify test framework works
  - Add `.sisyphus/evidence/` directory for QA artifacts

  **Must NOT do**:
  - Do NOT add test dependencies to Cargo.toml unless needed
  - Do NOT create empty test modules that fail compilation

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple setup task, create file + run command
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 0 — sequential
  - **Blocks**: All subsequent tasks (tests must exist before TDD)
  - **Blocked By**: 0.1

  **References**:
  - `Cargo.toml` — no `[dev-dependencies]` listed

  **Acceptance Criteria**:
  - [ ] `cargo test` executes and passes at least 1 test
  - [ ] `.sisyphus/evidence/` directory exists

  **QA Scenarios**:
  ```
  Scenario: Test framework operational
    Tool: Bash
    Steps:
      1. cargo test 2>&1
      2. Assert exit code is 0
      3. Assert output contains "test result: ok" with at least 1 passed
    Expected Result: Tests run and pass
    Evidence: .sisyphus/evidence/task-0.2-test-results.log
  ```

  **Commit**: YES
  - Message: `build: establish test infrastructure and smoke test`
  - Files: `src/test_utils.rs` (empty if unused), test additions
  - Pre-commit: `cargo test`

- [x] 0.3. Create evidence directory structure

  **What to do**:
  - Create `.sisyphus/evidence/` directory if not already created in 0.2
  - Create subdirectories: `wave-1/`, `wave-2/`, `wave-3/`, `wave-4/`, `wave-5/`, `final/`

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 0 — sequential
  - **Blocked By**: 0.1

  **Acceptance Criteria**:
  - [ ] All evidence directories exist

  **QA Scenarios**: N/A (directory creation verified by acceptance criteria)

  **Commit**: NO (filesystem setup)

### Wave 1 — Bug Fixes (Track A: Skybox)

- [x] 1A.1. Fix SkyboxShader::default() and ::new() missing get_uniforms()

  **What to do**:
  - In `src/shaders/skybox_shader.rs`, add `result.get_uniforms()` call in BOTH `SkyboxShader::default()` (after line 55) and `SkyboxShader::new()` (after line 34)
  - Follow the exact pattern from `ChunkShader::default()` at `src/shaders/chunk_shader.rs:29`:
    ```rust
    let result = Self { ... };
    result.get_uniforms();
    result
    ```
  - The `get_uniforms()` method must be called AFTER the program is loaded (line 47-51 in default, line 22-30 in new) and BEFORE returning

  **Must NOT do**:
  - Do NOT modify `get_uniforms()` method itself
  - Do NOT change any other shader's initialization
  - Do NOT reorder the program loading or uniform calls

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Two-line fix following existing pattern
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - None needed — pattern is well-established in adjacent files

  **Parallelization**:
  - **Parallel Group**: Wave 1 — Track A
  - **Blocks**: 1A.2, 1A.3

  **References**:
  - `src/shaders/chunk_shader.rs:24-32` — Pattern to follow (ChunkShader::default calls get_uniforms())
  - `src/shaders/skybox_shader.rs:22-40` — SkyboxShader::new() — insert get_uniforms() before line 40 return
  - `src/shaders/skybox_shader.rs:43-60` — SkyboxShader::default() — insert get_uniforms() before line 60 return
  - `src/shaders/skybox_shader.rs:62-70` — get_uniforms() implementation to call

  **Acceptance Criteria**:
  - [ ] `location_projection_view`, `location_view_matrix`, `location_tex_sampler` all set to non-zero values after construction
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: SkyboxShader::default() initializes uniforms
    Tool: Bash (cargo test)
    Preconditions: None
    Steps:
      1. Add #[test] that calls SkyboxShader::default()
      2. Assert location_projection_view != 0
      3. Assert location_view_matrix != 0  
      4. Assert location_tex_sampler != 0
    Expected Result: All uniform locations are non-zero (valid GL locations)
    Failure Indicators: Any uniform location stays at 0
    Evidence: .sisyphus/evidence/task-1A.1-uniforms.test
  ```

  **Commit**: YES
  - Message: `fix(renderer): add missing get_uniforms() call in SkyboxShader`
  - Files: `src/shaders/skybox_shader.rs`

- [x] 1A.2. Fix skybox depth test (GL_LEQUAL before draw)

  **What to do**:
  - In `src/renderer/render_master.rs`, in the `finish_render` method, before the skybox render call (before line 79 `self.skybox_renderer.render(...)`):
  - Add: `unsafe { gl::DepthFunc(gl::LEQUAL); }`
  - After the skybox render call (after line 79):
  - Restore: `unsafe { gl::DepthFunc(gl::LESS); }`
  - This ensures skybox fragments at maximum depth pass the depth test even when behind geometry

  **Must NOT do**:
  - Do NOT change the rendering order (skybox stays last)
  - Do NOT modify `glClear` or other GL state in the render master

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Two GL state changes at a known location
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 1 — Track A
  - **Blocks**: 1A.3
  - **Blocked By**: 1A.1 (until uniform fix applied)

  **References**:
  - `src/renderer/render_master.rs:62-84` — finish_render method where skybox is rendered
  - `src/renderer/render_master.rs:79` — line where `self.skybox_renderer.render(...)` is called
  - `src/context.rs:61` — pattern for GL state changes with unsafe blocks

  **Acceptance Criteria**:
  - [ ] `gl::DepthFunc(gl::LEQUAL)` appears before skybox draw
  - [ ] `gl::DepthFunc(gl::LESS)` restored after skybox draw
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: Depth function changed for skybox rendering
    Tool: Bash (cargo build + code verification)
    Steps:
      1. cargo build
      2. grep -A2 "skybox_renderer.render" src/renderer/render_master.rs | grep "LEQUAL"
      3. Assert match found
    Expected Result: LEQUAL appears between skybox render call context
    Failure Indicators: No LEQUAL found near skybox render call
    Evidence: .sisyphus/evidence/task-1A.2-depth-fix.diff
  ```

  **Commit**: YES (grouped with 1A.1)
  - Message: `fix(renderer): set depth func to LEQUAL for skybox pass`
  - Files: `src/renderer/render_master.rs`

- [x] 1A.3. Write unit test for skybox uniform initialization

  **What to do**:
  - Create `src/shaders/skybox_shader_test.rs` (or `#[cfg(test)]` module in `skybox_shader.rs`)
  - Test that `SkyboxShader::default()` returns a shader with non-zero uniform locations
  - Test that `SkyboxShader::new()` (if accessible) also initializes uniforms
  - Note: Since OpenGL context IS required for actual shader compilation, this test may need to be an integration test that runs after context init, OR we test the code structure (assert `get_uniforms()` is called) rather than the runtime values
  - Alternative approach: Add a flag field `uniforms_loaded: bool` that gets set in `get_uniforms()`, test the flag

  **Must NOT do**:
  - Do NOT require a running OpenGL context for unit tests (use structural assertions instead)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Test writing following established patterns
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 1 — Track A
  - **Blocked By**: 1A.1, 1A.2

  **References**:
  - `src/shaders/chunk_shader.rs:24-32` — pattern for Default that calls get_uniforms
  - `src/shaders/skybox_shader.rs:43-60` — SkyboxShader struct with uniform location fields

  **Acceptance Criteria**:
  - [ ] At least 1 test passes verifying uniform initialization
  - [ ] `cargo test` passes

  **QA Scenarios**:
  ```
  Scenario: Skybox shader uniform initialization test passes
    Tool: Bash
    Steps:
      1. cargo test skybox_shader 2>&1
      2. Assert exit code is 0
      3. Assert output contains "test result: ok" for skybox tests
    Expected Result: Tests pass
    Evidence: .sisyphus/evidence/task-1A.3-test-output.log
  ```

  **Commit**: YES
  - Message: `test(shaders): add skybox uniform initialization tests`
  - Files: `src/shaders/skybox_shader.rs`, `src/shaders/skybox_shader_test.rs` (if separate file)

### Wave 1 — Bug Fixes (Track B: Frustum Culling)

- [x] 1B.1. Fix matrix.rs:53 wrong Z rotation axis

  **What to do**:
  - In `src/maths/matrix.rs`, line 53: change the rotation axis for Z rotation
  - Current (wrong): `let rotation_z = glm::rotate(&glm::Mat4::identity(), rotation.z, &glm::vec3(1.0, 0.0, 1.0));`
  - Fixed (correct): `let rotation_z = glm::rotate(&glm::Mat4::identity(), rotation.z, &glm::vec3(0.0, 0.0, 1.0));`
  - This bug affects ALL rendering (view matrix used for both projection and frustum calculation)

  **Must NOT do**:
  - Do NOT change any other rotation axes
  - Do NOT change the rotation order (XYZ is correct)
  - Do NOT touch the projection matrix construction

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: One-character change (1→0), but high impact
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 1 — Track B
  - **Blocks**: 1B.4, 1B.5 (tests depend on corrected matrix)

  **References**:
  - `src/maths/matrix.rs:41-58` — `make_view_matrix` function
  - `src/maths/matrix.rs:53` — exact line to fix
  - `src/camera.rs:83` — where make_view_matrix is called for camera

  **Acceptance Criteria**:
  - [ ] Line 53 uses `glm::vec3(0.0, 0.0, 1.0)` for rotation.z axis
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: Z rotation uses correct axis
    Tool: Bash (ast_grep or grep)
    Steps:
      1. grep -n "rotation_z" src/maths/matrix.rs
      2. Assert line contains "vec3(0.0, 0.0, 1.0)" not "vec3(1.0, 0.0, 1.0)"
    Expected Result: Correct Z-axis vector found
    Evidence: .sisyphus/evidence/task-1B.1-matrix-fix.diff
  ```

  **Commit**: YES (grouped with all 1B frustum fixes)
  - Message: `fix(maths): correct Z rotation axis in view matrix`
  - Files: `src/maths/matrix.rs`

- [x] 1B.2. Fix frustum.rs:74 wrong left-plane matrix element

  **What to do**:
  - In `src/maths/frustum.rs`, line 74: fix the left plane distance calculation
  - Current (wrong): Uses `mat[(0, 2)]` — this reads the wrong column element
  - Fixed (correct): Use `mat[(0, 3)]` — consistent with all other plane calculations
  - The pattern: For plane N, distance = mat[(3,3)] ± mat[(N,3)]
  - Left plane (N=0): should use mat[(0,3)], all others already use (_,3)

  **Must NOT do**:
  - Do NOT change any other plane calculations (they're correct)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: One-element fix (2→3)
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 1 — Track B
  - **Blocks**: 1B.4, 1B.5

  **References**:
  - `src/maths/frustum.rs:70-80` — left plane extraction block
  - `src/maths/frustum.rs:74` — exact line: `let distance = mat[(3, 3)] + mat[(0, 2)];` → change `(0, 2)` to `(0, 3)`
  - `src/maths/frustum.rs:86` — right plane for comparison: `mat[(3, 3)] - mat[(0, 3)]` (correct pattern)

  **Acceptance Criteria**:
  - [ ] Left plane distance uses `mat[(0, 3)]` not `mat[(0, 2)]`
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: Left plane uses correct matrix element
    Tool: Bash (grep)
    Steps:
      1. grep -n "left" src/maths/frustum.rs -A5
      2. Assert "mat[(0, 3)]" appears in left plane calculation
      3. Assert "mat[(0, 2)]" does NOT appear
    Expected Result: Correct matrix indices for left plane
    Evidence: .sisyphus/evidence/task-1B.2-frustum-fix.diff
  ```

  **Commit**: YES (grouped with 1B.1)
  - Message: `fix(maths): correct left plane matrix element in frustum extraction`
  - Files: `src/maths/frustum.rs`

- [x] 1B.3. Fix frustum.rs:120-121 premature intersection return

  **What to do**:
  - In `src/maths/frustum.rs`, the `is_box_in_frustum` function (lines ~116-125):
  - Current bug: `vp > 0 && vn < 0` case returns `true` immediately — but it should CONTINUE checking remaining planes
  - Change: Remove `return true;` on the intersection case, replace with `continue;`
  - The function should only return `false` when vp < 0 (outside), and return `true` ONLY after all 6 planes pass
  - Corrected logic:
    ```rust
    if vp < 0 { return false; }           // outside this plane → cull
    else if vn < 0 { continue; }          // intersects plane → check others
    // else: inside plane → continue to next
    ```
    At end of loop (all 6 planes checked without rejection): `return true;`

  **Must NOT do**:
  - Do NOT change the vp/vn calculation (those are correct)
  - Do NOT change the plane iteration order

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Logic fix at known location
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 1 — Track B
  - **Blocks**: 1B.4, 1B.5

  **References**:
  - `src/maths/frustum.rs:116-125` — `is_box_in_frustum` method
  - `src/physics/aabb.rs` — `get_vp`, `get_vn` methods used by frustum test
  - `src/world/chunk/chunk.rs:62-69,76-87` — where frustum culling is called (mesh gen + render)

  **Acceptance Criteria**:
  - [ ] `is_box_in_frustum` iterates all 6 planes before returning true
  - [ ] No `return true` before loop completes (except at end)
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: Frustum culling checks all 6 planes
    Tool: Bash (code review)
    Steps:
      1. Read frustum.rs lines 116-130
      2. Assert no "return true" before the loop closing brace
      3. Assert only "return false" inside loop, "return true" after loop
    Expected Result: Correct loop structure
    Evidence: .sisyphus/evidence/task-1B.3-culling-fix.diff
  ```

  **Commit**: YES (grouped with 1B.1, 1B.2)
  - Message: `fix(maths): fix early return in frustum box intersection test`
  - Files: `src/maths/frustum.rs`

- [x] 1B.4. Write frustum plane extraction unit tests

  **What to do**:
  - Add `#[cfg(test)] mod tests` in `src/maths/frustum.rs`
  - Test: Create a known projection-view matrix (e.g., identity or simple transform)
  - Construct a `ViewFrustum` from it
  - Assert plane normals are correctly oriented (all point inward)
  - Assert plane distances are within expected ranges
  - Verify all 6 planes (left/right/bottom/top/near/far) are extractable

  **Must NOT do**:
  - Do NOT depend on OpenGL context (use pure nalgebra-glm math)
  - Do NOT test with random matrices (use deterministic, known-good matrices)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Writing math verification tests requires careful correctness
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 1 — Track B
  - **Blocked By**: 1B.1, 1B.2, 1B.3

  **References**:
  - `src/maths/frustum.rs:50-80` — frustum extraction logic to test
  - `src/maths/matrix.rs:41-58` — matrix construction to use in test inputs

  **Acceptance Criteria**:
  - [ ] At least 3 test functions pass
  - [ ] Tests cover: all 6 planes extracted, normals normalized, correct inward orientation
  - [ ] `cargo test` passes all frustum tests

  **QA Scenarios**:
  ```
  Scenario: Frustum extraction unit tests pass
    Tool: Bash
    Steps:
      1. cargo test maths::frustum 2>&1
      2. Assert exit code is 0
      3. Assert all tests in frustum module pass
    Expected Result: All frustum tests pass
    Evidence: .sisyphus/evidence/task-1B.4-frustum-tests.log
  ```

  **Commit**: YES
  - Message: `test(maths): add frustum plane extraction unit tests`
  - Files: `src/maths/frustum.rs`

- [x] 1B.5. Write frustum AABB culling unit tests

  **What to do**:
  - Add tests for `is_box_in_frustum` in `src/maths/frustum.rs` test module
  - Test cases:
    - AABB completely inside frustum → should return true
    - AABB completely outside frustum (in front of near plane) → should return false
    - AABB completely outside frustum (behind far plane) → should return false
    - AABB intersecting a frustum plane → should return true
    - AABB straddling 2+ frustum planes → should return true
  - Construct a known frustum (e.g., symmetric 90° FOV, near=0.1, far=1000)
  - Position AABBs at known positions relative to frustum planes

  **Must NOT do**:
  - Do NOT use the actual camera frustum from running game (use synthetic)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Math-heavy test design with corner cases
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 1 — Track B
  - **Blocked By**: 1B.3 (culling logic fix), 1B.1 (matrix fix)

  **References**:
  - `src/maths/frustum.rs:100-130` — `is_box_in_frustum` method
  - `src/physics/aabb.rs:35-60` — AABB struct with get_vp/get_vn methods

  **Acceptance Criteria**:
  - [ ] 5+ test cases pass covering inside/outside/intersection
- [x] `cargo test` passes all tests
  - [ ] Boxes known to be IN frustum return true; known OUT return false

  **QA Scenarios**:
  ```
  Scenario: AABB frustum culling tests pass
    Tool: Bash
    Steps:
      1. cargo test maths::frustum 2>&1
      2. Assert exit code is 0
      3. Assert output shows multiple test cases passed
    Expected Result: All culling tests pass
    Evidence: .sisyphus/evidence/task-1B.5-culling-tests.log
  ```

  **Commit**: YES
  - Message: `test(maths): add AABB frustum culling unit tests`
  - Files: `src/maths/frustum.rs`

### Wave 2 — Mechanical Cleanup

- [x] 2C.1. Replace `extern crate nalgebra_glm as glm;` with `use nalgebra_glm as glm;`

  **What to do**:
  - Search all 14 files containing `extern crate nalgebra_glm as glm;`
  - Replace each with `use nalgebra_glm as glm;` (at top of file, after license header)
  - Verify each file still compiles (cargo build)
  - Files to modify (from research): `camera.rs`, `chunk.rs`, `chunk_mesh.rs`, `chunk_mesh_builder.rs`, `chunk_section.rs`, `classic_over_world_generator.rs`, `frustum.rs`, `matrix.rs`, `noise_generator.rs`, `ray.rs`, `player.rs`, `aabb.rs`, `render_master.rs`, plus any others found

  **Must NOT do**:
  - Do NOT change any `glm::` usage within the files
  - Do NOT remove the import — only change `extern crate` → `use`

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Search-and-replace across known files
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 2 — Track C
  - **Blocks**: None (all subsequent builds)
  - **Can Run In Parallel**: YES (with Track D and Track E)

  **References**:
  - `src/camera.rs:5` — example `extern crate nalgebra_glm as glm;` to replace

  **Acceptance Criteria**:
  - [ ] Zero files contain `extern crate nalgebra_glm as glm;`
  - [ ] 14+ files contain `use nalgebra_glm as glm;`
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: No extern crate declarations remain
    Tool: Bash
    Steps:
      1. grep -rn "extern crate" src/ 2>&1
      2. Assert zero matches (empty output)
    Expected Result: No extern crate nalgebra_glm found
    Evidence: .sisyphus/evidence/task-2C.1-extern-crate-clean.log
  
  Scenario: Build still passes after use migration
    Tool: Bash
    Steps:
      1. cargo build 2>&1
      2. Assert exit code is 0 and no errors
    Expected Result: Build succeeds
    Evidence: .sisyphus/evidence/task-2C.1-build.log
  ```

  **Commit**: YES
  - Message: `refactor: replace extern crate with use for nalgebra-glm`
  - Files: All 14+ files with the change

- [x] 2D.1-19. Migrate each mod.rs to sibling module_name.rs (19 files)

  **What to do** — for EACH of the 19 `mod.rs` files:
  - Use `git mv src/{path}/mod.rs src/{path_sibling}.rs`
  - Example: `git mv src/world/mod.rs src/world.rs` (world.rs becomes sibling of world/ directory)
  - Verify `cargo build` passes after each batch of 4-5 files
  - Migrated files list (in dependency order):
    1. `src/gl/mod.rs` → `src/gl.rs`
    2. `src/util/mod.rs` → `src/util.rs`
    3. `src/maths/mod.rs` → `src/maths.rs`
    4. `src/input/mod.rs` → `src/input.rs`
    5. `src/physics/mod.rs` → `src/physics.rs`
    6. `src/shaders/mod.rs` → `src/shaders.rs`
    7. `src/texture/mod.rs` → `src/texture.rs`
    8. `src/item/mod.rs` → `src/item.rs`
    9. `src/renderer/mod.rs` → `src/renderer.rs`
    10. `src/states/mod.rs` → `src/states.rs`
    11. `src/player/mod.rs` → `src/player.rs`
    12. `src/world/block/block_types/mod.rs` → `src/world/block/block_types.rs`
    13. `src/world/block/mod.rs` → `src/world/block.rs`
    14. `src/world/chunk/mod.rs` → `src/world/chunk.rs`
    15. `src/world/event/mod.rs` → `src/world/event.rs`
    16. `src/world/generation/biome/mod.rs` → `src/world/generation/biome.rs`
    17. `src/world/generation/structure/mod.rs` → `src/world/generation/structure.rs`
    18. `src/world/generation/mod.rs` → `src/world/generation.rs`
    19. `src/world/mod.rs` → `src/world.rs`

  **Must NOT do**:
  - Do NOT change any Rust source contents (no import path changes)
  - Do NOT use `mv` (use `git mv` so Git tracks the rename)
  - Do NOT delete existing `{name}.rs` files if they conflict (none should exist)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Purely mechanical git mv operations
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 2 — Track D
  - **Can Run In Parallel**: YES — all 19 renames are independent of each other
  - **Blocks**: None

  **References**:
  - Research confirmed zero `use ...::mod::...` imports — no code changes needed
  - `src/main.rs:24-42` — module declarations (these don't reference mod.rs files)

  **Acceptance Criteria**:
  - [ ] Zero `mod.rs` files remain in `src/` tree
  - [ ] All 19 sibling `.rs` files correctly placed
  - [ ] `cargo build` passes after migration
  - [ ] `git status` shows rename operations, not delete+add

  **QA Scenarios**:
  ```
  Scenario: No mod.rs files remain
    Tool: Bash
    Steps:
      1. find src/ -name "mod.rs" | wc -l
      2. Assert output is "0"
    Expected Result: Zero mod.rs files found
    Evidence: .sisyphus/evidence/task-2D.modrs-clean.log

  Scenario: Build passes after migration
    Tool: Bash
    Steps:
      1. cargo build 2>&1
      2. Assert exit code is 0
    Expected Result: Build succeeds
    Evidence: .sisyphus/evidence/task-2D.build.log
  ```

  **Commit**: YES (committed in batches of 4-5 per commit)
  - Message: `refactor: migrate mod.rs to module_name.rs convention (batch N/M)`
  - Files: The renamed files in each batch

- [x] 2E.1. Replace `lazy_static!` with `std::sync::LazyLock`

  **What to do**:
  - Find ALL `lazy_static!` invocations in the codebase
  - Replace with `std::sync::LazyLock` (stable since Rust 1.80)
  - Pattern: `lazy_static! { static ref FOO: Type = init; }` → `static FOO: LazyLock<Type> = LazyLock::new(|| init);`
  - Access: `&FOO` → `&*FOO` or `FOO.deref()`
  - From Cargo.toml, `lazy_static` is a dependency — after all replacements, remove it from Cargo.toml

  **Must NOT do**:
  - Do NOT change the initialization logic
  - Do NOT remove lazy_static from Cargo.toml until ALL usages are migrated

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Pattern replacement with standard library equivalent
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 2 — Track E
  - **Can Run In Parallel**: YES (independent of tracks C and D)

  **References**:
  - `Cargo.toml:10` — `lazy_static = "1.4.0"` dependency
  - `src/world/block/block_id.rs` — expected to have lazy_static usage
  - `src/world/generation/classic_over_world_generator.rs` — may have lazy_static

  **Acceptance Criteria**:
  - [ ] Zero `lazy_static!` invocations remain
  - [ ] `lazy_static` removed from Cargo.toml dependencies
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: No lazy_static usage remains
    Tool: Bash
    Steps:
      1. grep -rn "lazy_static" src/ 2>&1
      2. Assert zero matches
    Expected Result: No lazy_static references in source
    Evidence: .sisyphus/evidence/task-2E.1-lazy-static-clean.log

  Scenario: Build without lazy_static dependency
    Tool: Bash
    Steps:
      1. cargo build 2>&1
      2. Assert exit code is 0
    Expected Result: Build succeeds without lazy_static crate
    Evidence: .sisyphus/evidence/task-2E.1-build.log
  ```

  **Commit**: YES
  - Message: `refactor: replace lazy_static with std::sync::LazyLock`
  - Files: All files with lazy_static, `Cargo.toml`

### Wave 3 — Foundation Refactoring (low-dependency modules)

- [x] 3.1. Refactor `util/unsafe_cell_wrapper.rs` — scope unsafe tightly

  **What to do**:
  - Keep the `UnsafeCellWrapper<T>` struct (still needed for threading)
  - Add `// SAFETY:` documentation explaining WHY `unsafe impl Send/Sync` is sound:
    - The wrapper is only used where external synchronization is guaranteed via Mutex/AtomicBool
  - Add `# Safety` doc comments to `get()` and `get_mut()` methods
  - Move Send/Sync impls closer to struct definition for clarity
  - Add a `#[allow(clippy::arc_with_non_send_sync)]` at the usage site if needed

  **Must NOT do**:
  - Do NOT remove UnsafeCellWrapper (threading architecture depends on it)
  - Do NOT change its public API

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Documentation + scoping, no logic changes
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 3
  - **Can Run In Parallel**: YES (with 3.2-3.9, all independent)
  - **Blocked By**: Wave 2

  **References**:
  - `src/util/unsafe_cell_wrapper.rs:42-44` — `unsafe impl Send` and `unsafe impl Sync`
  - `src/util/unsafe_cell_wrapper.rs:17-29` — struct definition and get/get_mut

  **Acceptance Criteria**:
  - [ ] All `unsafe impl` blocks have `// SAFETY:` comments
  - [ ] `cargo build` passes
  - [ ] `cargo clippy` passes (no new warnings)

  **QA Scenarios**:
  ```
  Scenario: SAFETY comments exist on unsafe impls
    Tool: Bash
    Steps:
      1. grep -A3 "unsafe impl" src/util/unsafe_cell_wrapper.rs
      2. Assert output contains "SAFETY:"
    Expected Result: SAFETY comments present
    Evidence: .sisyphus/evidence/task-3.1-safety-docs.log
  ```

  **Commit**: YES
  - Message: `docs(util): add SAFETY documentation to UnsafeCellWrapper`
  - Files: `src/util/unsafe_cell_wrapper.rs`

- [x] 3.2. Refactor `util/random.rs` — OnceLock singleton

  **What to do**:
  - Replace `lazy_static!` (if used) or manual singleton pattern with `std::sync::OnceLock`
  - Pattern: `static RANDOM: OnceLock<Mutex<RandomSingleton>> = OnceLock::new();`
  - Replace `get()` method with `RANDOM.get_or_init(|| Mutex::new(RandomSingleton::new()))`
  - Remove any `static mut` or `Box::leak` for random instance
  - Lock mutex only during random number generation (minimal scope)

  **Must NOT do**:
  - Do NOT change the random number generation algorithm
  - Do NOT change the public API of RandomSingleton

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Singleton replacement with standard library
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 3 — parallel

  **References**:
  - `src/util/random.rs` — entire file
  - `std::sync::OnceLock` — standard library stable since 1.70

  **Acceptance Criteria**:
  - [ ] No unsafe for random singleton creation
  - [ ] `cargo test` for random module passes
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: Random singleton initialization is safe
    Tool: Bash
    Steps:
      1. grep -n "unsafe" src/util/random.rs
      2. Assert zero matches (or only FFI-safe unsafes)
      3. cargo test -- random
    Expected Result: No unsafe in random module, tests pass
    Evidence: .sisyphus/evidence/task-3.2-random-safe.log
  ```

  **Commit**: YES
  - Message: `refactor(util): replace singleton pattern with OnceLock in random`
  - Files: `src/util/random.rs`

- [x] 3.3. Refactor `util/array2d.rs` — idiomatic iterators

  **What to do**:
  - Replace C-style index loops with iterator methods
  - Replace `for i in 0..get(x, y)` pattern with proper get/set using iterators
  - Ensure all public methods return references or slices where appropriate
  - Remove unnecessary `unwrap()` on index bounds (use `get()` returning `Option`)

  **Must NOT do**:
  - Do NOT change the array semantics (2D grid)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Iterator replacement in utility module
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 3 — parallel

  **References**:
  - `src/util/array2d.rs` — entire file

  **Acceptance Criteria**:
  - [ ] `cargo build` passes
  - [ ] No manual index loops in array2d.rs

  **QA Scenarios**:
  ```
  Scenario: Array2D functionality preserved
    Tool: Bash
    Steps:
      1. cargo build 2>&1
      2. Assert exit code is 0
    Expected Result: Build succeeds, behavior unchanged
    Evidence: .sisyphus/evidence/task-3.3-build.log
  ```

  **Commit**: YES
  - Message: `refactor(util): use idiomatic iterators in array2d`
  - Files: `src/util/array2d.rs`

- [x] 3.4. Refactor `util/fps_counter.rs` — remove lifetime propagation

  **What to do**:
  - Remove unnecessary lifetime parameter (if present) from FPSCounter
  - If using SFML Text<'a>, clone the Font instead of borrowing (or use static reference)
  - Use `OnceLock<Font>` pattern instead of static mut for the font
  - This breaks the lifetime chain: `StatePlay<'a>` → no more `'a`

  **Must NOT do**:
  - Do NOT change the FPS display behavior
  - Do NOT change how FPS is calculated

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Lifetime simplification
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 3 — parallel

  **References**:
  - `src/util/fps_counter.rs` — FPSCounter struct
  - `src/states/play_state.rs:19` — `StatePlay<'a>` lifetime
  - `src/player/player.rs:344-345` — Font static mut + Box::leak pattern

  **Acceptance Criteria**:
  - [ ] FPSCounter no longer propagates a lifetime parameter through StatePlay
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: FPSCounter lifetime independent
    Tool: Bash
    Steps:
      1. cargo build 2>&1
      2. Assert no lifetime errors for StatePlay
    Expected Result: Build succeeds without lifetime propagation
    Evidence: .sisyphus/evidence/task-3.4-build.log
  ```

  **Commit**: YES
  - Message: `refactor(util): remove lifetime propagation from fps_counter`
  - Files: `src/util/fps_counter.rs`, `src/states/play_state.rs` (lifetime removal)

- [x] 3.5. Refactor `maths/frustum.rs` — idiomatic + docs

  **What to do**:
  - Convert Doxygen comments (`/// @brief`) to standard Rust doc comments (`///`)
  - Convert enum for plane indices to a clean enum with `From<usize>` impl
  - Replace manual enum-to-index conversions with `as usize` where appropriate
  - Add `#[inline]` where beneficial for hot path functions
  - Preserve the bug fixes from Wave 1 (don't regress)

  **Must NOT do**:
  - Do NOT change frustum plane extraction algorithm
  - Do NOT change culling logic beyond Wave 1 fixes

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Style cleanups on already-fixed code
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 3 — parallel
  - **Blocked By**: Wave 1B (must have bug fixes applied first)

  **References**:
  - `src/maths/frustum.rs` — entire file
  - `src/maths/matrix.rs` — related math module for reference

  **Acceptance Criteria**:
  - [ ] No Doxygen-style comments remain in frustum.rs
  - [ ] All existing tests still pass
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: Frustum tests still pass after style refactoring
    Tool: Bash
    Steps:
      1. cargo test maths::frustum 2>&1
      2. Assert exit code is 0
    Expected Result: All frustum tests pass
    Evidence: .sisyphus/evidence/task-3.5-test.log
  ```

  **Commit**: YES
  - Message: `style(maths): convert frustum module to idiomatic Rust docs`
  - Files: `src/maths/frustum.rs`

- [x] 3.6. Refactor `maths/matrix.rs` — idiomatic + docs

  **What to do**:
  - Convert Doxygen comments to standard Rust doc comments
  - Add doc comments explaining matrix construction purpose (make_projection_matrix, make_view_matrix)
  - Preserve the bug fix from Wave 1B.1 (correct Z rotation axis)
  - Consider adding unit tests for known matrix outputs (identity, simple rotations)

  **Must NOT do**:
  - Do NOT change matrix math logic
  - Do NOT change function signatures

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Documentation + style cleanup
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 3 — parallel
  - **Blocked By**: Wave 1B.1 (bug fix must be applied first)

  **References**:
  - `src/maths/matrix.rs:41-58` — make_view_matrix with previous bug
  - `src/maths/matrix.rs:8-30` — make_projection_matrix

  **Acceptance Criteria**:
  - [ ] No Doxygen comments remain
  - [ ] Bug fix preserved (Z axis is (0,0,1))
  - [ ] `cargo build` passes

  **Commit**: YES
  - Message: `style(maths): convert matrix module to idiomatic Rust docs`
  - Files: `src/maths/matrix.rs`

- [x] 3.7. Refactor `maths/ray.rs` — idiomatic

  **What to do**:
  - Convert Doxygen comments to standard Rust docs
  - Ensure public API uses idiomatic types
  - Remove any manual index tracking

  **Must NOT do**:
  - Do NOT change raycasting algorithm

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Style cleanup
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 3 — parallel
  - **Blocked By**: Wave 2

  **References**:
  - `src/maths/ray.rs` — entire file

  **Acceptance Criteria**:
  - [ ] No Doxygen comments remain in ray.rs
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: Ray module still compiles and works
    Tool: Bash
    Steps:
      1. cargo build 2>&1
      2. Assert exit code is 0
    Expected Result: Build succeeds
    Evidence: .sisyphus/evidence/task-3.7-build.log
  ```

  **Commit**: YES
  - Message: `style(maths): convert ray module to idiomatic Rust`
  - Files: `src/maths/ray.rs`, `src/maths/general_maths.rs`, `src/maths/vector2xz.rs`

- [x] 3.8. Refactor `maths/noise_generator.rs` — OnceLock singleton

  **What to do**:
  - Replace `static mut NOISE_GEN: bool = false;` with `std::sync::OnceLock<NoiseGenerator>`
  - Use `NOISE_GEN_INSTANCE.get_or_init(|| NoiseGenerator::new(parameters))` pattern
  - Remove the manual init-flag pattern (lines 58-81 area)
  - If noise parameters change dynamically, use `OnceLock` + flag for re-init

  **Must NOT do**:
  - Do NOT change noise generation algorithm
  - Do NOT change the Perlin noise implementation

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Singleton replacement
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 3 — parallel
  - **Blocked By**: Wave 2

  **References**:
  - `src/maths/noise_generator.rs:58` — `static mut NOISE_GEN` and surrounding code
  - `src/world/generation/classic_over_world_generator.rs:68` — where noise generator is initialized

  **Acceptance Criteria**:
  - [ ] No `static mut` in noise_generator.rs
  - [ ] `cargo build` passes
  - [ ] Noise generation still works (build + run verification)

  **QA Scenarios**:
  ```
  Scenario: Noise generator safe singleton
    Tool: Bash
    Steps:
      1. grep -n "static mut" src/maths/noise_generator.rs
      2. Assert zero matches
      3. cargo build
    Expected Result: No static mut, build passes
    Evidence: .sisyphus/evidence/task-3.8-noise-safe.log
  ```

  **Commit**: YES
  - Message: `refactor(maths): replace static mut with OnceLock in noise generator`
  - Files: `src/maths/noise_generator.rs`

- [x] 3.9. Refactor `config.rs` + `physics/aabb.rs` — misc cleanup

  **What to do**:
  - Convert Doxygen comments to standard Rust docs in both files
  - In `config.rs`: Use `#[derive(Default)]` or keep `Default` impl as-is (fine)
  - In `physics/aabb.rs`: Simplify math, remove unused methods, convert Doxygen

  **Must NOT do**:
  - Do NOT change AABB collision logic
  - Do NOT change Config field types

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Documentation + minor cleanup
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 3 — parallel
  - **Blocked By**: Wave 2

  **References**:
  - `src/config.rs` — Config struct
  - `src/physics/aabb.rs` — AABB struct and methods

  **Acceptance Criteria**:
  - [ ] No Doxygen comments remain in either file
  - [ ] `cargo build` passes

  **Commit**: YES
  - Message: `style: convert config and aabb to idiomatic Rust docs`
  - Files: `src/config.rs`, `src/physics/aabb.rs`

### Wave 4 — Core Module Refactoring (high-impact, sequential within each module track)

- [x] 4.1. Refactor `world/block/block_id.rs` — From impls

  **What to do**:
  - Replace raw `as` casts with `From`/`TryFrom` implementations
  - Add `From<BlockId> for u8` and `TryFrom<u8> for BlockId`
  - Remove Doxygen comments
  - Ensure all enum→int and int→enum conversions are type-safe

  **Must NOT do**:
  - Do NOT change block ID values
  - Do NOT add new block types

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard trait implementation
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 4
  - **Can Run In Parallel**: YES (with 4.2, 4.3, 4.4 — all in world/block/ but independent)
  - **Blocked By**: Wave 3

  **References**:
  - `src/world/block/block_id.rs:43-58` — enum/int conversion boilerplate
  - `src/item/material.rs:58-87` — similar pattern for reference

  **Acceptance Criteria**:
  - [ ] `From<BlockId> for u8` implemented
  - [ ] `TryFrom<u8> for BlockId` implemented
  - [ ] `cargo build` passes
  - [ ] All existing block ID conversions work

  **QA Scenarios**:
  ```
  Scenario: BlockId conversions are type-safe
    Tool: Bash
    Steps:
      1. cargo build 2>&1
      2. Assert exit code is 0
      3. Add #[test] for From/TryFrom roundtrips
      4. cargo test block_id
    Expected Result: Build succeeds, tests pass
    Evidence: .sisyphus/evidence/task-4.1-block-id.log
  ```

  **Commit**: YES
  - Message: `refactor(block): add From/TryFrom impls for BlockId`
  - Files: `src/world/block/block_id.rs`

- [x] 4.2. Refactor `world/block/block_data.rs` — enum methods

  **What to do**:
  - Replace large match/switch blocks with methods on enums or table-driven parsing
  - The block data parser (lines 129-196) uses C-style state machine — restructure into methods on parsing state
  - Remove Doxygen comments
  - Reduce `unwrap()` calls by returning `Result` from parsing methods

  **Must NOT do**:
  - Do NOT change what block data is parsed
  - Do NOT change the block_data.txt format

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Restructuring parser logic requires careful behavior preservation
  - **Skills**: []

  **Parallelization**:
  - **Parallel Group**: Wave 4
  - **Can Run In Parallel**: YES (with 4.1, 4.3, 4.4 — independent)
  - **Blocked By**: Wave 3

  **References**:
  - `src/world/block/block_data.rs:129-196` — large match block
  - `src/world/block/block_data.rs:73-92` — enum/int bridging

  **Acceptance Criteria**:
  - [ ] Block data parsing produces same results as before
  - [ ] `cargo build` passes
  - [ ] `cargo test` passes (add TDD tests first)

  **QA Scenarios**:
  ```
  Scenario: Block data parsing unchanged after refactor
    Tool: Bash
    Steps:
      1. cargo test block_data 2>&1
      2. Assert all tests pass
      3. Assert exit code is 0
    Expected Result: Tests pass, parsing behavior preserved
    Evidence: .sisyphus/evidence/task-4.2-block-data-test.log
  ```

  **Commit**: YES
  - Message: `refactor(block): restructure block_data parser with enum methods`
  - Files: `src/world/block/block_data.rs`

- [x] 4.3. Refactor `world/block/block_database.rs` — OnceLock singleton

  **What to do**:
  - Replace `static mut INSTANCE_PTR: *mut BlockDatabase` + `Box::leak` with `OnceLock<BlockDatabase>`
  - `static INSTANCE: OnceLock<BlockDatabase> = OnceLock::new();`
  - `instance()` → `INSTANCE.get_or_init(|| BlockDatabase::new())`
  - Remove all `unsafe` from singleton creation

  **Must NOT do**: Do NOT change block registration/query API

  **Recommended Agent Profile**: `quick`
  **Parallelization**: Wave 4 — parallel (independent of 4.1, 4.2, 4.4)
  **Blocked By**: Wave 3

  **References**: `src/world/block/block_database.rs:24,52-58`

  **Acceptance Criteria**:
  - [ ] Zero `unsafe` for singleton creation in block_database.rs
  - [ ] `cargo build` passes

  **QA Scenarios**:
  ```
  Scenario: BlockDatabase singleton is safe
    Tool: Bash
    Steps:
      1. grep -n "static mut\|Box::leak\|INSTANCE_PTR" src/world/block/block_database.rs
      2. Assert zero matches
      3. cargo build
    Expected Result: No unsafe singleton, build passes
    Evidence: .sisyphus/evidence/task-4.3-block-db-safe.log
  ```

  **Commit**: YES — `refactor(block): replace singleton with OnceLock in block_database`
  - Files: `src/world/block/block_database.rs`

- [x] 4.4. Refactor `world/block/chunk_block.rs` — newtype

  **What to do**:
  - Replace `type BlockType = u8;` with `struct BlockType(u8);`
  - Add `From<BlockType> for u8`, `TryFrom<u8> for BlockType`
  - Update all usages across codebase

  **Must NOT do**: Do NOT change block semantics
  **Recommended Agent Profile**: `quick`
  **Parallelization**: Wave 4 — parallel; **Blocks**: 4.8
  **References**: `src/world/block/chunk_block.rs:23-25`

  **Acceptance Criteria**:
  - [ ] `BlockType` is a newtype struct
  - [ ] `cargo build` passes

  **Commit**: YES — `refactor(block): convert BlockType alias to newtype struct`
  - Files: `src/world/block/chunk_block.rs` + updated usages

- [x] 4.5. Refactor `player/player.rs` — replace static mut globals

  **What to do**:
  - Replace `static mut FONT`, `USE_MOUSE_KEY_PTR`, `LAST_MOUSE_POSITION_PTR` with `OnceLock` variants
  - Remove lifetime `'a` from `Player<'a>` (depends on FPSCounter fix from 3.4)
  - Add `// SAFETY:` for any remaining unsafe

  **Must NOT do**: Do NOT change player movement/hotbar logic
  **Recommended Agent Profile**: `deep`
  **Parallelization**: Wave 4 — sequential within player; **Blocked By**: 3.4; **Blocks**: 4.11
  **References**: `src/player/player.rs:268-313,342-363`

  **Acceptance Criteria**:
  - [ ] Zero `static mut` + `Box::leak` in player.rs
  - [ ] `Player` no longer has lifetime `'a`
  - [ ] `cargo build` + `cargo test` pass

  **Commit**: YES (multiple commits) — `refactor(player): replace static mut globals with OnceLock`
  - Files: `src/player/player.rs`, `src/states/play_state.rs`

- [x] 4.6. Refactor `camera.rs` — safe entity tracking

  **What to do**:
  - Replace `PtrConstEntity` raw pointer with owned position copy
  - Store `(glm::Vec3, glm::Vec3)` for position/rotation, update each frame via `update_from_entity(&Entity)`
  - Remove `unsafe impl Send for PtrConstEntity`
  - Replace `hook_entity()` with `update_from_entity()`

  **Must NOT do**: Do NOT change camera matrix construction
  **Recommended Agent Profile**: `unspecified-high`
  **Parallelization**: Wave 4; **Blocked By**: Wave 3, 4.5; **Blocks**: 4.11
  **References**: `src/camera.rs:25,30,43,56,77,90-92`

  **Acceptance Criteria**:
  - [ ] No `PtrConstEntity`, no `unsafe impl Send` in camera.rs
  - [ ] `cargo build` passes

  **Commit**: YES — `refactor(camera): replace PtrConstEntity with safe entity tracking`
  - Files: `src/camera.rs`, `src/states/play_state.rs`

- [x] 4.7. Refactor `world/event/player_dig_event.rs` — safe player ref

  **What to do**:
  - Replace `PtrMutPlayer` raw pointer wrapper with `&mut Player` (scoped lifetime)
  - Replace `unsafe impl Send for PtrMutPlayer` with proper Send-safe design
  - Remove `as *mut Player<'static>` casts

  **Must NOT do**: Do NOT change dig/place event logic
  **Recommended Agent Profile**: `quick`
  **Parallelization**: Wave 4 — parallel; **Blocked By**: 4.5
  **References**: `src/world/event/player_dig_event.rs:28-60`

  **Acceptance Criteria**:
  - [ ] No raw player pointers in play_dig_event.rs
  - [ ] `cargo build` passes

  **Commit**: YES — `refactor(event): replace raw player pointer with safe reference`
  - Files: `src/world/event/player_dig_event.rs`

- [x] 4.8. Refactor `world/chunk/chunk_section.rs` — scope unsafe

  **What to do**:
  - Replace `unsafe { &mut *self.p_world.get() }` with properly scoped `unsafe` block with `// SAFETY:` comment
  - Keep UnsafeCellWrapper access but document WHY each deref is safe at each call site
  - Reduce unsafe block scope to single-line dereference

  **Must NOT do**: Do NOT remove world backref (architecture depends on it)
  **Recommended Agent Profile**: `unspecified-high`
  **Parallelization**: Wave 4
  **References**: `src/world/chunk/chunk_section.rs:44,67,114,136,182,194`

  **Acceptance Criteria**:
  - [ ] Every `unsafe` block has `// SAFETY:` justification
  - [ ] `cargo build` passes

  **Commit**: YES — `refactor(chunk): add SAFETY documentation to chunk_section`
  - Files: `src/world/chunk/chunk_section.rs`

- [x] 4.9. Refactor `world/chunk/chunk_manager.rs` — remove raw ptr cast

  **What to do**:
  - Replace `unsafe { (*ptr).as_mut() }` at line 102 with a safe accessor method
  - The terrain generator can be wrapped in `Box<T>` with proper ownership, or use `&mut` directly

  **Must NOT do**: Do NOT change chunk loading/unloading logic
  **Recommended Agent Profile**: `quick`
  **Parallelization**: Wave 4
  **References**: `src/world/chunk/chunk_manager.rs:101-102`

  **Acceptance Criteria**:
  - [ ] No raw pointer cast remaining
  - [ ] `cargo build` passes

  **Commit**: YES — `refactor(chunk): remove raw pointer cast in chunk_manager`
  - Files: `src/world/chunk/chunk_manager.rs`

- [x] 4.10. Refactor `world/world.rs` — replace raw ptr updates

  **What to do**:
  - Replace `chunk_updates: HashMap<IVec3, *mut ChunkSection>` with safe wrapper (e.g., `HashMap<IVec3, usize>` indexing into a Vec, or `Arc<Mutex<ChunkSection>>`)
  - Replace explicit `drop(lock)` calls with RAII scope blocks
  - Fix `is_running` in Drop: use `store(false, Ordering::Release)` instead of `get_mut()`
  - Add `// SAFETY:` for every remaining unsafe

  **Must NOT do**: Do NOT change threading model; Do NOT change chunk loading algorithm
  **Recommended Agent Profile**: `deep`
  **Parallelization**: Wave 4 — sequential; **Blocked By**: 4.8, 4.9
  **References**: `src/world/world.rs:50,127-145,333-341,393-397`

  **Acceptance Criteria**:
  - [ ] No raw mutable pointers stored in data structures
  - [ ] `is_running` uses atomic store
  - [ ] `cargo build` passes

  **Commit**: YES — `refactor(world): replace raw pointer updates with safe structures`
  - Files: `src/world/world.rs`

- [x] 4.11. Refactor `states/play_state.rs` — eliminate static mut

  **What to do**:
  - Replace `static mut TIME_ELAPSED` → `OnceLock<AtomicF32>` or thread-local
  - Replace `static mut TIMER_PTR` → `OnceLock<Clock>`
  - Replace `static mut DT_PTR`, `DRAW_GUI`, `DRAW_KEY_PTR` → `OnceLock` equivalents
  - Remove `Box::leak` for Clock/ToggleKey creation
  - Remove lifetime `'a` from `StatePlay<'a>` (depends on 3.4 + 4.5)
  - Replace `Rc<UnsafeCell<Application>>` accesses with proper `RefCell`-like borrowing

  **Must NOT do**: Do NOT change game loop logic; Do NOT change input handling
  **Recommended Agent Profile**: `deep`
  **Parallelization**: Wave 4 — **Blocks many downstream tasks** (4.12, 4.13)
  **Blocked By**: 3.4, 4.5, 4.6
  **References**: `src/states/play_state.rs:52-56,93-96,178-184`

  **Acceptance Criteria**:
  - [ ] Zero `static mut` in play_state.rs
  - [ ] `StatePlay` no longer has lifetime `'a`
  - [ ] `cargo build` + `cargo test` pass

  **Commit**: YES (multiple commits) — `refactor(play_state): eliminate static mut globals`
  - Files: `src/states/play_state.rs`

- [x] 4.12. Refactor `application.rs` — separate Rc<UnsafeCell> init

  **What to do**:
  - In `Application::new()`, separate initialization logic from `Rc<UnsafeCell>` wrapping
  - Push state AFTER Application is fully constructed, removing need for unsafe push_state
  - Replace `unsafe { (*self.camera.get()).update(); }` with safe method using interior mutability
  - Add `// SAFETY:` for any remaining unsafe (global TIME_ELAPSED)

  **Must NOT do**: Do NOT change the frame loop structure
  **Recommended Agent Profile**: `unspecified-high`
  **Parallelization**: Wave 4
  **Blocked By**: 4.6 (camera), 4.11 (play_state)
  **References**: `src/application.rs:17,45,54,57,78,83,95`

  **Acceptance Criteria**:
  - [ ] No `unsafe` for push_state during construction
  - [ ] `cargo build` passes

  **Commit**: YES — `refactor(application): separate initialization from UnsafeCell wrapping`
  - Files: `src/application.rs`

- [x] 4.13. Refactor `main.rs` — safe Application::run_loop()

  **What to do**:
  - Remove `Rc<UnsafeCell<Application>>` pattern in main.rs
  - Application::new() should return `Application` directly (not wrapped)
  - `run_loop()` becomes `app.run_loop()` without unsafe deref
  - Remove Doxygen comments from load_config and main

  **Must NOT do**: Do NOT change config parsing logic
  **Recommended Agent Profile**: `quick`
  **Parallelization**: Wave 4
  **Blocked By**: 4.11, 4.12
  **References**: `src/main.rs:109-113`

  **Acceptance Criteria**:
  - [ ] No `unsafe` in main.rs
  - [ ] `cargo build` + `cargo run` (launches successfully)

  **Commit**: YES — `refactor(main): remove unsafe from Application initialization`
  - Files: `src/main.rs`

### Wave 5 — Polish (idiomatic style across remaining files)

- [x] 5.1. Replace `&Vec<T>` with `&[T]` in model.rs + others

  **What to do**:
  - `model.rs:76,91` and any other files: change `&Vec<GLuint>` → `&[GLuint]`, `&Vec<GLfloat>` → `&[GLfloat]`
  - Search codebase: `grep -rn "&Vec<" src/` to find all occurrences

  **Must NOT do**: Do NOT change the function logic
  **Recommended Agent Profile**: `quick`
  **Parallelization**: Wave 5 — parallel (independent of other polish tasks)

  **Acceptance Criteria**:
  - [ ] Zero `&Vec<T>` in function signatures (only `&[T]`)
  - [ ] `cargo build` passes

  **Commit**: YES — `refactor: replace &Vec<T> params with &[T]`
  - Files: All affected files

- [x] 5.2. Convert index loops to iterators

  **What to do**:
  - `player/player.rs:183-207,349-360` — replace manual index tracking with iterators
  - `world/chunk/chunk_mesh.rs:60-70` — use `enumerate()` pattern
  - `world/chunk/chunk_mesh_builder.rs:69-80` — use iterators
  - `maths/noise_generator.rs:81-90` — use iterators

  **Must NOT do**: Do NOT change mesh generation or iteration order
  **Recommended Agent Profile**: `quick`
  **Parallelization**: Wave 5 — parallel

  **Acceptance Criteria**:
  - [ ] No `for i in 0..len { ... }` manual index patterns in listed files
  - [ ] `cargo build` passes

  **Commit**: YES — `refactor: convert index-based loops to idiomatic iterators`
  - Files: Listed files above

- [x] 5.3. Convert Doxygen comments to standard Rust docs (all files)

  **What to do**:
  - Replace `/// @brief Description` → `/// Description`
  - Replace `/// @param name Description` → document parameter in prose or use `# Arguments` section
  - Replace `/// @return Description` → `/// Returns Description` or integrate into main docs
  - Target ALL remaining files not covered in Waves 3-4 (shader files, renderer files, texture files, world generation files)

  **Must NOT do**: Do NOT change or remove any documentation content
  **Recommended Agent Profile**: `writing`
  **Parallelization**: Wave 5 — can run in parallel with 5.1, 5.2

  **Acceptance Criteria**:
  - [ ] Zero `@brief`, `@param`, `@return` in any `.rs` file
  - [ ] `cargo build` passes

  **Commit**: YES (in batches by directory) — `docs: convert Doxygen comments to standard Rust docs`
  - Files: All remaining files with Doxygen comments

- [x] 5.4. Fix non-idiomatic naming

  **What to do**:
  - Rename `p_world` → `world`, `p_chunk` → `chunk`, `p_player` → `player`, `p_camera` → `camera`
  - Rename `PtrConstEntity` → removed by 4.6
  - Rename `PtrMutPlayer` → removed by 4.7
  - Use `lsp_rename` to safely rename across workspace

  **Must NOT do**: Do NOT rename public API types that are part of the stable interface
  **Recommended Agent Profile**: `quick`
  **Parallelization**: Wave 5 — parallel

  **Acceptance Criteria**:
  - [ ] No Hungarian-notation field names (`p_*`)
  - [ ] `cargo build` passes

  **Commit**: YES — `refactor: use idiomatic Rust naming conventions`
  - Files: All affected

- [x] 5.5. Add `#![warn(unsafe_code)]` attribute with FFI exceptions

  **What to do**:
  - Add `#![warn(unsafe_code)]` to `main.rs` (or better, `lib.rs` if we create one)
  - Add `#[allow(unsafe_code)]` on specific modules where unsafe is necessary (GL FFI calls, SFML calls)
  - This ensures future code additions are checked for unnecessary unsafe

  **Must NOT do**: Do NOT add deny (use warn) — don't break build
  **Recommended Agent Profile**: `quick`
  **Parallelization**: Wave 5

  **Acceptance Criteria**:
  - [ ] `cargo build` produces unsafe_code warnings (not errors) where intentional, none for new code
  - [ ] Well-documented allow attributes on FFI modules

  **Commit**: YES — `lint: enable unsafe_code warning with FFI exceptions`
  - Files: `src/main.rs`, FFI modules

---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing.
> **Do NOT auto-proceed after verification. Wait for user's explicit approval.**

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists. For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in `.sisyphus/evidence/`. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo build` + `cargo clippy` + `cargo test`. Review all changed files for: `as any`/`@ts-ignore`-equivalents, empty catches, console.log-like debug prints, commented-out code, unused imports. Check AI slop: excessive comments, over-abstraction, generic names. Count remaining `unsafe` blocks.
  Output: `Build [PASS/FAIL] | Clippy [N warn] | Tests [N pass/N fail] | Unsafe remaining [N] | VERDICT`

- [ ] F3. **Real Manual QA** — `unspecified-high` (+ `playwright` skill if UI)
  Start from clean state. `nix develop --command cargo run` — verify:
  1. Skybox visible (not black background)
  2. Frustum culling correct (no visible geometry missing at screen edges)
  3. Game runs 10+ seconds without crash
  4. Block breaking/placing works
  5. Movement/camera works
  Save screenshots to `.sisyphus/evidence/final-qa/`.
  Output: `Visual regressions [N] | Crashes [N] | Skybox [OK/FAIL] | Frustum [OK/FAIL] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **Wave 0**: 1 commit for test infra
- **Wave 1**: 2 commits (Track A: skybox fix + test; Track B: frustum fixes + tests)
- **Wave 2**: 3 commits (extern crate removal, mod.rs migration batches, lazy_static removal)
- **Wave 3**: 9 individual commits (one per task, independent modules)
- **Wave 4**: 13 commits (one per task, some with multi-commit for deep tasks)
- **Wave 5**: 5 commits (one per task)
- **Total**: ~33 commits

Commit format: Conventional Commits — `type(scope): description`

---

## Success Criteria

### Verification Commands
```bash
# Build verification
nix develop --command cargo build 2>&1 | grep -E "^error"  # Expected: no output

# Test verification  
nix develop --command cargo test 2>&1                       # Expected: all pass

# Lint verification
nix develop --command cargo clippy 2>&1                     # Expected: 0 new warnings

# Unsafe count verification (pre-refactor baseline: ~42 non-FFI)
grep -rn "unsafe" src/ | grep -v "gl::\|sfml::\|CStr::from_ptr" | wc -l  # Expected: <10

# mod.rs verification
find src/ -name "mod.rs" | wc -l                            # Expected: 0

# Run smoke test
nix develop --command timeout 10 cargo run 2>&1             # Expected: launches without crash
```

### Final Checklist
- [ ] All "Must Have" present
- [ ] All "Must NOT Have" absent
- [ ] Skybox renders correctly
- [ ] Frustum culling correct
- [ ] Zero `mod.rs` files
- [ ] All non-FFI unsafe eliminated or tightly scoped with SAFETY docs
- [ ] `cargo build` passes in Nix dev shell
- [ ] `cargo test` passes all tests
- [ ] `cargo clippy` reports zero new warnings
- [ ] Game launches and runs without crash (10+ seconds)

