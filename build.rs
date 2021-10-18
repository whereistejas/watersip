use dirtcrunch::create_file;
use std::{env, fs, io::Write};

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=airbyte");

    let sources_list =
        "airbyte/airbyte-config/init/src/main/resources/seed/source_definitions.yaml";

    let file =
        fs::read_to_string(sources_list).expect("Could not read the list of source connectors.");

    let sources: serde_yaml::Value = serde_yaml::from_str(&file).unwrap();

    let file = create_file(sources).await;

    let source_path = format!("{}/src/sources.rs", env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut f = fs::OpenOptions::new()
        .write(true)
        .open(source_path)
        .unwrap();

    assert!(f.write_all(file.as_bytes()).is_ok());
}
