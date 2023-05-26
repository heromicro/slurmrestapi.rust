/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.37
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0Period0Period37Qos : QOS description



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0Period0Period37Qos {
    /// QOS description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of properties of QOS
    #[serde(rename = "flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    /// Database id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<Box<crate::models::Dbv0037QosLimits>>,
    #[serde(rename = "preempt", skip_serializing_if = "Option::is_none")]
    pub preempt: Option<Box<crate::models::Dbv0037QosPreempt>>,
    /// QOS priority
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Usage factor
    #[serde(rename = "usage_factor", skip_serializing_if = "Option::is_none")]
    pub usage_factor: Option<f32>,
    /// Usage threshold
    #[serde(rename = "usage_threshold", skip_serializing_if = "Option::is_none")]
    pub usage_threshold: Option<f32>,
}

impl Dbv0Period0Period37Qos {
    /// QOS description
    pub fn new() -> Dbv0Period0Period37Qos {
        Dbv0Period0Period37Qos {
            description: None,
            flags: None,
            id: None,
            limits: None,
            preempt: None,
            priority: None,
            usage_factor: None,
            usage_threshold: None,
        }
    }
}


