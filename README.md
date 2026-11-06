# lig — Life is Graph

Нативное настольное приложение на Rust и [Iced 0.14](https://docs.rs/iced/0.14.0/iced/). Стартовый
экран показывает **Hello, world!** на непрозрачной поверхности в стиле Liquid Glass: мягкий
градиент, светлая окантовка, скругления и тень. Это кроссплатформенное оформление Iced, а не
системный материал Apple.

Поддерживаемые платформы: **macOS, Windows и Linux**. Системная рамка окна сохранена: окно можно
перемещать, изменять его размер и закрывать обычными средствами ОС.

## Подготовка

- Установите [Rust через rustup](https://rustup.rs/). Rust 1.90.0, `rustfmt` и `clippy` закреплены в
  `rust-toolchain.toml`.
- Для форматирования документации и конфигурации установите
  [dprint 0.64.3](https://github.com/dprint/dprint/releases/tag/0.64.3). На macOS доступна команда
  `brew install dprint`. Node.js/npm не требуются.
- Для коротких команд `make ...` нужен GNU Make. На macOS он доступен в Xcode Command Line Tools
  (`xcode-select --install`), на Linux — в инструментах сборки.
- На Windows установите Rust MSVC и Visual Studio Build Tools с компонентом **Desktop development
  with C++**. GNU Make можно установить отдельно, например через `winget install GnuWin32.Make`, или
  использовать команды Cargo и dprint напрямую.
- На Ubuntu / Debian установите зависимости:

  ```sh
  sudo apt-get install build-essential pkg-config libx11-dev libxkbcommon-dev libwayland-dev libudev-dev
  ```

## Запуск и сборка

```sh
make run
make release
```

Эквивалентные команды без Make:

```sh
cargo run --locked
cargo build --locked --release
```

Готовый бинарный файл: `target/release/lig`, на Windows — `target/release/lig.exe`.

## Проверки

```sh
make check
make build
```

| Команда          | Назначение                                 |
| ---------------- | ------------------------------------------ |
| `make run`       | Запустить приложение                       |
| `make build`     | Собрать отладочный бинарный файл           |
| `make release`   | Собрать релизный бинарный файл             |
| `make fmt`       | Применить rustfmt и dprint                 |
| `make fmt-check` | Проверить форматирование                   |
| `make lint`      | Запустить Clippy с запретом предупреждений |
| `make typecheck` | Проверить Rust-код и тестовые цели         |
| `make test`      | Запустить тесты                            |
| `make check`     | Выполнить все проверки                     |

Все проверки можно запустить без Make на любой поддерживаемой ОС:

```sh
cargo fmt --all -- --check
dprint check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo check --locked --all-targets --all-features
cargo test --locked --all-targets --all-features
```

Rust форматируется через rustfmt, Markdown / JSON / YAML — через dprint. UI-тесты используют
`iced_test` и программный рендерер `tiny-skia`, поэтому работают без дисплея и GPU. Они проверяют
видимость и центрирование приветствия при разных размерах окна, название приложения и непрозрачность
фона.

GitHub Actions запускает сборку, Clippy, проверку типов и тесты на macOS, Windows и Linux.
Форматирование проверяется отдельным заданием. Эти проверки не заменяют визуальную проверку окна на
каждой ОС.

## Структура

- `src/main.rs` — приложение, настройки окна и стартовый экран.
- `src/tests.rs` — тесты интерфейса и конфигурации приложения.
- `Makefile` — команды запуска, сборки, форматирования и проверок.
- `dprint.json` — настройки и закреплённые версии форматтеров документации.
- `.github/workflows/ci.yml` — проверки трёх платформ.
- `AGENTS.md` — обязательные правила работы над проектом.
