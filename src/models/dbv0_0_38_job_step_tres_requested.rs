/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.38
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0038JobStepTresRequested : TRES requested for job



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0038JobStepTresRequested {
    /// TRES list of attributes
    #[serde(rename = "average", skip_serializing_if = "Option::is_none")]
    pub average: Option<Vec<crate::models::Dbv0038TresListInner>>,
    /// TRES list of attributes
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<Vec<crate::models::Dbv0038TresListInner>>,
    /// TRES list of attributes
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<Vec<crate::models::Dbv0038TresListInner>>,
    /// TRES list of attributes
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<Vec<crate::models::Dbv0038TresListInner>>,
}

impl Dbv0038JobStepTresRequested {
    /// TRES requested for job
    pub fn new() -> Dbv0038JobStepTresRequested {
        Dbv0038JobStepTresRequested {
            average: None,
            max: None,
            min: None,
            total: None,
        }
    }
}


