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
pub struct V0039StepStatisticsEnergy {
    #[serde(rename = "consumed", skip_serializing_if = "Option::is_none")]
    pub consumed: Option<Box<crate::models::V0Period0Period39Uint64NoVal>>,
}

impl V0039StepStatisticsEnergy {
    pub fn new() -> V0039StepStatisticsEnergy {
        V0039StepStatisticsEnergy {
            consumed: None,
        }
    }
}


