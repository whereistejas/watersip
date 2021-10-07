use dirtcrunch::{Command, Source};
use serde_json::Value;
use async_trait::async_trait;

pub struct Config {
dataset_name:String,
format:String,
url:String,
provider:String,

}


struct SourceFile {}

impl SourceFile {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Source<Config> for SourceFile {
    fn specs(&self) -> Command
    { 
        let value: Value = Value::String(r#" {"spec":{"connectionSpecification":{"$schema":"http://json-schema.org/draft-07/schema#","additionalProperties":false,"properties":{"dataset_name":{"description":"Name of the final table where to replicate this file (should include only letters, numbers dash and underscores)","type":"string"},"format":{"default":"csv","description":"File Format of the file to be replicated (Warning: some format may be experimental, please refer to docs).","enum":["csv","json","jsonl","excel","feather","parquet"],"type":"string"},"provider":{"default":"Public Web","description":"Storage Provider or Location of the file(s) to be replicated.","oneOf":[{"properties":{"storage":{"default":"HTTPS","enum":["HTTPS"],"type":"string"}},"required":["storage"],"title":"HTTPS: Public Web"},{"properties":{"service_account_json":{"description":"In order to access private Buckets stored on Google Cloud, this connector would need a service account json credentials with the proper permissions as described <a href=\"https://cloud.google.com/iam/docs/service-accounts\" target=\"_blank\">here</a>. Please generate the credentials.json file and copy/paste its content to this field (expecting JSON formats). If accessing publicly available data, this field is not necessary.","type":"string"},"storage":{"default":"GCS","enum":["GCS"],"type":"string"}},"required":["storage"],"title":"GCS: Google Cloud Storage"},{"properties":{"aws_access_key_id":{"description":"In order to access private Buckets stored on AWS S3, this connector would need credentials with the proper permissions. If accessing publicly available data, this field is not necessary.","type":"string"},"aws_secret_access_key":{"airbyte_secret":true,"description":"In order to access private Buckets stored on AWS S3, this connector would need credentials with the proper permissions. If accessing publicly available data, this field is not necessary.","type":"string"},"storage":{"default":"S3","enum":["S3"],"type":"string"}},"required":["storage"],"title":"S3: Amazon Web Services"},{"properties":{"sas_token":{"airbyte_secret":true,"description":"To access Azure Blob Storage, this connector would need credentials with the proper permissions. One option is a SAS (Shared Access Signature) token. If accessing publicly available data, this field is not necessary.","type":"string"},"shared_key":{"airbyte_secret":true,"description":"To access Azure Blob Storage, this connector would need credentials with the proper permissions. One option is a storage account shared key (aka account key or access key). If accessing publicly available data, this field is not necessary.","type":"string"},"storage":{"default":"AzBlob","enum":["AzBlob"],"type":"string"},"storage_account":{"description":"The globally unique name of the storage account that the desired blob sits within. See <a href=\"https://docs.microsoft.com/en-us/azure/storage/common/storage-account-overview\" target=\"_blank\">here</a> for more details.","type":"string"}},"required":["storage","storage_account"],"title":"AzBlob: Azure Blob Storage"},{"properties":{"host":{"type":"string"},"password":{"airbyte_secret":true,"type":"string"},"port":{"default":"22","type":"string"},"storage":{"default":"SSH","enum":["SSH"],"type":"string"},"user":{"type":"string"}},"required":["storage","user","host"],"title":"SSH: Secure Shell"},{"properties":{"host":{"type":"string"},"password":{"airbyte_secret":true,"type":"string"},"port":{"default":"22","type":"string"},"storage":{"default":"SCP","enum":["SCP"],"type":"string"},"user":{"type":"string"}},"required":["storage","user","host"],"title":"SCP: Secure copy protocol"},{"properties":{"host":{"type":"string"},"password":{"airbyte_secret":true,"type":"string"},"port":{"default":"22","type":"string"},"storage":{"default":"SFTP","enum":["SFTP"],"type":"string"},"user":{"type":"string"}},"required":["storage","user","host"],"title":"SFTP: Secure File Transfer Protocol"},{"properties":{"storage":{"default":"local","description":"WARNING: Note that local storage URL available for read must start with the local mount \"/local/\" at the moment until we implement more advanced docker mounting options...","enum":["local"],"type":"string"}},"required":["storage"],"title":"Local Filesystem (limited)"}],"type":"object"},"reader_options":{"description":"This should be a valid JSON string used by each reader/parser to provide additional options and tune its behavior","examples":["{}","{'sep': ' '}"],"type":"string"},"url":{"description":"URL path to access the file to be replicated","type":"string"}},"required":["dataset_name","format","url","provider"],"title":"File Source Spec","type":"object"},"documentationUrl":"https://docs.airbyte.io/integrations/sources/file"},"type":"SPEC"} "#.to_string());

        Command::Spec(value)
    }
    async fn discover(&self, config: &Config) -> Command { todo!() }
    fn read(&self, config: &Config) -> Command { todo!() }
}
