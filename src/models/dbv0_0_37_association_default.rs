/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.37
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0037AssociationDefault : Default settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0037AssociationDefault {
    /// Default QOS
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<String>,
}

impl Dbv0037AssociationDefault {
    /// Default settings
    pub fn new() -> Dbv0037AssociationDefault {
        Dbv0037AssociationDefault {
            qos: None,
        }
    }
}


