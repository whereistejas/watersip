use dirtcrunch::{create_file, get_objects};
use std::{env, fs};

#[tokio::main]
async fn main() {
    let sources_list =
        "airbyte/airbyte-config/init/src/main/resources/seed/source_definitions.yaml";

    let file =
        fs::read_to_string(sources_list).expect("Could not read the list of source connectors.");

    let sources: serde_yaml::Value = serde_yaml::from_str(&file).unwrap();

    let objects = get_objects(sources).await;
    let file = create_file(objects);

    let src_dir = format!("{}/src/", env::var("CARGO_MANIFEST_DIR").unwrap());

    let source_path = format!("{}/sources.rs", src_dir);
    assert!(fs::write(source_path, &file).is_ok());
}
