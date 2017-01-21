extern crate pkg_config;

use std::{env, fs, process};
use std::path::PathBuf;

macro_rules! cmd(($name:expr) => (process::Command::new($name)));
macro_rules! get(($name:expr) => (ok!(env::var($name))));
macro_rules! ok(($result:expr) => ($result.unwrap()));

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(process::Stdio::inherit())
                        .stderr(process::Stdio::inherit())
                        .status().unwrap().success());
    );
);

fn main() {
    if pkg_config::find_library("ao").is_ok() {
        return;
    }

    let source = PathBuf::from(&get!("CARGO_MANIFEST_DIR")).join("source");
    let output = PathBuf::from(&get!("OUT_DIR"));

    let build = output.join("build");
    ok!(fs::create_dir_all(&build));

    run!(cmd!(source.join("configure")).current_dir(&build)
                                       .arg(&format!("--prefix={}", output.display())));

    run!(cmd!("make").current_dir(&build).arg("install"));

    println!("cargo:rustc-link-lib=dylib=ao");
    println!("cargo:rustc-link-search={}", output.join("lib").display());
}
