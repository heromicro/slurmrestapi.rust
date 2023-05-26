/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.38
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0038JobStepTres : TRES usage



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0038JobStepTres {
    #[serde(rename = "requested", skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<crate::models::Dbv0038JobStepTresRequested>>,
    #[serde(rename = "consumed", skip_serializing_if = "Option::is_none")]
    pub consumed: Option<Box<crate::models::Dbv0038JobStepTresRequested>>,
    /// TRES list of attributes
    #[serde(rename = "allocated", skip_serializing_if = "Option::is_none")]
    pub allocated: Option<Vec<crate::models::Dbv0038TresListInner>>,
}

impl Dbv0038JobStepTres {
    /// TRES usage
    pub fn new() -> Dbv0038JobStepTres {
        Dbv0038JobStepTres {
            requested: None,
            consumed: None,
            allocated: None,
        }
    }
}


