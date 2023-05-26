/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.38
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0038QosLimitsMaxWallClockPer : Limit on wallclock per settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0038QosLimitsMaxWallClockPer {
    /// Max wallclock per QOS
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    /// Max wallclock per job
    #[serde(rename = "job", skip_serializing_if = "Option::is_none")]
    pub job: Option<i32>,
}

impl Dbv0038QosLimitsMaxWallClockPer {
    /// Limit on wallclock per settings
    pub fn new() -> Dbv0038QosLimitsMaxWallClockPer {
        Dbv0038QosLimitsMaxWallClockPer {
            qos: None,
            job: None,
        }
    }
}


