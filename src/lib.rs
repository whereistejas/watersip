pub mod sources;

use sources::File;

pub mod file_commands {
    use crate::File;
    use dirtcrunch::{Docker, Source};
    use futures::StreamExt;

    pub async fn discover() -> String {
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

        catalog
    }

    pub async fn read() -> String {
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

        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_commands::{discover, read};

    #[tokio::test]
    async fn test_discover() {
        let catalog = discover().await;

        println!("{:?}", catalog);

        assert!(
            catalog
                .contains(r#""properties": {"1": {"type": "number"}, "2": {"type": "number"}, "3": {"type": "number"}, "4": {"type": "number"}}"#)
        );
    }

    #[tokio::test]
    async fn test_read() {
        let lines = read().await;

        println!("{:?}", lines);

        assert!(lines.contains("\"type\": \"RECORD\""));
    }
}
