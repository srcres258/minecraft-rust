## 2026-05-01
- Replacing Doxygen tags in Rust doc comments is safest when tags are removed line-by-line; a broad regex can accidentally eat the newline after `@param` lines.
- `@param name` lines with no extra prose should become plain doc text on their own line (`/// name`) before the function signature.
- When refactoring mesh builders, iterating with `enumerate()` over the backing array is simpler than manually advancing a parallel iterator.
- Renaming `p_` fields is easiest when comments and shadowing locals are updated in the same patch, especially inside `unsafe` blocks.
- For `Drop` on a shared atomic flag, `store(false, Ordering::Release)` matches the worker loop's `load(Ordering::Acquire)` shutdown pattern.
- SAFETY comments are best placed immediately before the `unsafe` block that dereferences raw chunk pointers so the invariant is obvious at the call site.
