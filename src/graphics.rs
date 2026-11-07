//! Start Iced with one graphics backend at a time, in platform priority order.
//!
//! Each attempt has its own process: native event loops cannot be recreated, and
//! setting the child's environment avoids mutating a multithreaded environment.

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

pub fn run(application: impl FnOnce() -> iced::Result) -> ExitCode {
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
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn priorities_match_each_supported_platform() {
        assert_eq!(
            priority("macos"),
            Some([Backend::Metal, Backend::Vulkan, Backend::OpenGl].as_slice())
        );
        assert_eq!(
            priority("windows"),
            Some([Backend::Vulkan, Backend::DirectX12, Backend::OpenGl].as_slice())
        );
        assert_eq!(
            priority("linux"),
            Some([Backend::Vulkan, Backend::OpenGl].as_slice())
        );
        assert!(priority("unsupported").is_none());
    }

    #[test]
    fn failed_initialization_tries_the_next_backend_on_every_platform() {
        for os in ["macos", "windows", "linux"] {
            let backends = priority(os).unwrap();

            for available in 0..backends.len() {
                let mut tried = Vec::new();
                let result = try_backends(backends, |backend| {
                    tried.push(backend);
                    Ok(if backend == backends[available] {
                        0
                    } else {
                        GRAPHICS_UNAVAILABLE
                    })
                });

                assert_eq!(result.unwrap(), Some(0));
                assert_eq!(tried, backends[..=available]);
            }
        }
    }

    #[test]
    fn unrelated_application_failures_do_not_retry() {
        let mut tried = Vec::new();
        let result = try_backends(priority("windows").unwrap(), |backend| {
            tried.push(backend);
            Ok(1)
        });

        assert_eq!(result.unwrap(), Some(1));
        assert_eq!(tried, [Backend::Vulkan]);
    }

    #[test]
    fn all_unavailable_backends_are_attempted_once() {
        let backends = priority("macos").unwrap();
        let mut tried = Vec::new();
        let result = try_backends(backends, |backend| {
            tried.push(backend);
            Ok(GRAPHICS_UNAVAILABLE)
        });

        assert_eq!(result.unwrap(), None);
        assert_eq!(tried, backends);
    }

    #[test]
    fn process_start_failure_does_not_retry() {
        let mut attempts = 0;
        let result = try_backends(priority("linux").unwrap(), |_| {
            attempts += 1;
            Err(io::Error::new(io::ErrorKind::PermissionDenied, "test"))
        });

        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(attempts, 1);
    }

    #[test]
    fn child_gets_exact_backend_and_original_arguments() {
        for (backend, expected) in [
            (Backend::Metal, "metal"),
            (Backend::Vulkan, "vulkan"),
            (Backend::DirectX12, "dx12"),
            (Backend::OpenGl, "gl"),
        ] {
            let args = [OsString::from("a path with spaces")];
            let command = child_command(Path::new("lig"), &args, backend);
            let variables: std::collections::BTreeMap<_, _> = command.get_envs().collect();

            assert_eq!(command.get_args().collect::<Vec<_>>(), args);
            assert_eq!(variables[OsStr::new(CHILD_MARKER)], Some(OsStr::new("1")));
            assert_eq!(
                variables[OsStr::new("ICED_BACKEND")],
                Some(OsStr::new("wgpu"))
            );
            assert_eq!(
                variables[OsStr::new("WGPU_BACKEND")],
                Some(OsStr::new(expected))
            );
        }
    }

    #[test]
    fn only_graphics_creation_errors_trigger_fallback() {
        let graphics_error = iced_test::renderer::graphics::Error::List(Vec::new());

        assert_eq!(
            application_exit_code(Err(iced::Error::GraphicsCreationFailed(graphics_error))),
            GRAPHICS_UNAVAILABLE
        );
        assert_eq!(application_exit_code(Ok(())), 0);
        assert_eq!(
            application_exit_code(Err(iced::Error::ExecutorCreationFailed(io::Error::other(
                "test"
            )))),
            1
        );
    }
}
