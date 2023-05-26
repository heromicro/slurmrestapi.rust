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
pub struct V0Period0Period37Node {
    /// computer architecture
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// BcastAddr
    #[serde(rename = "burstbuffer_network_address", skip_serializing_if = "Option::is_none")]
    pub burstbuffer_network_address: Option<String>,
    /// total number of boards per node
    #[serde(rename = "boards", skip_serializing_if = "Option::is_none")]
    pub boards: Option<i32>,
    /// timestamp of node boot
    #[serde(rename = "boot_time", skip_serializing_if = "Option::is_none")]
    pub boot_time: Option<i64>,
    /// number of cores per socket
    #[serde(rename = "cores", skip_serializing_if = "Option::is_none")]
    pub cores: Option<i32>,
    /// Default task binding
    #[serde(rename = "cpu_binding", skip_serializing_if = "Option::is_none")]
    pub cpu_binding: Option<i32>,
    /// CPU load * 100
    #[serde(rename = "cpu_load", skip_serializing_if = "Option::is_none")]
    pub cpu_load: Option<i64>,
    /// free memory in MiB
    #[serde(rename = "free_memory", skip_serializing_if = "Option::is_none")]
    pub free_memory: Option<i32>,
    /// configured count of cpus running on the node
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<i32>,
    /// 
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    /// list of a node's available features
    #[serde(rename = "active_features", skip_serializing_if = "Option::is_none")]
    pub active_features: Option<String>,
    /// list of a node's generic resources
    #[serde(rename = "gres", skip_serializing_if = "Option::is_none")]
    pub gres: Option<String>,
    /// list of drained GRES
    #[serde(rename = "gres_drained", skip_serializing_if = "Option::is_none")]
    pub gres_drained: Option<String>,
    /// list of GRES in current use
    #[serde(rename = "gres_used", skip_serializing_if = "Option::is_none")]
    pub gres_used: Option<String>,
    /// mcs label if mcs plugin in use
    #[serde(rename = "mcs_label", skip_serializing_if = "Option::is_none")]
    pub mcs_label: Option<String>,
    /// node name to slurm
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// state after reboot
    #[serde(rename = "next_state_after_reboot", skip_serializing_if = "Option::is_none")]
    pub next_state_after_reboot: Option<String>,
    /// node state flags
    #[serde(rename = "next_state_after_reboot_flags", skip_serializing_if = "Option::is_none")]
    pub next_state_after_reboot_flags: Option<Vec<String>>,
    /// state after reboot
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// node's hostname
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// current node state
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// node state flags
    #[serde(rename = "state_flags", skip_serializing_if = "Option::is_none")]
    pub state_flags: Option<Vec<String>>,
    /// operating system
    #[serde(rename = "operating_system", skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// User allowed to use this node
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// assigned partitions
    #[serde(rename = "partitions", skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<String>>,
    /// TCP port number of the slurmd
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// configured MB of real memory on the node
    #[serde(rename = "real_memory", skip_serializing_if = "Option::is_none")]
    pub real_memory: Option<i32>,
    /// reason for node being DOWN or DRAINING
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Time stamp when reason was set
    #[serde(rename = "reason_changed_at", skip_serializing_if = "Option::is_none")]
    pub reason_changed_at: Option<i32>,
    /// User that set the reason
    #[serde(rename = "reason_set_by_user", skip_serializing_if = "Option::is_none")]
    pub reason_set_by_user: Option<String>,
    /// timestamp of slurmd startup
    #[serde(rename = "slurmd_start_time", skip_serializing_if = "Option::is_none")]
    pub slurmd_start_time: Option<i64>,
    /// total number of sockets per node
    #[serde(rename = "sockets", skip_serializing_if = "Option::is_none")]
    pub sockets: Option<i32>,
    /// number of threads per core
    #[serde(rename = "threads", skip_serializing_if = "Option::is_none")]
    pub threads: Option<i32>,
    /// configured MB of total disk in TMP_FS
    #[serde(rename = "temporary_disk", skip_serializing_if = "Option::is_none")]
    pub temporary_disk: Option<i32>,
    /// arbitrary priority of node for scheduling
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// TRES on node
    #[serde(rename = "tres", skip_serializing_if = "Option::is_none")]
    pub tres: Option<String>,
    /// TRES used on node
    #[serde(rename = "tres_used", skip_serializing_if = "Option::is_none")]
    pub tres_used: Option<String>,
    /// TRES weight used on node
    #[serde(rename = "tres_weighted", skip_serializing_if = "Option::is_none")]
    pub tres_weighted: Option<f64>,
    /// Slurmd version
    #[serde(rename = "slurmd_version", skip_serializing_if = "Option::is_none")]
    pub slurmd_version: Option<String>,
    /// Allocated CPUs
    #[serde(rename = "alloc_cpus", skip_serializing_if = "Option::is_none")]
    pub alloc_cpus: Option<i64>,
    /// Idle CPUs
    #[serde(rename = "idle_cpus", skip_serializing_if = "Option::is_none")]
    pub idle_cpus: Option<i64>,
    /// Allocated memory (MB)
    #[serde(rename = "alloc_memory", skip_serializing_if = "Option::is_none")]
    pub alloc_memory: Option<i64>,
}

impl V0Period0Period37Node {
    pub fn new() -> V0Period0Period37Node {
        V0Period0Period37Node {
            architecture: None,
            burstbuffer_network_address: None,
            boards: None,
            boot_time: None,
            cores: None,
            cpu_binding: None,
            cpu_load: None,
            free_memory: None,
            cpus: None,
            features: None,
            active_features: None,
            gres: None,
            gres_drained: None,
            gres_used: None,
            mcs_label: None,
            name: None,
            next_state_after_reboot: None,
            next_state_after_reboot_flags: None,
            address: None,
            hostname: None,
            state: None,
            state_flags: None,
            operating_system: None,
            owner: None,
            partitions: None,
            port: None,
            real_memory: None,
            reason: None,
            reason_changed_at: None,
            reason_set_by_user: None,
            slurmd_start_time: None,
            sockets: None,
            threads: None,
            temporary_disk: None,
            weight: None,
            tres: None,
            tres_used: None,
            tres_weighted: None,
            slurmd_version: None,
            alloc_cpus: None,
            idle_cpus: None,
            alloc_memory: None,
        }
    }
}


