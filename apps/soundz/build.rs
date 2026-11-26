//! Windows only: embed the app icon and version info (VERSIONINFO) into `soundz.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `SOUNDZ_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).
//!
//! `SOUNDZ_BUILD_CHANNEL=nightly-v1` compiles the isolated Nightly flavor: `cfg(soundz_nightly_v1)`,
//! the Nightly icon, `ProductName "SoundZ Nightly"` and the exact feed version as
//! `ProductVersion` (the Launcher checks that field, not FileVersion). An unset, empty or `stable`
//! channel is the stable build; anything else is a build error.

#[path = "src/build_channel.rs"]
mod build_channel;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SOUNDZ_BUILD_CHANNEL");
    println!("cargo:rustc-check-cfg=cfg(soundz_nightly_v1)");
    let nightly = match std::env::var("SOUNDZ_BUILD_CHANNEL").as_deref() {
        Ok("nightly-v1") => {
            println!("cargo:rustc-cfg=soundz_nightly_v1");
            true
        }
        // An empty value (an unset CI input) means the default stable build.
        Ok("stable" | "") | Err(std::env::VarError::NotPresent) => false,
        _ => {
            eprintln!("unsupported SOUNDZ_BUILD_CHANNEL (expected stable or nightly-v1)");
            std::process::exit(1);
        }
    };
    println!("cargo:rerun-if-changed=../../assets/app-icon/soundz.ico");
    println!("cargo:rerun-if-changed=../../assets/app-icon/soundz-nightly.ico");
    println!("cargo:rerun-if-env-changed=SOUNDZ_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=SOUNDZ_VERSION");
    println!("cargo:rerun-if-env-changed=SOUNDZ_OVERRIDE_VERSION");
    let built_version = if nightly {
        // The feed identity is SOUNDZ_VERSION. The packaging scripts (packaging/env.sh and the
        // Linux package.sh, inherited from PhotoZ) set the same value as
        // SOUNDZ_OVERRIDE_VERSION, which is accepted as the one alias so both agree.
        let identity = std::env::var("SOUNDZ_VERSION").or_else(|_| std::env::var("SOUNDZ_OVERRIDE_VERSION"));
        match identity {
            Ok(version) if build_channel::is_nightly_version(&version) => version,
            _ => {
                eprintln!("nightly-v1 requires an explicit SOUNDZ_VERSION feed identity (SOUNDZ_OVERRIDE_VERSION is accepted)");
                std::process::exit(1);
            }
        }
    } else {
        std::env::var("CARGO_PKG_VERSION").unwrap_or_default()
    };
    println!("cargo:rustc-env=SOUNDZ_BUILT_VERSION={built_version}");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon(build_channel::windows_icon_path(nightly))
        .set("ProductName", if nightly { "SoundZ Nightly" } else { "SoundZ" })
        .set("FileDescription", if nightly { "SoundZ Nightly digital audio workstation" } else { "SoundZ digital audio workstation" })
        .set("CompanyName", "Learning Machines LLC")
        .set("LegalCopyright", "Copyright (c) the SoundCraft authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "soundz.exe")
        .set("InternalName", "soundz");
    if nightly {
        // The Launcher verifies the PE ProductVersion against the exact feed identity, not FileVersion.
        res.set("ProductVersion", &built_version);
    }
    if let Err(e) = res.compile() {
        if std::env::var_os("SOUNDZ_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=soundz.exe built without icon/version resources: {e}");
    }
}
