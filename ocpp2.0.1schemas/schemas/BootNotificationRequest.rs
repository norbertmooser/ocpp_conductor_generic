use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomDataType {
    #[serde(rename = "vendorId")]
    pub vendor_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModemType {
    pub custom_data: Option<CustomDataType>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStationType {
    pub custom_data: Option<CustomDataType>,
    pub serial_number: Option<String>,
    pub model: String,
    pub modem: Option<ModemType>,
    pub vendor_name: String,
    pub firmware_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BootReasonEnumType {
    ApplicationReset,
    FirmwareUpdate,
    LocalReset,
    PowerUp,
    RemoteReset,
    ScheduledReset,
    Triggered,
    Unknown,
    Watchdog,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    pub custom_data: Option<CustomDataType>,
    pub charging_station: ChargingStationType,
    pub reason: BootReasonEnumType,
}