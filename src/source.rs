use dirtcrunch::Source;

pub struct AmazonSellerPartner {}

impl AmazonSellerPartner {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl<'a> Source<'a> for AmazonSellerPartner {
    const IMAGE: &'a str = "airbyte/source-amazon-seller-partner";
    fn specs(&self) -> serde_json::Value { 
        serde_json::Value::String(r#" {"spec":{"changelogUrl":"https://docs.airbyte.io/integrations/sources/amazon-seller-partner","connectionSpecification":{"definitions":{"AWSEnvironment":{"description":"An enumeration.","enum":["PRODUCTION","SANDBOX"],"title":"AWSEnvironment","type":"string"},"AWSRegion":{"description":"An enumeration.","enum":["AE","DE","PL","EG","ES","FR","IN","IT","NL","SA","SE","TR","UK","AU","JP","SG","US","BR","CA","MX","GB"],"title":"AWSRegion","type":"string"}},"properties":{"aws_access_key":{"airbyte_secret":true,"description":"AWS user access key","title":"Aws Access Key","type":"string"},"aws_environment":{"description":"An enumeration.","enum":["PRODUCTION","SANDBOX"],"title":"AWSEnvironment","type":"string"},"aws_secret_key":{"airbyte_secret":true,"description":"AWS user secret key","title":"Aws Secret Key","type":"string"},"lwa_app_id":{"airbyte_secret":true,"description":"Your login with amazon app id","title":"Lwa App Id","type":"string"},"lwa_client_secret":{"airbyte_secret":true,"description":"Your login with amazon client secret","title":"Lwa Client Secret","type":"string"},"refresh_token":{"airbyte_secret":true,"description":"The refresh token used obtained via authorization (can be passed to the client instead)","title":"Refresh Token","type":"string"},"region":{"description":"An enumeration.","enum":["AE","DE","PL","EG","ES","FR","IN","IT","NL","SA","SE","TR","UK","AU","JP","SG","US","BR","CA","MX","GB"],"title":"AWSRegion","type":"string"},"replication_start_date":{"description":"UTC date and time in the format 2017-01-25T00:00:00Z. Any data before this date will not be replicated.","examples":["2017-01-25T00:00:00Z"],"pattern":"^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$","title":"Replication Start Date","type":"string"},"role_arn":{"airbyte_secret":true,"description":"The role's arn (needs permission to 'Assume Role' STS)","title":"Role Arn","type":"string"}},"required":["replication_start_date","refresh_token","lwa_app_id","lwa_client_secret","aws_access_key","aws_secret_key","role_arn","aws_environment","region"],"title":"Amazon Seller Partner Spec","type":"object"},"documentationUrl":"https://docs.airbyte.io/integrations/sources/amazon-seller-partner"},"type":"SPEC"} "#.to_string())
    }
}

pub struct Asana {}

impl Asana {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl<'a> Source<'a> for Asana {
    const IMAGE: &'a str = "airbyte/source-asana";
    fn specs(&self) -> serde_json::Value { 
        serde_json::Value::String(r#" {"spec":{"authSpecification":{"auth_type":"oauth2.0","oauth2Specification":{"oauthFlowInitParameters":[["client_id"],["client_secret"]],"oauthFlowOutputParameters":[["refresh_token"]],"rootObject":["credentials","1"]}},"connectionSpecification":{"$schema":"http://json-schema.org/draft-07/schema#","additionalProperties":true,"properties":{"credentials":{"description":"Choose how to authenticate to Github","oneOf":[{"properties":{"option_title":{"const":"PAT Credentials","description":"PAT Credentials","title":"Credentials title","type":"string"},"personal_access_token":{"airbyte_secret":true,"description":"Asana Personal Access Token (generate yours <a href=\"https://app.asana.com/0/developer-console\">here</a>).","title":"Personal Access Token","type":"string"}},"required":["personal_access_token"],"title":"Authenticate with Personal Access Token","type":"object"},{"properties":{"client_id":{"airbyte_secret":true,"description":"","title":"","type":"string"},"client_secret":{"airbyte_secret":true,"description":"","title":"","type":"string"},"option_title":{"const":"OAuth Credentials","description":"OAuth Credentials","title":"Credentials title","type":"string"},"refresh_token":{"airbyte_secret":true,"description":"","title":"","type":"string"}},"required":["client_id","client_secret","refresh_token"],"title":"Authenticate via Asana (Oauth)","type":"object"}],"title":"Authentication mechanism","type":"object"}},"title":"Asana Spec","type":"object"},"documentationUrl":"https://docsurl.com"},"type":"SPEC"} "#.to_string())
    }
}

pub struct Chargebee {}

impl Chargebee {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl<'a> Source<'a> for Chargebee {
    const IMAGE: &'a str = "airbyte/source-chargebee";
    fn specs(&self) -> serde_json::Value { 
        serde_json::Value::String(r#" {"spec":{"connectionSpecification":{"$schema":"http://json-schema.org/draft-07/schema#","additionalProperties":false,"properties":{"product_catalog":{"description":"Product Catalog version of your Chargebee site. Instructions on how to find your version you may find <a href=\"https://apidocs.chargebee.com/docs/api?prod_cat_ver=2\">here</a> under `API Version` section.","enum":["1.0","2.0"],"title":"Product Catalog","type":"string"},"site":{"description":"The site prefix for your Chargebee instance.","examples":["airbyte-test"],"title":"Site","type":"string"},"site_api_key":{"airbyte_secret":true,"description":"The API key from your Chargebee instance.","examples":["test_3yzfanAXF66USdWC9wQcM555DQJkSYoppu"],"title":"API Key","type":"string"},"start_date":{"description":"UTC date and time in the format 2021-01-25T00:00:00Z. Any data before this date will not be replicated.","examples":["2021-01-25T00:00:00Z"],"pattern":"^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$","title":"Start Date","type":"string"}},"required":["site","site_api_key","start_date","product_catalog"],"title":"Chargebee Spec","type":"object"},"documentationUrl":"https://apidocs.chargebee.com/docs/api"},"type":"SPEC"} "#.to_string())
    }
}

