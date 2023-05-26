/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.39
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V0Period0Period39ExtSensorsData {
    #[serde(rename = "consumed_energy", skip_serializing_if = "Option::is_none")]
    pub consumed_energy: Option<Box<crate::models::V0Period0Period39Uint64NoVal>>,
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Box<crate::models::V0Period0Period39Uint32NoVal>>,
    #[serde(rename = "energy_update_time", skip_serializing_if = "Option::is_none")]
    pub energy_update_time: Option<i64>,
    #[serde(rename = "current_watts", skip_serializing_if = "Option::is_none")]
    pub current_watts: Option<i32>,
}

impl V0Period0Period39ExtSensorsData {
    pub fn new() -> V0Period0Period39ExtSensorsData {
        V0Period0Period39ExtSensorsData {
            consumed_energy: None,
            temperature: None,
            energy_update_time: None,
            current_watts: None,
        }
    }
}


