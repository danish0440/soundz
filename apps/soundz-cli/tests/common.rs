//! Test helpers shared by the `soundz-cli` integration test binaries.

use std::ffi::OsString;

/// Nextest archives relocate binaries; plain Cargo keeps the compile-time path valid.
pub fn soundz_cli_binary() -> OsString {
    std::env::var_os("NEXTEST_BIN_EXE_soundz_cli")
        .or_else(|| std::env::var_os("NEXTEST_BIN_EXE_soundz-cli"))
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_soundz-cli").into())
}
