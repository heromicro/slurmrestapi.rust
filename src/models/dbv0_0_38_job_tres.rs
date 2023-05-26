/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.38
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0038JobTres : TRES settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0038JobTres {
    /// TRES list of attributes
    #[serde(rename = "allocated", skip_serializing_if = "Option::is_none")]
    pub allocated: Option<Vec<crate::models::Dbv0038TresListInner>>,
    /// TRES list of attributes
    #[serde(rename = "requested", skip_serializing_if = "Option::is_none")]
    pub requested: Option<Vec<crate::models::Dbv0038TresListInner>>,
}

impl Dbv0038JobTres {
    /// TRES settings
    pub fn new() -> Dbv0038JobTres {
        Dbv0038JobTres {
            allocated: None,
            requested: None,
        }
    }
}


