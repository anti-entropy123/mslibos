use lazy_static::lazy_static;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

lazy_static! {
    static ref WORKSPACE_ROOT_DIR: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
}

fn user_manifest_dirs() -> Vec<PathBuf> {
    let all_dir_entry = {
        // let mut app_dir: Vec<_> = fs::read_dir(WORKSPACE_ROOT_DIR.join("user"))
        //     .unwrap()
        //     .collect();
        let mut common_dir: Vec<_> = fs::read_dir(WORKSPACE_ROOT_DIR.join("common_service"))
            .unwrap()
            .collect();

        let mut all_entries = vec![];
        all_entries.append(&mut common_dir);
        // all_entries.append(&mut app_dir);
        all_entries
    };

    let mut result = vec![];
    for entry in all_dir_entry {
        let entry = entry.unwrap().path().join("Cargo.toml");

        if Path::is_file(&entry) {
            result.push(entry.parent().unwrap().to_owned())
        }
    }

    result
}

fn main() {
    return;
    let target_dirs = user_manifest_dirs();

    for lib_dir in target_dirs {
        let mut cargo_commd = Command::new("cargo");
        cargo_commd.args([
            "build",
            "--manifest-path",
            &lib_dir.join("Cargo.toml").to_string_lossy(),
        ]);
        assert!(
            cargo_commd.status().unwrap().success(),
            "build {:?} failed.",
            &lib_dir
        );

        let mut cp_commd = Command::new("cp");
        cp_commd.args([
            lib_dir.join(format!(
                "target/debug/lib{}.so",
                lib_dir.file_name().unwrap().to_string_lossy()
            )),
            WORKSPACE_ROOT_DIR.join("target/debug/"),
        ]);
        assert!(
            cp_commd.status().unwrap().success(),
            "cp failed, command: {:?}",
            cp_commd
        );
    }
}
