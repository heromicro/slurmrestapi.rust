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
pub struct V0039JobHet {
    #[serde(rename = "job_offset", skip_serializing_if = "Option::is_none")]
    pub job_offset: Option<Box<crate::models::V0Period0Period39Uint32NoVal>>,
}

impl V0039JobHet {
    pub fn new() -> V0039JobHet {
        V0039JobHet {
            job_offset: None,
        }
    }
}


