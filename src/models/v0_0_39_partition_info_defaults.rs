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
pub struct V0039PartitionInfoDefaults {
    #[serde(rename = "job", skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,
}

impl V0039PartitionInfoDefaults {
    pub fn new() -> V0039PartitionInfoDefaults {
        V0039PartitionInfoDefaults {
            job: None,
        }
    }
}


