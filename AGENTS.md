# Development Rules for lig

`lig` means **Life is Graph**. Build a native Rust application with Iced.

## Required platform behavior

- Maintain macOS, Windows, and Linux support.
- Prioritize Metal, Vulkan, then OpenGL on macOS.
- Prioritize Vulkan, DirectX 12, then OpenGL on Windows.
- Prioritize Vulkan, then OpenGL on Linux.
- Retry only after graphics initialization fails.
- Use cross-platform APIs. Isolate platform-specific code with `cfg`.
- Keep the three-platform CI matrix.
- Check interface changes with a local application run.
- Report visual checks separately from compilation checks.
- Keep the glass background opaque.

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

## Project Architecture & Engineering Rules

### 1. Technology Stack

- **Language:** Rust (stable, latest compatible edition).
- **UI Framework:** Iced.
- **Rendering:** Iced's native rendering stack, with GPU acceleration where appropriate.
- **Primary Platform:** macOS.
- **Additional Platforms:** Windows and Linux.
- **Architecture:** Modular Monolith + The Elm Architecture + pragmatic Clean Architecture.
- **Code Organization:** Feature-Based.

Do not introduce Electron, Tauri, HTML, CSS, JavaScript UI frameworks, Qt, or custom C++ code.

Prefer Rust-native solutions and cross-platform libraries.

Use the official Iced documentation and examples compatible with the project's pinned Iced version.
Never assume that APIs from older examples are still valid.

### 2. Architecture Principles

Follow these principles:

1. **Separation of Concerns** — UI, application logic, domain logic, and infrastructure must have
   clearly defined responsibilities.
2. **Single Responsibility Principle** — modules and components should have focused
   responsibilities.
3. **Dependency Inversion** — domain logic must remain independent of UI frameworks, databases, and
   external services.
4. **High Cohesion, Low Coupling** — related functionality belongs together; unrelated modules
   should not depend on each other's implementation details.
5. **Composition over Inheritance** — use Rust composition and traits where they provide real value.
6. **Explicit Dependencies** — avoid global mutable state, hidden dependencies, and unnecessary
   singletons.
7. **KISS, DRY, YAGNI** — prefer simple solutions, avoid meaningful duplication, and do not
   introduce abstractions for hypothetical requirements.
8. **Type Safety** — use Rust's type system to make invalid states difficult or impossible to
   represent.
9. **Testability** — design business logic so that it can be tested independently from the UI.

Avoid overengineering.

Do not introduce architectural complexity without a concrete reason.

### 3. Project Structure

Organize the application primarily by features.

Recommended initial structure:

```text
src/
├── main.rs
├── app/
│   ├── mod.rs
│   ├── state.rs
│   ├── message.rs
│   ├── update.rs
│   └── subscription.rs
├── features/
│   ├── notes/
│   ├── editor/
│   ├── graph/
│   ├── voice/
│   └── ai/
├── shared/
│   ├── ui/
│   ├── theme/
│   ├── animation/
│   └── error/
└── infrastructure/
    ├── database/
    ├── filesystem/
    ├── audio/
    └── ai_providers/

docs/
├── architecture.md
└── adr/
```

This is a reference structure, not a requirement to create empty directories.

Each feature may contain its own state, messages, update logic, views, domain models, and
application services as needed.

Rules:

- Organize functionality by domain or feature.
- Keep feature implementations encapsulated.
- Expose minimal public APIs.
- Avoid circular dependencies.
- Do not create large miscellaneous `utils`, `helpers`, or `common` modules.
- Extract shared components only when they are genuinely reusable.
- Introduce additional Cargo crates only when meaningful dependency boundaries justify them.
- Use Cargo Workspace when the project grows large enough to benefit from multiple crates.

### 4. Iced Architecture

Follow **The Elm Architecture** consistently.

Use the following concepts:

- **State:** Represents the current state of the UI.
- **Message:** Describes user actions and application events.
- **Update:** Processes messages and updates state.
- **View:** Declaratively describes the interface.
- **Task:** Handles asynchronous operations.
- **Subscription:** Handles ongoing event streams and subscriptions.

Rules:

1. Keep views declarative and free from business logic.
2. Do not perform blocking I/O inside UI update or rendering paths.
3. Use typed messages for state transitions.
4. Organize messages and state by feature when appropriate.
5. Delegate complex operations to application services.
6. Use `Task` for asynchronous operations and `Subscription` for ongoing event streams.
7. Do not store persistent domain data exclusively in UI state.
8. Keep UI-specific state separate from domain data.
9. Avoid unnecessary global application state.
10. Avoid rebuilding or recalculating expensive data when localized updates are sufficient.

Use the Iced API idiomatically rather than forcing patterns from React, Redux, or other UI
frameworks into the project.

### 5. Domain & Business Logic

Keep domain logic independent of Iced.

Domain modules must not depend on:

- UI widgets or rendering code.
- Iced application state.
- Concrete database implementations.
- Filesystem APIs.
- External AI SDK implementations.

Use application services or use cases to coordinate business operations.

Use traits at real infrastructure boundaries where testing or multiple implementations require them.

Do not create repository interfaces, service layers, or dependency injection abstractions
automatically for every type.

### 6. UI, Design System & Animations

Build a consistent, maintainable design system.

Centralize:

- Colors and semantic color tokens.
- Typography.
- Spacing.
- Border radii.
- Shadows and effects.
- Animation durations and easing.
- Reusable components.
- Light and dark themes.

Avoid hardcoded styling values scattered across features.

Prefer reusable, composable UI components.

For animations:

- Separate animation state from persistent domain state.
- Use time-based animation logic rather than frame-count assumptions.
- Avoid unnecessary redraws.
- Support smooth transitions, hover effects, and animated state changes.
- Profile performance-sensitive effects.
- Provide reduced-motion behavior where practical.

Use Iced's native rendering capabilities, custom widgets, and Canvas where appropriate.

Introduce custom GPU rendering only when necessary.

### 7. Graph Visualization

Treat graph visualization as an independent subsystem.

Separate:

1. Graph data model.
2. Graph algorithms.
3. Layout and physics simulation.
4. Interaction state.
5. Rendering.
6. Animation.

Graph algorithms must not depend on Iced.

Requirements:

- Support incremental graph updates.
- Avoid unnecessary full layout recalculations.
- Keep rendering independent from graph persistence.
- Support pan, zoom, selection, and drag operations.
- Optimize rendering for large graphs.
- Use caching, spatial indexing, and viewport culling when justified by profiling.
- Keep expensive calculations off the UI thread.
- Write tests for graph algorithms and data transformations.

### 8. Text Editor

Separate the document model from editor interaction and rendering.

Keep independent:

- Document content.
- Cursor and selection state.
- Editing commands.
- Undo/redo history.
- Markdown parsing and formatting.
- Persistence.

Avoid coupling document storage to specific Iced widgets.

Preserve document integrity during asynchronous saves.

Design editor components with keyboard navigation, clipboard operations, Unicode text handling, and
future extensibility in mind.

Do not implement a complex custom editor engine unless existing solutions are insufficient.

### 9. Audio & AI

Separate the following concerns:

- Microphone capture.
- Audio recording state.
- Encoding and file storage.
- Transcription.
- AI processing.
- Knowledge extraction.

Never block the UI thread during recording, encoding, transcription, or AI processing.

Keep external AI providers behind clear interfaces when provider substitution or testing requires
it.

Do not couple the domain model to any specific AI vendor.

Handle errors, cancellation, microphone permissions, and resource cleanup explicitly.

### 10. Rust Code Quality

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

### 11. Testing & Verification

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

### 12. Cross-Platform Development

Develop for macOS first without unnecessarily coupling the application to macOS.

- Prefer cross-platform APIs.
- Isolate platform-specific implementations.
- Avoid hardcoded paths and platform assumptions.
- Handle filesystem locations using appropriate platform APIs.
- Consider platform-specific keyboard shortcuts.
- Verify Windows and Linux compatibility as development progresses.
- Keep native integrations isolated behind clear boundaries.

### 13. Documentation & Architectural Decisions

Documentation is a required part of application development. Update documentation when changing
application behavior, configuration, storage, architecture, or development workflows. Include
documentation in the same commit or pull request. Complete documentation before declaring the change
finished.

Before implementation, read the relevant project documentation. After implementation, update the
existing documentation or create the necessary document if none exists.

Maintain the following sources of truth:

- `README.md` — project overview, setup, dependencies, supported platforms, configuration, run,
  build, and verification commands.
- `docs/architecture.md` — the implemented system structure, module responsibilities, data flow, and
  dependency boundaries.
- `docs/features/<feature>.md` — explain feature purpose, user scenarios, behavior, data models,
  interfaces, errors, and limitations. Describe persistence and migrations when applicable.
- `docs/adr/` — lightweight Architecture Decision Records for significant architectural decisions:
  context, chosen approach, alternatives, trade-offs, and consequences.

Documentation rules:

- Write project documentation and code comments primarily in English. This includes README,
  architecture documents, feature documents, ADRs, Rustdoc, and inline comments.
- Russian is the secondary language. Put English before Russian when providing both languages.
  Alternatively, link from the English document to a Russian translation.
- Write new documentation and comments in English first. Translate affected Russian-only
  documentation or comments into English during substantive updates. Retain Russian as a secondary
  translation where useful.
- Keep translations consistent with the English version; update affected translations in the same
  change. This language rule does not determine the application's UI language or the language used
  to communicate with the user.
- Describe the actual implementation. Clearly distinguish planned behavior from working features.
- Update affected instructions, examples, links, and diagrams when behavior or structure changes.
- Keep platform differences and known limitations explicit.
- Keep one authoritative description of each topic and link to it rather than duplicating it.
- Verify documented commands and examples where practical; report anything that could not be
  verified.
- Create documents when they contain useful information; do not create empty templates or
  directories.
- Do not duplicate obvious code or document trivial implementation details.
- Skip unrelated documentation changes when behavior and workflows remain unchanged.
- Review documentation alongside the code before declaring work complete. Run documentation
  formatting checks. Describe documentation changes in the completion report.

#### Clear Technical Writing

Use the user's five writing rules inspired by ASD-STE100. Apply these rules to new and changed
responses, documentation, and code comments.

1. Use active voice. Name the actor when the action could otherwise be unclear.
2. Limit each sentence to 20 words.
3. Express one idea per sentence.
4. Use one consistent term for each concept. Do not alternate synonyms for the same concept.
5. Use plain, literal language. Avoid idioms and jargon.

Keep API names, commands, and code identifiers exact. Explain required technical terms when you
introduce them. Preserve code and command syntax when editing prose. Review new and changed text
against these rules before completing a task.

### 14. AI Coding Agent Workflow

Before implementing a change:

1. Inspect the relevant existing code.
2. Understand the current architecture and established conventions.
3. Identify the responsible feature and architectural layer.
4. Reuse existing components and abstractions where appropriate.
5. For significant changes, briefly describe the implementation plan.
6. Implement the smallest complete solution.
7. Add or update meaningful tests.
8. Update the affected project documentation as part of the implementation.
9. Run relevant quality checks, including documentation formatting and applicable documented
   commands.
10. Review code and documentation for consistency, unnecessary complexity, and architectural
    violations.

Additional rules:

- Do not rewrite working architecture without a clear reason.
- Do not introduce unnecessary design patterns.
- Do not add abstractions purely for future possibilities.
- Do not create new dependencies without evaluating existing solutions.
- Avoid unrelated refactoring.
- Prefer targeted, maintainable changes.
- Preserve compatibility with existing project conventions.
- When an architectural rule must be broken, explain the trade-off before proceeding.
- Never invent library APIs or claim unsupported functionality exists.
- Prefer official documentation and examples matching installed dependency versions.

### 15. Core Engineering Philosophy

**Build for long-term maintainability without sacrificing development speed.**

The application should be:

- Modular.
- Performant.
- Testable.
- Cross-platform.
- Extensible.
- Visually polished.
- Easy to understand.
- Easy to refactor.

Favor clean architecture over clever abstractions.

Favor measurable performance over premature optimization.

Favor simplicity over unnecessary complexity.

**Every architectural decision should make the application easier to develop, understand, test, or
maintain.**
