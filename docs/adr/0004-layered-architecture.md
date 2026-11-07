# ADR 0004: Organize source code by responsibility layers

Status: Accepted. Supersedes [ADR 0001](0001-module-boundaries.md).

## Context

The previous structure grouped screens under `features` and presentation utilities under `shared`.
The project now requires explicit locations for presentation, user operations, business rules, and external integrations.
The current application has one greeting screen and no business operations.

## Decision

Keep one Rust binary crate and organize its source by layers.
Keep application composition and Iced message routing in `app`.
Move screens into `ui/screens` and presentation styling into `ui/theme/`.
Move localization into the independent `i18n` module.
Keep native integration code in `infrastructure`.

Introduce `application` for user operations and `domain` for business rules when implementation requires them.
Keep both layers independent of Iced and concrete infrastructure implementations.
Use explicit contracts for external operations when actual scenarios need them.
Do not create placeholder modules or a mandatory interface for every type.

Preserve separate test files, embedded translations, graphics priorities, and the existing interface behavior.
Keep feature guides under `docs/features/`; those documents describe behavior rather than source organization.

## Alternatives

Feature-based organization keeps related functionality together but makes layer responsibilities less visible at the root.
Subject-area modules can suit larger subsystems but require boundaries that the current application does not yet have.
A full ports-and-adapters structure would add unnecessary conventions at the current size.

## Consequences

Contributors locate code by its responsibility.
A user operation may require changes across several layers.
Layer rules keep presentation and external integrations outside business logic.
Tests keep their existing assertions and access through child modules.
Additional crates remain optional when concrete dependency boundaries justify them.
