use dirtcrunch::{create_file, create_objects, get_specs};
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Retrieve the SPECS for the `source-file` connector.
    let json = get_specs("airbyte/source-file").await;
    // Create a struct called `File` and implement the `Source` for the `source-file` connector.
    let objects = create_objects("File", "airbyte/source-file", json);
    // Create a .rs file with imports, connector structs and an implementation of the Source trait.
    let file = create_file(objects);

    let path = Path::new("src/source_file.rs");

    // Write the source.rs file to disk.
    assert!(fs::write(path, &file).is_ok());
}
