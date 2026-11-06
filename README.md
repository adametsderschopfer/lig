# lig — Life is Graph

Нативное настольное приложение на Rust и [Iced 0.14](https://docs.rs/iced/0.14.0/iced/). Стартовый
экран показывает **Hello, world!** в центре окна с прозрачным фоном.

Поддерживаемые платформы: **macOS, Windows и Linux**. Системная рамка окна сохранена: окно можно
перемещать, изменять его размер и закрывать обычными средствами ОС.

## Подготовка

- Установите [Rust через rustup](https://rustup.rs/). Версия Rust 1.90.0 и компоненты `rustfmt` /
  `clippy` закреплены в `rust-toolchain.toml`.
- Для Prettier и общей команды проверок установите Node.js 24 или новее и выполните `npm ci`. Само
  приложение запускается через Cargo без Node.js.
- На macOS нужны Xcode Command Line Tools: `xcode-select --install`.
- На Windows нужен Rust MSVC и Visual Studio Build Tools с компонентом **Desktop development with
  C++**.
- На Ubuntu / Debian установите зависимости:

  ```sh
  sudo apt-get install build-essential pkg-config libx11-dev libxkbcommon-dev libwayland-dev libudev-dev
  ```

## Запуск и сборка

```sh
cargo run --locked
cargo build --locked --release
```

Готовый бинарный файл: `target/release/lig`, на Windows — `target/release/lig.exe`.

Прозрачность включена и в настройках нативного окна, и в цвете фона Iced. На Linux визуальный эффект
зависит от композитора оконного менеджера; для X11 нужен включённый композитинг. Автотесты проверяют
настройки прозрачности, но не заменяют визуальную проверку окна на каждой ОС.

## Проверки

```sh
npm ci
npm run check
```

Команда запускает rustfmt, Prettier, Clippy, проверку типов и тесты. Отдельные команды:

| Команда                | Назначение                                 |
| ---------------------- | ------------------------------------------ |
| `npm run format`       | Применить rustfmt и Prettier               |
| `npm run format:check` | Проверить форматирование                   |
| `npm run lint`         | Запустить Clippy с запретом предупреждений |
| `npm run typecheck`    | Проверить Rust-код и тестовые цели         |
| `npm test`             | Запустить тесты                            |
| `npm start`            | Запустить приложение                       |

Rust форматируется через rustfmt, Markdown / JSON / YAML — через Prettier. UI-тесты используют
`iced_test` и программный рендерер `tiny-skia`, поэтому работают без дисплея и GPU. Они проверяют
наличие приветствия при разных размерах окна, название приложения и прозрачность во всех темах Iced.

GitHub Actions запускает сборку, Clippy, проверку типов и тесты на macOS, Windows и Linux.
Форматирование проверяется отдельным заданием.

## Структура

- `src/main.rs` — приложение, настройки окна и стартовый экран.
- `src/tests.rs` — тесты интерфейса и конфигурации приложения.
- `.github/workflows/ci.yml` — проверки трёх платформ.
- `AGENTS.md` — обязательные правила работы над проектом.
