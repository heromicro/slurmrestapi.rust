/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.38
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0038AssociationMaxPerAccount : Max per accounting settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0038AssociationMaxPerAccount {
    /// Max wallclock per account
    #[serde(rename = "wall_clock", skip_serializing_if = "Option::is_none")]
    pub wall_clock: Option<i32>,
}

impl Dbv0038AssociationMaxPerAccount {
    /// Max per accounting settings
    pub fn new() -> Dbv0038AssociationMaxPerAccount {
        Dbv0038AssociationMaxPerAccount {
            wall_clock: None,
        }
    }
}


