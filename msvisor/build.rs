use std::{
    fs::{read, write},
    path::PathBuf,
};
use toml::Table;

fn build_members() -> Vec<String> {
    let toml_context = {
        let file_path = PathBuf::new()
            .join(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("wrong parent")
            .join("Cargo.toml");

        let context = read(file_path).expect("read toml failed.");
        String::from_utf8(context).unwrap()
    };
    // assert!(toml_context.contains("workspace"))

    let workspace_members = {
        let value = toml_context.parse::<Table>().unwrap();
        value["workspace"].as_table().expect("missing workspace?")["members"]
            .as_array()
            .expect("missing members?")
            .clone()
    };
    // assert!(workspace_members.len() > 1, "{:?}", workspace_members);

    let mut result = vec![];
    for member in workspace_members {
        let t = PathBuf::from(member.as_str().unwrap());
        let name = t.file_name().unwrap().to_str().unwrap();
        if !name.eq("msvisor") {
            result.push(name.to_owned())
        }
    }
    result
}

fn main() {
    let members = build_members();
    let file_path = PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("wrong parent")
        .join("_members.txt");

    write(file_path, members.join(" ")).expect("write members failed.");
}
