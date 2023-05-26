/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.37
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0037JobArrayLimitsMax : Limits on array settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0037JobArrayLimitsMax {
    #[serde(rename = "running", skip_serializing_if = "Option::is_none")]
    pub running: Option<Box<crate::models::Dbv0037JobArrayLimitsMaxRunning>>,
}

impl Dbv0037JobArrayLimitsMax {
    /// Limits on array settings
    pub fn new() -> Dbv0037JobArrayLimitsMax {
        Dbv0037JobArrayLimitsMax {
            running: None,
        }
    }
}


