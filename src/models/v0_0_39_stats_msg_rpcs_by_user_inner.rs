/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.39
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// V0039StatsMsgRpcsByUserInner : user



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V0039StatsMsgRpcsByUserInner {
    /// user name
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// user id (numeric)
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// Number of RPCs received
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Average time spent processing RPC in seconds
    #[serde(rename = "average_time", skip_serializing_if = "Option::is_none")]
    pub average_time: Option<i64>,
    /// Total time spent processing RPC in seconds
    #[serde(rename = "total_time", skip_serializing_if = "Option::is_none")]
    pub total_time: Option<i64>,
}

impl V0039StatsMsgRpcsByUserInner {
    /// user
    pub fn new() -> V0039StatsMsgRpcsByUserInner {
        V0039StatsMsgRpcsByUserInner {
            user: None,
            user_id: None,
            count: None,
            average_time: None,
            total_time: None,
        }
    }
}


