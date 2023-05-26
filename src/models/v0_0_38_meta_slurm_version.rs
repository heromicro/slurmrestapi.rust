/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.38
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V0038MetaSlurmVersion {
    /// 
    #[serde(rename = "major", skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    /// 
    #[serde(rename = "micro", skip_serializing_if = "Option::is_none")]
    pub micro: Option<String>,
    /// 
    #[serde(rename = "minor", skip_serializing_if = "Option::is_none")]
    pub minor: Option<String>,
}

impl V0038MetaSlurmVersion {
    pub fn new() -> V0038MetaSlurmVersion {
        V0038MetaSlurmVersion {
            major: None,
            micro: None,
            minor: None,
        }
    }
}


