use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomDataType {
    #[serde(rename = "vendorId")]
    pub vendor_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HashAlgorithmEnumType {
    SHA256,
    SHA384,
    SHA512,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IdTokenEnumType {
    Central,
    EMAID,
    ISO14443,
    ISO15693,
    KeyCode,
    Local,
    MacAddress,
    NoAuthorization,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfoType {
    pub custom_data: Option<CustomDataType>,
    pub additional_id_token: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenType {
    pub custom_data: Option<CustomDataType>,
    pub additional_info: Option<Vec<AdditionalInfoType>>,
    pub id_token: String,
    pub r#type: IdTokenEnumType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType {
    pub custom_data: Option<CustomDataType>,
    pub hash_algorithm: HashAlgorithmEnumType,
    pub issuer_name_hash: String,
    pub issuer_key_hash: String,
    pub serial_number: String,
    pub responder_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    pub custom_data: Option<CustomDataType>,
    pub id_token: IdTokenType,
    pub certificate: Option<String>,
    pub iso15118_certificate_hash_data: Option<Vec<OCSPRequestDataType>>,
}
