/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.37
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0037JobTimeTotal : System time values



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0037JobTimeTotal {
    /// Total number of CPU-seconds used by the job, in seconds
    #[serde(rename = "seconds", skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i32>,
    /// Total number of CPU-seconds used by the job, in microseconds
    #[serde(rename = "microseconds", skip_serializing_if = "Option::is_none")]
    pub microseconds: Option<i32>,
}

impl Dbv0037JobTimeTotal {
    /// System time values
    pub fn new() -> Dbv0037JobTimeTotal {
        Dbv0037JobTimeTotal {
            seconds: None,
            microseconds: None,
        }
    }
}


