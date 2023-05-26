/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.37
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V0Period0Period37NodeAllocation {
    /// amount of assigned job memory
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    /// amount of assigned job CPUs
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<serde_json::Value>,
    /// assignment status of each socket by socket id
    #[serde(rename = "sockets", skip_serializing_if = "Option::is_none")]
    pub sockets: Option<serde_json::Value>,
    /// assignment status of each core by core id
    #[serde(rename = "cores", skip_serializing_if = "Option::is_none")]
    pub cores: Option<serde_json::Value>,
}

impl V0Period0Period37NodeAllocation {
    pub fn new() -> V0Period0Period37NodeAllocation {
        V0Period0Period37NodeAllocation {
            memory: None,
            cpus: None,
            sockets: None,
            cores: None,
        }
    }
}


