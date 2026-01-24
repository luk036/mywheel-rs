# AGENTS.md - Coding Guidelines for mywheel-rs

This file provides guidelines for agentic coding agents working in this Rust repository.

## Build and Test Commands

### Essential Commands
- **Build project**: `cargo build`
- **Build release**: `cargo build --release`
- **Run tests**: `cargo test --all-features --workspace`
- **Run single test**: `cargo test <test_name>` (e.g., `cargo test test_robin`)
- **Run tests in specific module**: `cargo test <module_name>` (e.g., `cargo test dllist`)
- **Check formatting**: `cargo fmt --all -- --check`
- **Format code**: `cargo fmt --all`
- **Clippy linting**: `cargo clippy --all-targets --all-features --workspace`
- **Generate docs**: `cargo doc --no-deps --document-private-items --all-features --workspace --examples`

### CI Requirements
The project uses CI pipelines that require:
- All tests pass on stable, beta, and nightly toolchains
- Code formatting with `rustfmt` must pass
- Clippy checks must pass with no warnings
- Documentation must build without warnings (`RUSTDOCFLAGS=-D warnings`)

## Code Style Guidelines

### Project Structure
- **Library root**: `src/lib.rs` - exports all public modules
- **Modules**: Each major data structure has its own module file:
  - `array_like.rs` - RepeatArray and ShiftArray implementations
  - `bpqueue.rs` - Bounded Priority Queue
  - `dllist.rs` - Doubly linked list and nodes
  - `map_adapter.rs` - Map-like adapter for Vec
  - `robin.rs` - Round robin scheduling
- **Main entry**: `src/main.rs` - simple "Hello, world!" for testing

### Naming Conventions
- **Struct names**: `PascalCase` (e.g., `BPQueue`, `Dllink`, `RepeatArray`)
- **Function names**: `snake_case` (e.g., `new`, `appendleft`, `is_empty`)
- **Constants**: `SCREAMING_SNAKE_CASE` (rarely used in this codebase)
- **Module names**: `snake_case` (matching file names)
- **Variables**: `snake_case` (e.g., `bucket`, `sentinel`, `curr`)

### Import Organization
- **Standard library imports**: Use `std::` prefix explicitly
- **Internal imports**: Use `crate::` for intra-crate imports
- **External dependencies**: Minimal - only `svgbobdoc` for documentation
- **Order**: External crates → std library → local modules

Example from codebase:
```rust
use crate::dllist::{Dllink, Dllist};
use core::cmp::Ordering;
```

### Documentation Style
- **Public items**: Must have rustdoc comments starting with `///`
- **Examples**: Include `# Examples` sections with working code
- **Props/Returns**: Document parameters and return values clearly
- **svgbob diagrams**: Use `#[doc = svgbobdoc::transform!()]` for complex structures
- **Private items**: Documented in CI via `--document-private-items`

Example:
```rust
/// Construct a new Dllink object
///
/// # Examples
///
/// ```rust
/// use mywheel_rs::dllist::Dllink;
/// let a = Dllink::new(3);
/// assert_eq!(a.data, 3);
/// ```
```

### Type Safety and Generics
- **Use generics**: Prefer generic implementations where appropriate
- **Bounds**: Specify trait bounds clearly (e.g., `T: Default + Clone`)
- **Lifetime parameters**: Use explicit lifetimes for references (`'a`)
- **Copy vs Clone**: Use `Copy` for trivial types, `Clone` for complex ones

### Error Handling
- **Panics**: Use `assert!` for precondition checks in public APIs
- **Assertions**: Common for bounds checking and algorithm invariants
- **Result types**: Not extensively used in this codebase (focus on algorithms)
- **Unsafe**: Used sparingly for pointer operations in linked lists

### Testing Patterns
- **Module structure**: Each module has `#[cfg(test)] mod tests`
- **Test naming**: `test_<module_name>` or descriptive names
- **Test structure**: 
  - Basic construction tests
  - Operation tests with assertions
  - Edge case tests (including `#[should_panic]`)
- **Example test**:
```rust
#[test]
fn test_dllist() {
    let mut a = Dllist::new(3);
    a.clear();
    assert!(a.is_empty());
    // ... more test logic
}
```

### Performance Considerations
- **Inline hints**: Use `#[inline]` for small, frequently called functions
- **Unsafe optimization**: Used for pointer arithmetic in linked lists
- **Memory layout**: Focus on efficient data structures
- **Iterator patterns**: Implement custom iterators for collections

### Algorithmic Patterns
- **FM algorithm focus**: Many structures support Fiduccia-Mattheyses partitioning
- **Bounded ranges**: Common use of integer ranges with offsets
- **Sentinel nodes**: Used extensively to avoid boundary checks
- **Non-owning structures**: Most collections don't own their nodes for flexibility

## Code Quality Standards

### Must-Do Requirements
- **All public functions must have rustdoc comments**
- **Include usage examples in documentation**
- **Run `cargo fmt` before committing**
- **Ensure `cargo clippy` passes without warnings**
- **Add tests for new functionality**
- **Update CHANGELOG.md for API changes**

### Must-Not-Do Rules
- **No `unwrap()` without context in production code**
- **No `todo!()` or `unimplemented!()` in committed code**
- **No explicit `panic!()` except in tests**
- **No dead code warnings should remain**
- **No TODO comments left in final implementation**

### Dependencies
- **Minimal external dependencies** - only `svgbobdoc` for documentation
- **Prefer standard library** when possible
- **Justification required** for adding new dependencies

## Development Workflow

1. **Setup**: `cargo test` to ensure everything builds and passes
2. **Development**: Write code following the patterns above
3. **Testing**: `cargo test --all-features --workspace`
4. **Linting**: `cargo clippy --all-targets --all-features --workspace`
5. **Formatting**: `cargo fmt --all`
6. **Documentation**: `cargo doc --no-deps --document-private-items --all-features --workspace --examples`

## Repository-Specific Context

This is a "reinventing the wheel" project focusing on fundamental data structures and algorithms. The code prioritizes:
- **Educational clarity** over micro-optimizations
- **Algorithm correctness** with extensive testing
- **Clean abstractions** for fundamental operations
- **Documentation** with visual diagrams (svgbob)

The codebase implements custom versions of common structures to understand their internals deeply.