use dirtcrunch::create_file;
use std::{env, fs};

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=airbyte");

    let sources_list =
        "airbyte/airbyte-config/init/src/main/resources/seed/source_definitions.yaml";

    let file =
        fs::read_to_string(sources_list).expect("Could not read the list of source connectors.");

    let sources: serde_yaml::Value = serde_yaml::from_str(&file).unwrap();

    let file = create_file(sources).await;
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("{}", src_dir);

    let source_path = format!("{}/src/sources.rs", src_dir);
    assert!(fs::write(source_path, &file).is_ok());
}
