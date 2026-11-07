//! Start Iced with one graphics backend at a time, in platform priority order.
//!
//! Each attempt starts a separate process.
//! Native event loops cannot restart within one process.
//! Child environment settings avoid changes to the parent environment.

use std::env;
use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::process::{Command, ExitCode};

const CHILD_MARKER: &str = "LIG_GRAPHICS_CHILD";
const GRAPHICS_UNAVAILABLE: u8 = 78;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Backend {
    Metal,
    Vulkan,
    DirectX12,
    OpenGl,
}

impl Backend {
    fn name(self) -> &'static str {
        match self {
            Self::Metal => "Metal",
            Self::Vulkan => "Vulkan",
            Self::DirectX12 => "DirectX 12",
            Self::OpenGl => "OpenGL",
        }
    }

    fn wgpu_name(self) -> &'static str {
        match self {
            Self::Metal => "metal",
            Self::Vulkan => "vulkan",
            Self::DirectX12 => "dx12",
            Self::OpenGl => "gl",
        }
    }
}

fn priority(os: &str) -> Option<&'static [Backend]> {
    match os {
        "macos" => Some(&[Backend::Metal, Backend::Vulkan, Backend::OpenGl]),
        "windows" => Some(&[Backend::Vulkan, Backend::DirectX12, Backend::OpenGl]),
        "linux" => Some(&[Backend::Vulkan, Backend::OpenGl]),
        _ => None,
    }
}

pub(crate) fn run(application: impl FnOnce() -> iced::Result) -> ExitCode {
    if env::var_os(CHILD_MARKER).as_deref() == Some("1".as_ref()) {
        return ExitCode::from(application_exit_code(application()));
    }

    match launch() {
        Ok(Some(code)) => ExitCode::from(code),
        Ok(None) => {
            eprintln!("lig: none of the platform graphics backends could be initialized");
            ExitCode::FAILURE
        }
        Err(error) => {
            eprintln!("lig: could not start the application: {error}");
            ExitCode::FAILURE
        }
    }
}

fn launch() -> io::Result<Option<u8>> {
    let backends = priority(env::consts::OS).ok_or_else(|| {
        io::Error::new(io::ErrorKind::Unsupported, "unsupported operating system")
    })?;
    let executable = env::current_exe()?;
    let arguments: Vec<_> = env::args_os().skip(1).collect();

    try_backends(backends, |backend| {
        eprintln!("lig: trying {}", backend.name());
        let status = child_command(&executable, &arguments, backend).status()?;

        // Signals and platform-specific crash codes are fatal, not GPU fallbacks.
        Ok(status
            .code()
            .and_then(|code| u8::try_from(code).ok())
            .unwrap_or(1))
    })
}

fn child_command(executable: &Path, arguments: &[OsString], backend: Backend) -> Command {
    let mut command = Command::new(executable);
    command
        .args(arguments)
        .env(CHILD_MARKER, "1")
        // Prevent Iced from choosing tiny-skia before the other GPU backends.
        .env("ICED_BACKEND", "wgpu")
        .env("WGPU_BACKEND", backend.wgpu_name());
    command
}

fn try_backends(
    backends: &[Backend],
    mut attempt: impl FnMut(Backend) -> io::Result<u8>,
) -> io::Result<Option<u8>> {
    for &backend in backends {
        let code = attempt(backend)?;
        if code != GRAPHICS_UNAVAILABLE {
            return Ok(Some(code));
        }
    }

    Ok(None)
}

fn application_exit_code(result: iced::Result) -> u8 {
    match result {
        Ok(()) => 0,
        Err(iced::Error::GraphicsCreationFailed(error)) => {
            eprintln!("lig: graphics initialization failed: {error:?}");
            GRAPHICS_UNAVAILABLE
        }
        Err(error) => {
            eprintln!("lig: application failed: {error:?}");
            1
        }
    }
}

#[cfg(test)]
#[path = "../../tests/unit/graphics.rs"]
mod tests;
