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
pub struct V0Period0Period39JobSubmission {
    /// Executable script (full contents) to run in batch step for all job components
    #[serde(rename = "script", skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(rename = "job", skip_serializing_if = "Option::is_none")]
    pub job: Option<Box<crate::models::V0Period0Period39JobDescMsg>>,
    #[serde(rename = "jobs", skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<crate::models::V0Period0Period39JobDescMsg>>,
}

impl V0Period0Period39JobSubmission {
    pub fn new() -> V0Period0Period39JobSubmission {
        V0Period0Period39JobSubmission {
            script: None,
            job: None,
            jobs: None,
        }
    }
}


