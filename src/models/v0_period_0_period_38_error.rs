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
pub struct V0Period0Period38Error {
    /// error message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Slurm internal error number
    #[serde(rename = "error_number", skip_serializing_if = "Option::is_none")]
    pub error_number: Option<i32>,
}

impl V0Period0Period38Error {
    pub fn new() -> V0Period0Period38Error {
        V0Period0Period38Error {
            error: None,
            error_number: None,
        }
    }
}


