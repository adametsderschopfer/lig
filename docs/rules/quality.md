# Rust quality and verification

Follow these rules when [AGENTS.md](../../AGENTS.md) directs you here.

## Required tests

- Add meaningful tests for new functionality and behavior changes.
- Add regression tests for bug fixes.
- Test observable behavior, boundaries, and errors.
- Do not copy implementation logic into tests or limit tests to constant comparisons.
- Keep tests separate from production source files.
- Use `iced_test` for interface tests without a display or GPU.
- Check window settings and Iced styles when changing opacity.
- Preserve assertions and coverage when moving tests.
- Do not disable or weaken tests to pass checks.

## Required checks

Run these commands before completion, commits, and pushes:

```sh
make check
make build
```

`make check` runs these checks:

1. `cargo fmt --all -- --check`
2. `dprint check`
3. `cargo clippy --locked --all-targets --all-features -- -D warnings`
4. `cargo check --locked --all-targets --all-features`
5. `cargo test --locked --all-targets --all-features`

Run static checks after all edits. Fix failures before committing or pushing. Use `make fmt` to
apply formatting. Run formatting checks afterward. Use `make run` or `cargo run --locked` to start
the application. Do not introduce Node.js or npm for application development. Keep shared Make
commands independent of Unix shell utilities. Keep equivalent Cargo and dprint commands available on
every supported platform. Commit dependency changes with `Cargo.lock`. Pin formatter plugins in
`dprint.json`. Check CI results on every supported platform. Report unavailable checks explicitly.

macOS is the primary development platform. Preserve existing Windows and Linux compatibility. Cargo
forbids unsafe code in this crate.

## Rust Code Quality

Follow idiomatic Rust practices.

- Prefer stable Rust.
- Follow Rust API Guidelines.
- Use descriptive names.
- Prefer small, focused modules and functions.
- Use `Result` and meaningful error types.
- Avoid `unwrap()` and `expect()` in recoverable production paths.
- Avoid unnecessary cloning and allocations.
- Prefer borrowing when appropriate.
- Avoid excessive `Arc<Mutex<T>>` and shared mutable state.
- Use async only when beneficial.
- Do not add unsafe code. Cargo forbids it in this crate.
- Prefer established, actively maintained libraries.
- Minimize unnecessary dependencies.
- Pin dependencies reproducibly and commit `Cargo.lock`.

Do not optimize prematurely. Measure performance before introducing complex optimizations.

## Testing & Verification

Write meaningful automated tests.

Prioritize:

- Domain logic.
- Graph algorithms.
- Document editing operations.
- Data persistence.
- AI response transformations.
- Error handling and recovery.

Use unit tests for isolated logic and integration tests for interactions between components.

Keep test implementations in separate files under the root `tests/` directory.

- Store unit tests in `tests/unit/`, organized by the module or feature they verify.
- Include unit test files through `#[cfg(test)]` and `#[path]` declarations in their owning modules.
- Keep test functions and inline test module bodies out of production source files.
- Preserve private access through child test modules. Do not expose implementation details solely
  for testing.
- Place integration test entry points in `tests/*.rs`. Test public behavior through the
  application's available interfaces.
- Store shared integration helpers in `tests/common/mod.rs` when multiple tests need them.
- Preserve test coverage and assertions when moving tests.

Check significant changes with:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Run checks supported by the current project configuration.

Never claim a test or check passed unless it was actually executed successfully.

Report failed or unavailable checks.
