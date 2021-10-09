use dirtcrunch::{create_file, create_objects, get_specs};
use futures::future::join_all;
use std::fs;
use std::path::Path;

#[derive(serde::Deserialize, Debug)]
struct Source {
    name: String,
    #[serde(rename(deserialize = "sourceDefinitionId"))]
    source_definition_id: String,
    #[serde(rename(deserialize = "dockerRepository"))]
    docker_repository: String,
    #[serde(rename(deserialize = "dockerImageTag"))]
    docker_image_tag: String,
    #[serde(rename(deserialize = "sourceType"))]
    source_type: String,
    #[serde(rename(deserialize = "documentationUrl"))]
    documentation_url: String,
}

#[tokio::main]
async fn main() {
    let sources_list =
        "airbyte/airbyte-config/init/src/main/resources/seed/source_definitions.yaml";

    let sources: Vec<Source> = if Path::new(sources_list).exists() {
        let file = fs::read_to_string(sources_list)
            .expect("Could not read the list of source connectors.");

        let value: serde_yaml::Value = serde_yaml::from_str(&file).unwrap();
        serde_yaml::from_value(value).unwrap()
    } else {
        panic!("Could not find the list of source connectors.")
    };

    let mut tasks = Vec::new();

    for (count, source) in sources.iter().enumerate() {
        if count == 3 {
            break;
        }
        tasks.push(get_specs(&source.docker_repository))
    }

    let specs = join_all(tasks).await;

    let mut objects = Vec::new();

    for (count, (spec, source)) in specs.iter().zip(sources.iter()).enumerate() {
        if count == 3 {
            break;
        }

        let iter = source.name.split_whitespace();

        let mut name = String::new();
        for (count, value) in iter.enumerate() {
            if count == 3 {
                break;
            }
            name.push_str(value);
        }

        objects.push(create_objects(
            name.as_str(),
            &source.docker_repository,
            spec.clone(),
        ))
    }

    let file = create_file(objects.join("\n"));

    let path = Path::new("src/source.rs");
    // Write the source.rs file to disk.
    assert!(fs::write(path, &file).is_ok());
}
