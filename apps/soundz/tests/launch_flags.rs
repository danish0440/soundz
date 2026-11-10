//! The desktop binary's command line, run as a process: the variable fallback of `--control`.

use std::process::{Command, Output};

fn soundz(args: &[&str], control_port: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_soundz"))
        .args(args)
        .env("SOUNDZ_NO_PREFS", "1")
        .env("SOUNDZ_CONTROL_PORT", control_port)
        .env_remove("RUST_LOG")
        // Without a display the window cannot open, so a launch that passes parsing ends instead of
        // staying up.
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .output()
        .expect("run soundz")
}

#[test]
fn a_bad_variable_is_a_usage_error_naming_it() {
    let out = soundz(&["--no-audio"], "nope");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid value 'nope' for 'SOUNDZ_CONTROL_PORT'"), "{stderr}");
}

#[cfg(all(unix, not(target_os = "macos")))]
#[test]
fn a_blank_variable_is_unset() {
    let out = soundz(&["--no-audio"], "  ");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ne!(out.status.code(), Some(2), "{stderr}");
    assert!(!stderr.contains("SOUNDZ_CONTROL_PORT"), "{stderr}");
}

/// `soundz-cli app` takes `host:port` from the same variable.
#[cfg(all(unix, not(target_os = "macos")))]
#[test]
fn a_loopback_host_and_port_variable_is_accepted() {
    let out = soundz(&["--no-audio"], "127.0.0.1:0");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ne!(out.status.code(), Some(2), "{stderr}");
    assert!(!stderr.contains("SOUNDZ_CONTROL_PORT"), "{stderr}");
}

/// The variable may point `soundz-cli app` at another machine: the app opens, with a warning
/// and no control server.
#[cfg(all(unix, not(target_os = "macos")))]
#[test]
fn another_host_in_the_variable_is_ignored() {
    let out = soundz(&["--no-audio"], "10.0.0.5:7979");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ne!(out.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("SOUNDZ_CONTROL_PORT=10.0.0.5:7979 names another host, so no control server starts"), "{stderr}");
}
