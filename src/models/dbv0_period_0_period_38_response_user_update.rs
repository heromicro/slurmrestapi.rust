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
pub struct Dbv0Period0Period38ResponseUserUpdate {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Dbv0Period0Period38Meta>>,
    /// Slurm errors
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::Dbv0Period0Period38Error>>,
}

impl Dbv0Period0Period38ResponseUserUpdate {
    pub fn new() -> Dbv0Period0Period38ResponseUserUpdate {
        Dbv0Period0Period38ResponseUserUpdate {
            meta: None,
            errors: None,
        }
    }
}


