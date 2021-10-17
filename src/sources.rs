use dirtcrunch::Source;

pub struct AmazonSellerPartner;

impl AmazonSellerPartner {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Source for AmazonSellerPartner {
    const IMAGE: &'static str = "airbyte/source-amazon-seller-partner";
    fn specs(&self) -> serde_json::Value {
        serde_json::json!(r#" {"spec":{"changelogUrl":"https://docs.airbyte.io/integrations/sources/amazon-seller-partner","connectionSpecification":{"definitions":{"AWSEnvironment":{"description":"An enumeration.","enum":["PRODUCTION","SANDBOX"],"title":"AWSEnvironment","type":"string"},"AWSRegion":{"description":"An enumeration.","enum":["AE","DE","PL","EG","ES","FR","IN","IT","NL","SA","SE","TR","UK","AU","JP","SG","US","BR","CA","MX","GB"],"title":"AWSRegion","type":"string"}},"properties":{"aws_access_key":{"airbyte_secret":true,"description":"AWS user access key","title":"Aws Access Key","type":"string"},"aws_environment":{"description":"An enumeration.","enum":["PRODUCTION","SANDBOX"],"title":"AWSEnvironment","type":"string"},"aws_secret_key":{"airbyte_secret":true,"description":"AWS user secret key","title":"Aws Secret Key","type":"string"},"lwa_app_id":{"airbyte_secret":true,"description":"Your login with amazon app id","title":"Lwa App Id","type":"string"},"lwa_client_secret":{"airbyte_secret":true,"description":"Your login with amazon client secret","title":"Lwa Client Secret","type":"string"},"refresh_token":{"airbyte_secret":true,"description":"The refresh token used obtained via authorization (can be passed to the client instead)","title":"Refresh Token","type":"string"},"region":{"description":"An enumeration.","enum":["AE","DE","PL","EG","ES","FR","IN","IT","NL","SA","SE","TR","UK","AU","JP","SG","US","BR","CA","MX","GB"],"title":"AWSRegion","type":"string"},"replication_start_date":{"description":"UTC date and time in the format 2017-01-25T00:00:00Z. Any data before this date will not be replicated.","examples":["2017-01-25T00:00:00Z"],"pattern":"^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$","title":"Replication Start Date","type":"string"},"role_arn":{"airbyte_secret":true,"description":"The role's arn (needs permission to 'Assume Role' STS)","title":"Role Arn","type":"string"}},"required":["replication_start_date","refresh_token","lwa_app_id","lwa_client_secret","aws_access_key","aws_secret_key","role_arn","aws_environment","region"],"title":"Amazon Seller Partner Spec","type":"object"},"documentationUrl":"https://docs.airbyte.io/integrations/sources/amazon-seller-partner"},"type":"SPEC"} "#.to_string())
    }
}

pub struct Asana;

impl Asana {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Source for Asana {
    const IMAGE: &'static str = "airbyte/source-asana";
    fn specs(&self) -> serde_json::Value {
        serde_json::json!(r#" {"spec":{"authSpecification":{"auth_type":"oauth2.0","oauth2Specification":{"oauthFlowInitParameters":[["client_id"],["client_secret"]],"oauthFlowOutputParameters":[["refresh_token"]],"rootObject":["credentials","1"]}},"connectionSpecification":{"$schema":"http://json-schema.org/draft-07/schema#","additionalProperties":true,"properties":{"credentials":{"description":"Choose how to authenticate to Github","oneOf":[{"properties":{"option_title":{"const":"PAT Credentials","description":"PAT Credentials","title":"Credentials title","type":"string"},"personal_access_token":{"airbyte_secret":true,"description":"Asana Personal Access Token (generate yours <a href=\"https://app.asana.com/0/developer-console\">here</a>).","title":"Personal Access Token","type":"string"}},"required":["personal_access_token"],"title":"Authenticate with Personal Access Token","type":"object"},{"properties":{"client_id":{"airbyte_secret":true,"description":"","title":"","type":"string"},"client_secret":{"airbyte_secret":true,"description":"","title":"","type":"string"},"option_title":{"const":"OAuth Credentials","description":"OAuth Credentials","title":"Credentials title","type":"string"},"refresh_token":{"airbyte_secret":true,"description":"","title":"","type":"string"}},"required":["client_id","client_secret","refresh_token"],"title":"Authenticate via Asana (Oauth)","type":"object"}],"title":"Authentication mechanism","type":"object"}},"title":"Asana Spec","type":"object"},"documentationUrl":"https://docsurl.com"},"type":"SPEC"} "#.to_string())
    }
}

pub struct Chargebee;

impl Chargebee {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Source for Chargebee {
    const IMAGE: &'static str = "airbyte/source-chargebee";
    fn specs(&self) -> serde_json::Value {
        serde_json::json!(r#" {"spec":{"connectionSpecification":{"$schema":"http://json-schema.org/draft-07/schema#","additionalProperties":false,"properties":{"product_catalog":{"description":"Product Catalog version of your Chargebee site. Instructions on how to find your version you may find <a href=\"https://apidocs.chargebee.com/docs/api?prod_cat_ver=2\">here</a> under `API Version` section.","enum":["1.0","2.0"],"title":"Product Catalog","type":"string"},"site":{"description":"The site prefix for your Chargebee instance.","examples":["airbyte-test"],"title":"Site","type":"string"},"site_api_key":{"airbyte_secret":true,"description":"The API key from your Chargebee instance.","examples":["test_3yzfanAXF66USdWC9wQcM555DQJkSYoppu"],"title":"API Key","type":"string"},"start_date":{"description":"UTC date and time in the format 2021-01-25T00:00:00Z. Any data before this date will not be replicated.","examples":["2021-01-25T00:00:00Z"],"pattern":"^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$","title":"Start Date","type":"string"}},"required":["site","site_api_key","start_date","product_catalog"],"title":"Chargebee Spec","type":"object"},"documentationUrl":"https://apidocs.chargebee.com/docs/api"},"type":"SPEC"} "#.to_string())
    }
}

pub struct ExchangeRatesApi;

impl ExchangeRatesApi {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Source for ExchangeRatesApi {
    const IMAGE: &'static str = "airbyte/source-exchange-rates";
    fn specs(&self) -> serde_json::Value {
        serde_json::json!(r#" {"spec":{"connectionSpecification":{"$schema":"http://json-schema.org/draft-07/schema#","additionalProperties":false,"properties":{"access_key":{"airbyte_secret":true,"description":"Your API Access Key. See <a href=\"https://exchangeratesapi.io/documentation/\">here</a>. The key is case sensitive.","type":"string"},"base":{"description":"ISO reference currency. See <a href=\"https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html\">here</a>. Free plan doesn't support Source Currency Switching, default base currency is EUR","examples":["EUR","USD"],"type":"string"},"start_date":{"description":"Start getting data from that date.","examples":["YYYY-MM-DD"],"pattern":"^[0-9]{4}-[0-9]{2}-[0-9]{2}$","type":"string"}},"required":["start_date","access_key"],"title":"ratesapi.io Source Spec","type":"object"},"documentationUrl":"https://docs.airbyte.io/integrations/sources/exchangeratesapi"},"type":"SPEC"} "#.to_string())
    }
}

pub struct File;

impl File {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Source for File {
    const IMAGE: &'static str = "airbyte/source-file";
    fn specs(&self) -> serde_json::Value {
        serde_json::json!(r#" {"spec":{"connectionSpecification":{"$schema":"http://json-schema.org/draft-07/schema#","additionalProperties":false,"properties":{"dataset_name":{"description":"Name of the final table where to replicate this file (should include only letters, numbers dash and underscores)","type":"string"},"format":{"default":"csv","description":"File Format of the file to be replicated (Warning: some format may be experimental, please refer to docs).","enum":["csv","json","jsonl","excel","feather","parquet"],"type":"string"},"provider":{"default":"Public Web","description":"Storage Provider or Location of the file(s) to be replicated.","oneOf":[{"properties":{"storage":{"default":"HTTPS","enum":["HTTPS"],"type":"string"}},"required":["storage"],"title":"HTTPS: Public Web"},{"properties":{"service_account_json":{"description":"In order to access private Buckets stored on Google Cloud, this connector would need a service account json credentials with the proper permissions as described <a href=\"https://cloud.google.com/iam/docs/service-accounts\" target=\"_blank\">here</a>. Please generate the credentials.json file and copy/paste its content to this field (expecting JSON formats). If accessing publicly available data, this field is not necessary.","type":"string"},"storage":{"default":"GCS","enum":["GCS"],"type":"string"}},"required":["storage"],"title":"GCS: Google Cloud Storage"},{"properties":{"aws_access_key_id":{"description":"In order to access private Buckets stored on AWS S3, this connector would need credentials with the proper permissions. If accessing publicly available data, this field is not necessary.","type":"string"},"aws_secret_access_key":{"airbyte_secret":true,"description":"In order to access private Buckets stored on AWS S3, this connector would need credentials with the proper permissions. If accessing publicly available data, this field is not necessary.","type":"string"},"storage":{"default":"S3","enum":["S3"],"type":"string"}},"required":["storage"],"title":"S3: Amazon Web Services"},{"properties":{"sas_token":{"airbyte_secret":true,"description":"To access Azure Blob Storage, this connector would need credentials with the proper permissions. One option is a SAS (Shared Access Signature) token. If accessing publicly available data, this field is not necessary.","type":"string"},"shared_key":{"airbyte_secret":true,"description":"To access Azure Blob Storage, this connector would need credentials with the proper permissions. One option is a storage account shared key (aka account key or access key). If accessing publicly available data, this field is not necessary.","type":"string"},"storage":{"default":"AzBlob","enum":["AzBlob"],"type":"string"},"storage_account":{"description":"The globally unique name of the storage account that the desired blob sits within. See <a href=\"https://docs.microsoft.com/en-us/azure/storage/common/storage-account-overview\" target=\"_blank\">here</a> for more details.","type":"string"}},"required":["storage","storage_account"],"title":"AzBlob: Azure Blob Storage"},{"properties":{"host":{"type":"string"},"password":{"airbyte_secret":true,"type":"string"},"port":{"default":"22","type":"string"},"storage":{"default":"SSH","enum":["SSH"],"type":"string"},"user":{"type":"string"}},"required":["storage","user","host"],"title":"SSH: Secure Shell"},{"properties":{"host":{"type":"string"},"password":{"airbyte_secret":true,"type":"string"},"port":{"default":"22","type":"string"},"storage":{"default":"SCP","enum":["SCP"],"type":"string"},"user":{"type":"string"}},"required":["storage","user","host"],"title":"SCP: Secure copy protocol"},{"properties":{"host":{"type":"string"},"password":{"airbyte_secret":true,"type":"string"},"port":{"default":"22","type":"string"},"storage":{"default":"SFTP","enum":["SFTP"],"type":"string"},"user":{"type":"string"}},"required":["storage","user","host"],"title":"SFTP: Secure File Transfer Protocol"},{"properties":{"storage":{"default":"local","description":"WARNING: Note that local storage URL available for read must start with the local mount \"/local/\" at the moment until we implement more advanced docker mounting options...","enum":["local"],"type":"string"}},"required":["storage"],"title":"Local Filesystem (limited)"}],"type":"object"},"reader_options":{"description":"This should be a valid JSON string used by each reader/parser to provide additional options and tune its behavior","examples":["{}","{'sep': ' '}"],"type":"string"},"url":{"description":"URL path to access the file to be replicated","type":"string"}},"required":["dataset_name","format","url","provider"],"title":"File Source Spec","type":"object"},"documentationUrl":"https://docs.airbyte.io/integrations/sources/file"},"type":"SPEC"} "#.to_string())
    }
}
