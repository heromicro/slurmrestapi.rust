/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.37
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0037DiagStatisticsRpcsInnerTime : Time values



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0037DiagStatisticsRpcsInnerTime {
    /// Average time spent processing this RPC type
    #[serde(rename = "average", skip_serializing_if = "Option::is_none")]
    pub average: Option<i32>,
    /// Total time spent processing this RPC type
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl Dbv0037DiagStatisticsRpcsInnerTime {
    /// Time values
    pub fn new() -> Dbv0037DiagStatisticsRpcsInnerTime {
        Dbv0037DiagStatisticsRpcsInnerTime {
            average: None,
            total: None,
        }
    }
}


