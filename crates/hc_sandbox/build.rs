/// There isn't a great way to get a path to the target directory at runtime,
/// so we build it in at compile time. The code generated by this build step
/// will only be used in integration tests, not the sandbox binaries themselves.
fn main() {
    let out_dir: std::path::PathBuf = std::env::var_os("OUT_DIR").unwrap().into();

    let mut target_dir = out_dir.clone();
    target_dir.pop();
    target_dir.pop();
    target_dir.pop();

    let content = format!(
        "const TARGET: &[u8] = &{:?};",
        target_dir.into_os_string().into_encoded_bytes(),
    );

    let mut target_file = out_dir.clone();
    target_file.push("target.rs");

    std::fs::write(target_file, content).unwrap();
}