use std::{
    env, fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

/// Writes `BUILD_ID` (`<version>-<commit12>[-dirty]-<epoch>`), `GIT_COMMIT`
/// and `BUILD_EPOCH` for `crate::build_info`.
fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
    println!("cargo:rerun-if-changed=.git/refs/heads/main");
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    let git = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|v| v.status.success())
        .map(|v| String::from_utf8_lossy(&v.stdout).trim().to_owned())
        .unwrap_or_else(|| "uncommitted".to_owned());
    let dirty = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map(|v| !v.stdout.is_empty())
        .unwrap_or(true);
    let epoch = env::var("SOURCE_DATE_EPOCH")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        });
    let id = format!(
        "{}-{}{}-{epoch}",
        env!("CARGO_PKG_VERSION"),
        &git[..git.len().min(12)],
        if dirty { "-dirty" } else { "" }
    );
    let data = format!(
        "pub const BUILD_ID: &str = {id:?};
pub const GIT_COMMIT: &str = {git:?};
pub const BUILD_EPOCH: u64 = {epoch};
"
    );
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").expect("Cargo OUT_DIR")).join("build_info.rs"),
        data,
    )
    .expect("write build metadata");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("switch") {
        // Bind this NRO's references to its own global functions, so a hook
        // callback can never resolve to a same-named symbol in another plugin.
        // Hook callbacks also keep the `ssbu_fx_control_` prefix.
        println!("cargo:rustc-cdylib-link-arg=-Bsymbolic-functions");
    }
}
