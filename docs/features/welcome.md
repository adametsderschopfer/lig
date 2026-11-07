# Welcome screen

## Behavior

Show the translated greeting at the center of the window. Keep the content opaque with a light glass
surface. Preserve native window controls for moving, resizing, and closing. Start at 960 × 640
logical pixels. Allow resizing down to 320 × 240 logical pixels. Translate the title and greeting
through the selected i18n catalog.

| Language | Greeting      |
| -------- | ------------- |
| English  | Hello, world! |
| Russian  | Привет, мир!  |
| Spanish  | ¡Hola, mundo! |

See [language selection](i18n.md#language-selection) for startup behavior.

## Implementation

`app` owns window configuration and application state. `features::welcome` builds the view.
`shared::theme` owns styling values. `shared::i18n` supplies translated messages. The screen
performs no I/O and stores no persistent data. It emits no messages.

## Verification

Application tests check the initial screen, window sizes, title, window opacity, and application
background. Welcome tests check visibility and centering at minimum, default, and larger sizes in
all three languages. Theme tests check gradient opacity. Run `make test` to execute these checks.

## Limitations

The screen currently uses a fixed light theme. The glass appearance uses Iced gradients and shadows.
It does not use Apple's system Liquid Glass material.
