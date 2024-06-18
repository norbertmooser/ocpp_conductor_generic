use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomDataType {
    #[serde(rename = "vendorId")]
    pub vendor_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RegistrationStatusEnumType {
    Accepted,
    Pending,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfoType {
    pub custom_data: Option<CustomDataType>,
    pub reason_code: String,
    pub additional_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    pub custom_data: Option<CustomDataType>,
    pub current_time: String,
    pub interval: i64,
    pub status: RegistrationStatusEnumType,
    pub status_info: Option<StatusInfoType>,
}
