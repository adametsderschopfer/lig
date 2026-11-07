# Welcome screen

## Behavior

Show only the translated greeting at the center of the window.
Preserve native window controls for moving, resizing, and closing.
Start at 960 × 640 logical pixels. Allow resizing down to 320 × 240 logical pixels.
Translate the title and greeting through the selected i18n catalog.
On macOS, hide the visible title and extend the content beneath the transparent title bar.
Keep the localized title available to the operating system.

| Language | Greeting      |
| -------- | ------------- |
| English  | Hello, world! |
| Russian  | Привет, мир!  |
| Spanish  | ¡Hola, mundo! |

See [language selection](i18n.md#language-selection) for startup behavior.

## Window material

Use the denser native material to reduce distracting text behind the window.

On macOS 26+, request native Liquid Glass with the Regular style.
Clear the Iced background after native material initialization succeeds.
The native material lets users see content behind the window.
Keep system window corners and controls. Add no inset panel, gradient, outline, shadow, or custom corner radius.
Use a dark theme with light greeting text.

Use an opaque gray background (`#212121`) on Windows, Linux, and older macOS versions.
Use the same background when native material initialization fails.
Show no technical status message for this fallback.

## Implementation

`app` owns window configuration, material readiness, and message routing.
`ui::screens::welcome` centers the greeting. `ui::theme` defines text and background colors.
`infrastructure::window_effect` applies the native material through Iced's window callback.
The macOS implementation uses the safe `window-vibrancy` API.
The dependency checks the macOS version before creating the native material.
Other platforms return an unavailable result without native material access.
`i18n` supplies translated messages. The screen performs no I/O and stores no persistent data.

## Verification

Application tests cover window configuration, initial opacity, and material completion transitions.
Welcome tests cover greeting visibility and centering at minimum, default, and larger sizes in all three languages.
Theme tests cover the transparent material background and opaque gray fallback.
Run `make test` to execute these checks.

Native appearance needs a separate manual check on macOS 26+.
Place another window behind lig and check transparency, text visibility, resizing, and native controls.
Check gray fallback separately on older macOS, Windows, and Linux.
Headless tests cannot establish native material appearance.

## Limitations

The operating system controls the material appearance and accessibility adaptations.
The application does not reproduce ChatGPT's private window implementation.
