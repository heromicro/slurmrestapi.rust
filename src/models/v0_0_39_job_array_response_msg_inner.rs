/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.39
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// V0039JobArrayResponseMsgInner : ArrayJob



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V0039JobArrayResponseMsgInner {
    /// JobId
    #[serde(rename = "job_id", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i32>,
    /// numeric error code
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// error code description
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// error message
    #[serde(rename = "why", skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
}

impl V0039JobArrayResponseMsgInner {
    /// ArrayJob
    pub fn new() -> V0039JobArrayResponseMsgInner {
        V0039JobArrayResponseMsgInner {
            job_id: None,
            error_code: None,
            error: None,
            why: None,
        }
    }
}


