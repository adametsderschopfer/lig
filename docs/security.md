# Dependency security checks

Run `make security` to audit the complete `Cargo.lock` file. Install cargo-audit with the command in
[the setup guide](../README.md#setup). CI runs the same audit in the **Dependency security** job.

## Coverage

cargo-audit compares dependency versions with the [RustSec advisory database](https://rustsec.org/).
It checks dependencies for all platforms because the command applies no platform filters. The audit
requires network access to refresh advisory data. It does not analyze application control flow or
establish whether each advisory affects a running application.

The project uses cargo-audit's default exit policy. Vulnerability findings fail the check.
Informational findings produce visible warnings, including `unmaintained` and `unsound` findings. An
`unsound` finding describes a possible violation of Rust's memory safety guarantees. Treat warnings
as findings that need review. A successful exit does not mean every dependency is safe. The project
defines no advisory ignore list.

## Findings on 2026-11-07

This snapshot predates the `window-vibrancy` dependency addition.
The user deferred the next audit. These findings do not describe the updated lockfile.

The local audit scanned 416 dependencies. It returned zero vulnerability-class findings and six
informational findings.

| Dependency      | Version | Classification | Advisory                                                              |
| --------------- | ------- | -------------- | --------------------------------------------------------------------- |
| paste           | 1.0.15  | Unmaintained   | [RUSTSEC-2024-0436](https://rustsec.org/advisories/RUSTSEC-2024-0436) |
| ttf-parser      | 0.25.1  | Unmaintained   | [RUSTSEC-2026-0192](https://rustsec.org/advisories/RUSTSEC-2026-0192) |
| zbus_names      | 4.3.5   | Unmaintained   | [RUSTSEC-2026-0371](https://rustsec.org/advisories/RUSTSEC-2026-0371) |
| zvariant        | 5.15.1  | Unmaintained   | [RUSTSEC-2026-0370](https://rustsec.org/advisories/RUSTSEC-2026-0370) |
| zvariant_derive | 5.15.1  | Unmaintained   | [RUSTSEC-2026-0372](https://rustsec.org/advisories/RUSTSEC-2026-0372) |
| lru             | 0.16.4  | Unsound        | [RUSTSEC-2026-0253](https://rustsec.org/advisories/RUSTSEC-2026-0253) |

The `lru` advisory concerns memory access after release when a key destructor panics during
`LruCache::pop()`. Upstream fixed the issue in version 0.18.2. See
[the upstream fix](https://github.com/jeromefroe/lru-rs/pull/238). The current dependency chain is
`iced_wgpu` → `cryoglyph` → `lru`. `cryoglyph` 0.1.0 requires `lru` 0.16, which excludes the fixed
version. A compatible lockfile update alone cannot resolve this finding. Review an upstream graphics
dependency update before changing that requirement. This audit did not establish whether lig can
trigger the reported condition.

Review these findings after dependency updates. Refresh this record when the findings change.
