# ADR 0001: Keep one crate with explicit module boundaries

Status: Superseded by [ADR 0004](0004-layered-architecture.md).

This record describes the earlier organization. ADR 0004 defines the current layer boundaries.
The historical test layout below no longer applies.
See [test organization rules](../rules/quality.md#testing--verification) for the current layout.

## Context

The starter application combined window configuration, interface construction, and styling in
`main.rs`. Project rules require focused modules and feature-based organization. The current feature
set does not justify multiple crates.

## Decision

Keep one binary crate. Use `app` for application composition and state. Use `features` for
implemented screens and behavior. Use `shared` for reusable presentation code. Use `infrastructure`
for graphics and process integration. Keep the entry point small. Keep unit tests in separate files
under `tests/unit/`.

## Alternatives

A single source file would preserve unnecessary coupling. Multiple crates would add maintenance
without meaningful dependency boundaries today. Separate files for every empty state or message type
would add navigation overhead.

## Consequences

Each current module owns a concrete responsibility. Private child tests avoid widening production
APIs. Add modules and crates when implementation needs justify them. Do not create empty feature
directories.
