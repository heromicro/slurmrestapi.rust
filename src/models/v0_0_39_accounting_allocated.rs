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
pub struct V0039AccountingAllocated {
    #[serde(rename = "seconds", skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i64>,
}

impl V0039AccountingAllocated {
    pub fn new() -> V0039AccountingAllocated {
        V0039AccountingAllocated {
            seconds: None,
        }
    }
}


