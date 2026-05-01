## BLOCKER: Task 4.6 - Camera PtrConstEntity removal

This task requires coordinated changes across 3 files (camera.rs, play_state.rs, application.rs) 
due to the UnsafeCellWrapper<Camera> pattern shared between Application and StatePlay. 
Subagents consistently time out trying to handle the cross-module borrow semantics.

Resolution: Human developer should:
1. Remove PtrConstEntity from camera.rs
2. Add update_from_entity(&Entity) to Camera
3. In play_state.rs update(), sync camera position after player.update()
4. Remove unsafe deref in camera.update()

