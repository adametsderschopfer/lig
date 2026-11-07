# Architecture

Follow these rules when [AGENTS.md](../../AGENTS.md) directs you here.

## Technology Stack

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

## Architecture Principles

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

## Project Structure

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

## Domain & Business Logic

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

## Core Engineering Philosophy

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
