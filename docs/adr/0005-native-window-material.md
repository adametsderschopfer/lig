# ADR 0005: Native window material

## Status

Accepted.

## Context

The welcome window must show native Liquid Glass when available.
The previous opaque gradient panel prevented users from seeing content behind the window.
Unsupported systems need a plain gray background.
The project forbids unsafe code in application modules.

## Decision

Use macOS-only `objc2`, `objc2-app-kit`, and `raw-window-handle` dependencies.
Call safe AppKit APIs through Iced's `window::run` callback after window creation.
Check the native glass class before allocating it.
Match the raw handle against retained content views without dereferencing pointers.
Find the shared ancestor of the content view and native close button.
Require the content view to be a direct child of that ancestor.
Insert glass as a sibling below the content view.
Copy the content frame and autoresizing mask to keep both views aligned.
Verify sibling order before reporting success. Remove the glass if ordering fails.
Preserve the renderer's content view and input handling.
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
The initial `window-vibrancy` integration inserted glass inside the renderer's content view.
That placement blurred the greeting as part of the background.
Safe AppKit bindings allow explicit sibling placement without unsafe application code.
Older vibrancy materials provide a different appearance from Liquid Glass.
The requested fallback uses plain gray instead.

## Consequences

Liquid Glass requires macOS 26 or newer.
Windows and Linux retain their normal windows with an opaque gray content background.
The bindings own unsafe native calls. The application adds no unsafe code or Tauri runtime.
Automated tests cover fallback transitions, presentation configuration, and sibling order validation.
Unavailable native classes, unmatched handles, and unexpected hierarchy retain the gray fallback.
Native appearance still requires manual verification with the actual window and graphics backend.
