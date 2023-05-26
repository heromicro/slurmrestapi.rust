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
pub struct V0039PartitionInfoGroups {
    #[serde(rename = "allowed", skip_serializing_if = "Option::is_none")]
    pub allowed: Option<String>,
}

impl V0039PartitionInfoGroups {
    pub fn new() -> V0039PartitionInfoGroups {
        V0039PartitionInfoGroups {
            allowed: None,
        }
    }
}


