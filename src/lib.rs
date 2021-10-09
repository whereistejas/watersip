pub mod source_file;

#[cfg(test)]
mod tests {
    use crate::source_file::*;
    use dirtcrunch::{Docker, Source};
    use futures::StreamExt;

    #[tokio::test]
    async fn file_discover() {
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
                .contains(r#""properties": {"1": {"type": "number"}, "2": {"type": "number"}, "3": {"type": "number"}, "4": {"type": "number"}}"#)
        );
    }

    #[tokio::test]
    async fn file_read() {
        let file = File::new();

        let config_json = r#"
{
    "dataset_name": "some",
    "format": "csv",
    "url": "/app/some.csv",
    "provider": {
        "storage": "local"
    }
}
        "#;

        let catalog_json = r#"
{
    "streams": [
        {
            "stream": {
                "name": "some",
                "json_schema": {
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "type": "object",
                    "properties": {
                        "1": {
                            "type": "number"
                        },
                        "2": {
                            "type": "number"
                        },
                        "3": {
                            "type": "number"
                        },
                        "4": {
                            "type": "number"
                        }
                    }
                }
            },
            "sync_mode": "full_refresh",
            "destination_sync_mode": "overwrite"
        }
    ]
}
        "#;

        let config: serde_json::Value = serde_json::from_str(config_json).unwrap();
        let catalog: serde_json::Value = serde_json::from_str(catalog_json).unwrap();

        let docker = Docker::new();
        let mut reader = file.read(&docker, &config, &catalog).await;

        let mut lines = String::new();

        while let Some(blahblah) = reader.next().await {
            lines.push_str(blahblah.as_str())
        }

        println!("{:?}", lines);

        assert!(lines.contains("\"type\": \"RECORD\""));
    }
}
