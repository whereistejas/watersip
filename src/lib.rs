pub mod source_file;

#[cfg(test)]
mod tests {
    use crate::source_file::*;
    use dirtcrunch::Source;

    #[tokio::test]
    async fn it_works() {
        let file = File::new();

        let json = r#"
{
    "dataset_name": "some",
    "format": "csv",
    "url": "/app/some.csv",
    "provider": {
        "storage": "local"
    }
}
        "#;

        let config: serde_json::Value = serde_json::from_str(json).unwrap();

        let catalog = file.discover(&config).await;

        println!("{:?}", catalog);

        assert!(
            catalog[0]
                .contains("\"properties\": {\"1\": {\"type\": \"string\"}, \"2\": {\"type\": \"string\"}, \"3\": {\"type\": \"string\"}, \"4\": {\"type\": \"string\"}}")
        );
    }
}
