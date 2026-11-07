# ADR 0002: Try graphics backends in separate processes

Status: Accepted.

## Context

The project requires an explicit backend order on each platform. Iced 0.14 reads `WGPU_BACKEND` as
an allowed backend set. That set does not establish the requested priority. Native event loops also
restrict initialization within one process.

## Decision

Launch the same executable with one backend per attempt. Set `ICED_BACKEND=wgpu` to prevent an early
software fallback. Set `WGPU_BACKEND` to the current candidate. Use `LIG_GRAPHICS_CHILD=1` to
identify the application process. Keep this marker internal.

Try Metal, Vulkan, then OpenGL on macOS. Try Vulkan, DirectX 12, then OpenGL on Windows. Try Vulkan,
then OpenGL on Linux.

Reserve exit code 78 for Iced graphics initialization failures. Only that code triggers another
attempt. Stop after normal closure, unrelated application failures, process failures, or signals. Do
not change the parent process environment.

## Alternatives

One combined backend set would not enforce priority. Mutating process environment would conflict
with the crate's unsafe-code prohibition. A custom compositor would require more maintenance against
Iced internals.

## Consequences

The launcher remains alive while the application process runs. macOS fallback backends require
external MoltenVK/Vulkan loader or ANGLE/EGL libraries. The build enables these backend features
without bundling their libraries. Unit tests verify ordering and failure handling without physical
GPUs. Native checks must separately verify available graphics backends.
