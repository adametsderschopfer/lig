# ADR 0003: Embed Fluent catalogs with explicit language state

Status: Accepted.

## Context

Every application message needs English, Russian, and Spanish translations. Translations must work
on macOS, Windows, and Linux without external files. Project rules prohibit hidden mutable
dependencies.

## Decision

Use Fluent catalogs and embed them with `include_str!`. Register message identifiers through a typed
key list. Keep the selected localizer in application state and pass it explicitly to views and
diagnostics. Use `sys-locale` to read the system locale. Let `LIG_LANG` override startup language
selection. Use English for unsupported locales and invalid selected messages. Validate every catalog
directly in tests before fallback can hide defects.

Use nontechnical fallback text when a message is unavailable. Never expose message identifiers in
the interface.

## Alternatives

Plain Rust string tables would mix translation content with implementation. JSON catalogs would
require additional formatting rules for variables and future plural forms. A global locale would
hide dependencies and complicate independent tests. Runtime translation files would add deployment
paths and missing-file failures.

## Consequences

Translation edits require a rebuild. Each text change requires three catalog updates. Fluent handles
message arguments and language-specific selection rules. The current application selects its
language only at startup. Tests cover catalog completeness, formatting, language selection,
fallback, and visible interface text.
