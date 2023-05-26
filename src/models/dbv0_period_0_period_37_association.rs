/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.37
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0Period0Period37Association : Association description



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0Period0Period37Association {
    /// is default association
    #[serde(rename = "is_default", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<i32>,
    /// Assigned account
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Assigned cluster
    #[serde(rename = "cluster", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<Box<crate::models::Dbv0037AssociationDefault>>,
    /// List of properties of association
    #[serde(rename = "flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<Box<crate::models::Dbv0037AssociationMax>>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<Box<crate::models::Dbv0037AssociationMin>>,
    /// Parent account name
    #[serde(rename = "parent_account", skip_serializing_if = "Option::is_none")]
    pub parent_account: Option<String>,
    /// Assigned partition
    #[serde(rename = "partition", skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    /// Assigned priority
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Assigned QOS
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Vec<String>>,
    /// Raw fairshare shares
    #[serde(rename = "shares_raw", skip_serializing_if = "Option::is_none")]
    pub shares_raw: Option<i32>,
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<crate::models::Dbv0037AssociationUsage>>,
    /// Assigned user
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Dbv0Period0Period37Association {
    /// Association description
    pub fn new() -> Dbv0Period0Period37Association {
        Dbv0Period0Period37Association {
            is_default: None,
            account: None,
            cluster: None,
            default: None,
            flags: None,
            max: None,
            min: None,
            parent_account: None,
            partition: None,
            priority: None,
            qos: None,
            shares_raw: None,
            usage: None,
            user: None,
        }
    }
}


