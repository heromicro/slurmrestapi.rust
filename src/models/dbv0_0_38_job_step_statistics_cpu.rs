/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.38
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0038JobStepStatisticsCpu : Statistics of CPU



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0038JobStepStatisticsCpu {
    /// Actual frequency of CPU during step
    #[serde(rename = "actual_frequency", skip_serializing_if = "Option::is_none")]
    pub actual_frequency: Option<i32>,
}

impl Dbv0038JobStepStatisticsCpu {
    /// Statistics of CPU
    pub fn new() -> Dbv0038JobStepStatisticsCpu {
        Dbv0038JobStepStatisticsCpu {
            actual_frequency: None,
        }
    }
}


