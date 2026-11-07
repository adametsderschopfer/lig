# ADR 0005: Native window material

## Status

Accepted.

## Context

The welcome window must show native Liquid Glass when available.
The previous opaque gradient panel prevented users from seeing content behind the window.
Unsupported systems need a plain gray background.
The project forbids unsafe code in application modules.

## Decision

Use `window-vibrancy` 0.8.1 as a macOS-only dependency.
Call its safe Liquid Glass API through Iced's `window::run` callback after window creation.
Prefer Regular over Clear to reduce background detail.
Request the Regular style without a custom tint, corner radius, or inset panel.
Keep native window controls and extend macOS content beneath the title bar.
Use Iced's dark theme to match native appearance with light greeting text.

Start with an opaque gray Iced background.
Enable background transparency only after the native API reports success.
Keep gray on unsupported systems and initialization errors.
Isolate native integration in `infrastructure::window_effect`.
Keep presentation state and message routing in `app`.

## Alternatives

Iced gradients cannot sample desktop content behind the window.
A plain transparent window provides no native glass material.
Direct AppKit access would require unsafe application code.
Older vibrancy materials provide a different appearance from Liquid Glass.
The requested fallback uses plain gray instead.

## Consequences

Liquid Glass requires macOS 26 or newer.
Windows and Linux retain their normal windows with an opaque gray content background.
The dependency owns the native API access. The application adds no unsafe code or Tauri runtime.
Automated tests cover fallback transitions and presentation configuration.
Native appearance still requires manual verification with the actual window and graphics backend.
