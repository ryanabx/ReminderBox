use std::process::Command;
use std::str;

fn main() {
    // Run the `git rev-parse --short HEAD` command
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let commit_hash = str::from_utf8(&output.stdout).unwrap().trim();

    // Tell Cargo to pass the result as an environment variable
    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", commit_hash);

    // Tell Cargo that if the HEAD changes, the script needs to be rerun
    println!("cargo:rerun-if-changed=.git/HEAD");
}
