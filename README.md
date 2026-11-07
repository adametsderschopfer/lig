# lig — Life is Graph

lig is a native desktop application built with Rust and Iced. The initial screen shows **Hello,
world!** on an opaque glass surface. Iced draws the gradient, border, rounded corners, and shadow.
The application uses a fixed light theme.

The interface and startup messages support English, Russian, and Spanish. The application uses the
system language and falls back to English for unsupported locales. Set `LIG_LANG` to `en`, `ru`, or
`es` to override it. See [internationalization](docs/features/i18n.md) for platform-specific
commands and translation requirements.

macOS is the primary development platform. The project also targets Windows and Linux.

## Setup

Install [Rust through rustup](https://rustup.rs/). `rust-toolchain.toml` pins Rust 1.90.0, rustfmt,
and Clippy. Install [dprint 0.64.3](https://github.com/dprint/dprint/releases/tag/0.64.3) for
documentation and configuration formatting. On macOS, run `brew install dprint`.

Install cargo-audit 0.22.2 for dependency security checks:

```sh
cargo install cargo-audit --version 0.22.2 --locked
```

Install the platform tools:

- macOS: install Xcode Command Line Tools with `xcode-select --install`.
- Windows: install Visual Studio Build Tools with **Desktop development with C++**. Use Rust's MSVC
  toolchain.
- Ubuntu or Debian: install the packages below.

```sh
sudo apt-get install build-essential pkg-config libx11-dev libxkbcommon-dev libwayland-dev libudev-dev
```

GNU Make provides command shortcuts. Windows users must install GNU Make separately or run Cargo
commands directly. Application development requires no Node.js or npm.

## Commands

| Command          | Purpose                                       |
| ---------------- | --------------------------------------------- |
| `make run`       | Start the application                         |
| `make build`     | Build the development executable              |
| `make release`   | Build the release executable                  |
| `make fmt`       | Apply rustfmt and dprint                      |
| `make fmt-check` | Check formatting                              |
| `make lint`      | Run Clippy with warnings treated as errors    |
| `make typecheck` | Check Rust types and test targets             |
| `make test`      | Run tests                                     |
| `make security`  | Check dependencies against RustSec advisories |
| `make check`     | Run all development checks                    |

Cargo commands also work without Make:

```sh
cargo run --locked
cargo build --locked --release
cargo fmt --all -- --check
dprint check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo check --locked --all-targets --all-features
cargo test --locked --all-targets --all-features
cargo audit --file Cargo.lock
```

The release executable is `target/release/lig`. Windows uses `target/release/lig.exe`.

## Graphics backends

| Platform | Priority                     |
| -------- | ---------------------------- |
| macOS    | Metal → Vulkan → OpenGL      |
| Windows  | Vulkan → DirectX 12 → OpenGL |
| Linux    | Vulkan → OpenGL              |

The launcher tries each backend separately. Only graphics initialization failures trigger the next
attempt. Normal closure and unrelated failures stop the launcher. On macOS, Vulkan requires a Vulkan
loader and MoltenVK. OpenGL/GLES requires ANGLE/EGL libraries. The application does not bundle these
libraries. Metal needs neither fallback library. See
[the graphics decision](docs/adr/0002-graphics-fallback.md) for implementation details.

## Tests and checks

Unit tests live in `tests/unit/`. Owning modules include them with `#[cfg(test)]` and `#[path]`.
Tests retain private access without expanding production APIs. Future integration tests belong in
`tests/*.rs`.

Interface tests use `iced_test` with the tiny-skia CPU renderer. They require neither a display nor
a GPU. CI runs separate jobs for Clippy, tests, type checks, and release builds on macOS, Windows,
and Linux. CI also runs dedicated formatting and dependency security jobs.

[cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit) checks `Cargo.lock` against
the RustSec advisory database. The security job fails when it finds known vulnerabilities.
Informational advisories appear as warnings. Security checks require network access to refresh
advisory data. They do not prove that application code has no security defects. See
[security notes](docs/security.md) for the audit policy and current warnings.

Platform builds check native compilation and linking. Tests use each runner's native environment.
These checks do not verify window appearance or every GPU backend.

## Documentation

- [Architecture](docs/architecture.md)
- [Welcome feature](docs/features/welcome.md)
- [Internationalization](docs/features/i18n.md)
- [Module boundaries](docs/adr/0001-module-boundaries.md)
- [Graphics fallback](docs/adr/0002-graphics-fallback.md)
- [Development rules](AGENTS.md)

`AGENTS.md` lists essential requirements and required reading. Detailed rules live in `docs/rules/`,
grouped by topic. Read the applicable files explicitly before making changes.

English is the primary documentation language. Russian translations are secondary when provided.
Update affected documentation with each implementation change.
