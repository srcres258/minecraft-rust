# AGENTS.md — minecraft-rust

## Build & Run

```bash
cargo run
```

SFML is a **native C++ dependency**. The `sfml` crate requires system libraries (`libsfml-dev` on Debian/Ubuntu, `sfml` on Arch, etc.). If `cargo build` fails with linker errors, install SFML first. See https://crates.io/crates/sfml for platform-specific setup.

No build.rs, no codegen. Pure `cargo` workflow. Edition 2021, `rustc 1.91`.

## Project Architecture

### Entry Point

`src/main.rs` → loads `config.txt` → creates `Application` → enters `run_loop()`.

### State Machine

`Application` owns a stack of `Box<dyn StateBase>` (trait in `states/state_base.rs`). Only `StatePlay` is currently implemented. Each frame calls:
1. `handle_input()` — process keyboard/mouse
2. `update(delta_time)` — physics, world updates, chunk loading
3. `render(&mut RenderMaster)` — submit draw calls
4. Post-frame: `master_renderer.finish_render(window, camera)` flushes to OpenGL

### Rendering Pipeline

OpenGL 3.3 Core Profile via SFML window + `gl` crate + `gl_loader`. Custom GL debug callback enabled (`gl_debug_output` in `context.rs`).

`RenderMaster` aggregates `ChunkRenderer`, `WaterRenderer`, `FloraRenderer`, `SkyboxRenderer`. Each renderer has its own shader (loaded from `Shaders/`). No render graph — direct GL calls.

Shaders are loaded via `shader_loader::load_shaders(name, name)` which reads `Shaders/{name}.vert` and `Shaders/{name}.frag`.

### World System

- **Chunk**: 16×256×16 (`CHUNK_SIZE` = 16, `WATER_LEVEL` = 64 in `world_constants.rs`)
- **ChunkSection**: 16×16×16 sub-chunk, holds the actual mesh data
- **ChunkManager**: multithreaded chunk loading/unloading based on player position
- **Terrain generation**: biome system with tree structures, perlin noise
- **Block raycasting**: `Ray` in `maths/ray.rs`, used in `StatePlay::handle_input` for block breaking/placing

### Key Source Modules

| Module | Purpose |
|---|---|
| `application.rs` | Main game loop, state stack, window events |
| `states/play_state.rs` | Active gameplay state (input, raycasting, world tick) |
| `context.rs` | SFML window creation, OpenGL init, debug callback |
| `renderer/render_master.rs` | Render orchestration, draw batching |
| `world/world.rs` | World owner — chunk management, events, spawn |
| `world/chunk/chunk_manager.rs` | Multithreaded chunk load/unload |
| `camera.rs` | First-person camera, view/projection matrices, frustum culling |
| `player/player.rs` | FPS movement, flying, item hotbar, inventory |
| `physics/aabb.rs` | Axis-aligned bounding box collision |

## Unsafe Patterns (Intentional — Don't "Fix")

This is a game engine port from C++. Multiple patterns exist by design:

1. **Mutable statics**: `TIME_ELAPSED`, `TIMER_PTR`, `DT_PTR`, `DRAW_GUI`, `DRAW_KEY_PTR` in `play_state.rs` are `static mut`. These are game-global singletons.

2. **Box::leak for global singletons**: Clocks and ToggleKeys are leaked to create `'static` mutable references. This is the pattern used instead of `lazy_static` for non-constant globals.

3. **`UnsafeCellWrapper<T>`** (`util/unsafe_cell_wrapper.rs`): Wraps `UnsafeCell<T>` and `unsafe impl Send + Sync`. Used to share `World` and `Camera` across threads via `Arc`. The game explicitly manages synchronization through `Mutex` and `AtomicBool`.

4. **`Rc<UnsafeCell<Application>>`**: Application is wrapped in both `Rc` (single-threaded ref count) and `UnsafeCell`. Shared with `StatePlay` for camera/window access.

5. **`PtrConstEntity`**: Wraps a `*const Entity` with `Deref` + `unsafe impl Send`. Camera hooks into the player's entity pointer for tracking.

**Do not refactor these patterns** without understanding the thread/callback architecture. The game is NOT single-threaded — chunk loading spawns worker threads.

## Config

Runtime config in `config.txt` at repo root (gitignored). Auto-generated if missing. Format: key-value pairs separated by spaces:
```
renderdistance 8
fullscreen 0
windowsize 1600 900
fov 105
```
Parsed by `main.rs::load_config()`. Config struct in `config.rs` with `Default` impl.

## Testing

**There are no tests.** No `#[test]` annotations, no `tests/` directory. Don't try to run `cargo test` expecting anything.

## Code Conventions

- **License header**: Every `.rs` file starts with `// SPDX-License-Identifier: Apache-2.0` followed by a copyright block. Follow this when adding new files.
- **Math**: `nalgebra-glm` aliased as `glm` via `extern crate nalgebra_glm as glm;` in files that use it.
- **SFML types**: Used for windowing and event handling. Vector types from SFML (`Vector2i`, `Vector3i`) coexist with `nalgebra_glm` vectors — be aware of type conversions.
- **No rustfmt config**: No `.rustfmt.toml`. Default rustfmt rules apply.
- **Error handling**: Uses `unwrap()` / `expect()` liberally — no `thiserror` or `anyhow`. This is a game, not a library.
- **Comments**: `/// @brief` doxygen-style doc comments used on structs and significant functions.
