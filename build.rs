use dirtcrunch::{create_file, create_objects, get_specs};
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    let json = get_specs("airbyte/source-file").await;
    let objects = create_objects("File", "airbyte/source-file", json);
    let file = create_file(objects);

    let path = Path::new("src/source_file.rs");

    assert!(fs::write(path, &file).is_ok());
}
