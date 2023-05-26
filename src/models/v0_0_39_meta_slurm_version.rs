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
pub struct V0039MetaSlurmVersion {
    /// 
    #[serde(rename = "major", skip_serializing_if = "Option::is_none")]
    pub major: Option<i32>,
    /// 
    #[serde(rename = "micro", skip_serializing_if = "Option::is_none")]
    pub micro: Option<i32>,
    /// 
    #[serde(rename = "minor", skip_serializing_if = "Option::is_none")]
    pub minor: Option<i32>,
}

impl V0039MetaSlurmVersion {
    pub fn new() -> V0039MetaSlurmVersion {
        V0039MetaSlurmVersion {
            major: None,
            micro: None,
            minor: None,
        }
    }
}


