/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.38
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0038QosLimitsMax : Limits on max settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0038QosLimitsMax {
    #[serde(rename = "wall_clock", skip_serializing_if = "Option::is_none")]
    pub wall_clock: Option<Box<crate::models::Dbv0038QosLimitsMaxWallClock>>,
    #[serde(rename = "jobs", skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Box<crate::models::Dbv0038QosLimitsMaxJobs>>,
    #[serde(rename = "accruing", skip_serializing_if = "Option::is_none")]
    pub accruing: Option<Box<crate::models::Dbv0038QosLimitsMaxAccruing>>,
    #[serde(rename = "tres", skip_serializing_if = "Option::is_none")]
    pub tres: Option<Box<crate::models::Dbv0038QosLimitsMaxTres>>,
}

impl Dbv0038QosLimitsMax {
    /// Limits on max settings
    pub fn new() -> Dbv0038QosLimitsMax {
        Dbv0038QosLimitsMax {
            wall_clock: None,
            jobs: None,
            accruing: None,
            tres: None,
        }
    }
}


