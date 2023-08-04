use std::{path::PathBuf, process::Command};

fn main() {
    let crate_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir: PathBuf = crate_dir.parent().unwrap().parent().unwrap().to_path_buf();

    let func_name = crate_dir.file_name().unwrap().to_string_lossy();

    let src_file = crate_dir.join(format!("target/debug/lib{}.so", func_name));
    let dst_file = workspace_dir.join(format!("target/debug/lib{}.so", func_name));

    if dst_file.is_symlink() {
        return;
    }

    let mut ln_commd = Command::new("ln");
    ln_commd.arg("-s");
    ln_commd.args([src_file, dst_file.to_owned()]);
    assert!(
        ln_commd.status().unwrap().success(),
        "ln -s failed, command: {:?}",
        ln_commd
    );

    assert!(dst_file.is_symlink(), "build symbol link failed.")
}
