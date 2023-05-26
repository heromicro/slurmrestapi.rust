/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.38
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0Period0Period38ClusterInfo {
    #[serde(rename = "controller", skip_serializing_if = "Option::is_none")]
    pub controller: Option<Box<crate::models::Dbv0038ClusterInfoController>>,
    /// List of properties of cluster
    #[serde(rename = "flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    /// Cluster name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Assigned nodes
    #[serde(rename = "nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<String>,
    /// Configured select plugin
    #[serde(rename = "select_plugin", skip_serializing_if = "Option::is_none")]
    pub select_plugin: Option<String>,
    #[serde(rename = "associations", skip_serializing_if = "Option::is_none")]
    pub associations: Option<Box<crate::models::Dbv0038ClusterInfoAssociations>>,
    /// Number rpc version
    #[serde(rename = "rpc_version", skip_serializing_if = "Option::is_none")]
    pub rpc_version: Option<i32>,
    /// List of TRES in cluster
    #[serde(rename = "tres", skip_serializing_if = "Option::is_none")]
    pub tres: Option<Vec<crate::models::Dbv0Period0Period38ResponseTres>>,
}

impl Dbv0Period0Period38ClusterInfo {
    pub fn new() -> Dbv0Period0Period38ClusterInfo {
        Dbv0Period0Period38ClusterInfo {
            controller: None,
            flags: None,
            name: None,
            nodes: None,
            select_plugin: None,
            associations: None,
            rpc_version: None,
            tres: None,
        }
    }
}


