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
pub struct V0Period0Period37JobProperties {
    /// Charge resources used by this job to specified account.
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Define the job accounting and profiling sampling intervals.
    #[serde(rename = "account_gather_frequency", skip_serializing_if = "Option::is_none")]
    pub account_gather_frequency: Option<String>,
    /// Arguments to the script.
    #[serde(rename = "argv", skip_serializing_if = "Option::is_none")]
    pub argv: Option<Vec<String>>,
    /// Submit a job array, multiple jobs to be executed with identical parameters. The indexes specification identifies what array index values should be used.
    #[serde(rename = "array", skip_serializing_if = "Option::is_none")]
    pub array: Option<String>,
    /// features required for batch script's node
    #[serde(rename = "batch_features", skip_serializing_if = "Option::is_none")]
    pub batch_features: Option<String>,
    /// Submit the batch script to the Slurm controller immediately, like normal, but tell the controller to defer the allocation of the job until the specified time.
    #[serde(rename = "begin_time", skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<i64>,
    /// Burst buffer specification.
    #[serde(rename = "burst_buffer", skip_serializing_if = "Option::is_none")]
    pub burst_buffer: Option<String>,
    /// Specifies features that a federated cluster must have to have a sibling job submitted to it.
    #[serde(rename = "cluster_constraint", skip_serializing_if = "Option::is_none")]
    pub cluster_constraint: Option<String>,
    /// An arbitrary comment.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// node features required by job.
    #[serde(rename = "constraints", skip_serializing_if = "Option::is_none")]
    pub constraints: Option<String>,
    /// Count of specialized threads per node reserved by the job for system operations and not used by the application.
    #[serde(rename = "core_specification", skip_serializing_if = "Option::is_none")]
    pub core_specification: Option<i32>,
    /// Restrict node selection to nodes with at least the specified number of cores per socket.
    #[serde(rename = "cores_per_socket", skip_serializing_if = "Option::is_none")]
    pub cores_per_socket: Option<i32>,
    /// Cpu binding
    #[serde(rename = "cpu_binding", skip_serializing_if = "Option::is_none")]
    pub cpu_binding: Option<String>,
    /// Cpu binding hint
    #[serde(rename = "cpu_binding_hint", skip_serializing_if = "Option::is_none")]
    pub cpu_binding_hint: Option<String>,
    /// Request that job steps initiated by srun commands inside this sbatch script be run at some requested frequency if possible, on the CPUs selected for the step on the compute node(s).
    #[serde(rename = "cpu_frequency", skip_serializing_if = "Option::is_none")]
    pub cpu_frequency: Option<String>,
    /// Number of CPUs requested per allocated GPU.
    #[serde(rename = "cpus_per_gpu", skip_serializing_if = "Option::is_none")]
    pub cpus_per_gpu: Option<String>,
    /// Advise the Slurm controller that ensuing job steps will require ncpus number of processors per task.
    #[serde(rename = "cpus_per_task", skip_serializing_if = "Option::is_none")]
    pub cpus_per_task: Option<i32>,
    /// Instruct Slurm to connect the batch script's standard output directly to the file name.
    #[serde(rename = "current_working_directory", skip_serializing_if = "Option::is_none")]
    pub current_working_directory: Option<String>,
    /// Remove the job if no ending is possible before this deadline (start > (deadline - time[-min])).
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    /// Do not reboot nodes in order to satisfied this job's feature specification if the job has been eligible to run for less than this time period.
    #[serde(rename = "delay_boot", skip_serializing_if = "Option::is_none")]
    pub delay_boot: Option<i32>,
    /// Defer the start of this job until the specified dependencies have been satisfied completed.
    #[serde(rename = "dependency", skip_serializing_if = "Option::is_none")]
    pub dependency: Option<String>,
    /// Specify alternate distribution methods for remote processes.
    #[serde(rename = "distribution", skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    /// Dictionary of environment entries.
    #[serde(rename = "environment")]
    pub environment: serde_json::Value,
    /// The job allocation can share nodes just other users with the \"user\" option or with the \"mcs\" option).
    #[serde(rename = "exclusive", skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<Exclusive>,
    /// Load new login environment for user on job node.
    #[serde(rename = "get_user_environment", skip_serializing_if = "Option::is_none")]
    pub get_user_environment: Option<bool>,
    /// Specifies a comma delimited list of generic consumable resources.
    #[serde(rename = "gres", skip_serializing_if = "Option::is_none")]
    pub gres: Option<String>,
    /// Specify generic resource task binding options.
    #[serde(rename = "gres_flags", skip_serializing_if = "Option::is_none")]
    pub gres_flags: Option<GresFlags>,
    /// Requested binding of tasks to GPU.
    #[serde(rename = "gpu_binding", skip_serializing_if = "Option::is_none")]
    pub gpu_binding: Option<String>,
    /// Requested GPU frequency.
    #[serde(rename = "gpu_frequency", skip_serializing_if = "Option::is_none")]
    pub gpu_frequency: Option<String>,
    /// GPUs per job.
    #[serde(rename = "gpus", skip_serializing_if = "Option::is_none")]
    pub gpus: Option<String>,
    /// GPUs per node.
    #[serde(rename = "gpus_per_node", skip_serializing_if = "Option::is_none")]
    pub gpus_per_node: Option<String>,
    /// GPUs per socket.
    #[serde(rename = "gpus_per_socket", skip_serializing_if = "Option::is_none")]
    pub gpus_per_socket: Option<String>,
    /// GPUs per task.
    #[serde(rename = "gpus_per_task", skip_serializing_if = "Option::is_none")]
    pub gpus_per_task: Option<String>,
    /// Specify the job is to be submitted in a held state (priority of zero).
    #[serde(rename = "hold", skip_serializing_if = "Option::is_none")]
    pub hold: Option<bool>,
    /// If a job has an invalid dependency, then Slurm is to terminate it.
    #[serde(rename = "kill_on_invalid_dependency", skip_serializing_if = "Option::is_none")]
    pub kill_on_invalid_dependency: Option<bool>,
    /// Specification of licenses (or other resources available on all nodes of the cluster) which must be allocated to this job.
    #[serde(rename = "licenses", skip_serializing_if = "Option::is_none")]
    pub licenses: Option<String>,
    /// Notify user by email when certain event types occur.
    #[serde(rename = "mail_type", skip_serializing_if = "Option::is_none")]
    pub mail_type: Option<String>,
    /// User to receive email notification of state changes as defined by mail_type.
    #[serde(rename = "mail_user", skip_serializing_if = "Option::is_none")]
    pub mail_user: Option<String>,
    /// This parameter is a group among the groups of the user.
    #[serde(rename = "mcs_label", skip_serializing_if = "Option::is_none")]
    pub mcs_label: Option<String>,
    /// Bind tasks to memory.
    #[serde(rename = "memory_binding", skip_serializing_if = "Option::is_none")]
    pub memory_binding: Option<String>,
    /// Minimum real memory per cpu (MB).
    #[serde(rename = "memory_per_cpu", skip_serializing_if = "Option::is_none")]
    pub memory_per_cpu: Option<i32>,
    /// Minimum memory required per allocated GPU.
    #[serde(rename = "memory_per_gpu", skip_serializing_if = "Option::is_none")]
    pub memory_per_gpu: Option<i32>,
    /// Minimum real memory per node (MB).
    #[serde(rename = "memory_per_node", skip_serializing_if = "Option::is_none")]
    pub memory_per_node: Option<i32>,
    /// Minimum number of CPUs per node.
    #[serde(rename = "minimum_cpus_per_node", skip_serializing_if = "Option::is_none")]
    pub minimum_cpus_per_node: Option<i32>,
    /// If a range of node counts is given, prefer the smaller count.
    #[serde(rename = "minimum_nodes", skip_serializing_if = "Option::is_none")]
    pub minimum_nodes: Option<bool>,
    /// Specify a name for the job allocation.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Run the job with an adjusted scheduling priority within Slurm.
    #[serde(rename = "nice", skip_serializing_if = "Option::is_none")]
    pub nice: Option<String>,
    /// Do not automatically terminate a job if one of the nodes it has been allocated fails.
    #[serde(rename = "no_kill", skip_serializing_if = "Option::is_none")]
    pub no_kill: Option<bool>,
    /// Request that a minimum of minnodes nodes and a maximum node count.
    #[serde(rename = "nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<i32>>,
    /// Open the output and error files using append or truncate mode as specified.
    #[serde(rename = "open_mode", skip_serializing_if = "Option::is_none")]
    pub open_mode: Option<OpenMode>,
    /// Request a specific partition for the resource allocation.
    #[serde(rename = "partition", skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    /// Request a specific job priority.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// Request a quality of service for the job.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<String>,
    /// Specifies that the batch job should eligible to being requeue.
    #[serde(rename = "requeue", skip_serializing_if = "Option::is_none")]
    pub requeue: Option<bool>,
    /// Allocate resources for the job from the named reservation.
    #[serde(rename = "reservation", skip_serializing_if = "Option::is_none")]
    pub reservation: Option<String>,
    /// When a job is within sig_time seconds of its end time, send it the signal sig_num.
    #[serde(rename = "signal", skip_serializing_if = "Option::is_none")]
    pub signal: Option<String>,
    /// Restrict node selection to nodes with at least the specified number of sockets.
    #[serde(rename = "sockets_per_node", skip_serializing_if = "Option::is_none")]
    pub sockets_per_node: Option<i32>,
    /// Spread the job allocation over as many nodes as possible and attempt to evenly distribute tasks across the allocated nodes.
    #[serde(rename = "spread_job", skip_serializing_if = "Option::is_none")]
    pub spread_job: Option<bool>,
    /// Instruct Slurm to connect the batch script's standard error directly to the file name.
    #[serde(rename = "standard_error", skip_serializing_if = "Option::is_none")]
    pub standard_error: Option<String>,
    /// Instruct Slurm to connect the batch script's standard input directly to the file name specified.
    #[serde(rename = "standard_input", skip_serializing_if = "Option::is_none")]
    pub standard_input: Option<String>,
    /// Instruct Slurm to connect the batch script's standard output directly to the file name.
    #[serde(rename = "standard_output", skip_serializing_if = "Option::is_none")]
    pub standard_output: Option<String>,
    /// Advises the Slurm controller that job steps run within the allocation will launch a maximum of number tasks and to provide for sufficient resources.
    #[serde(rename = "tasks", skip_serializing_if = "Option::is_none")]
    pub tasks: Option<i32>,
    /// Request the maximum ntasks be invoked on each core.
    #[serde(rename = "tasks_per_core", skip_serializing_if = "Option::is_none")]
    pub tasks_per_core: Option<i32>,
    /// Request the maximum ntasks be invoked on each node.
    #[serde(rename = "tasks_per_node", skip_serializing_if = "Option::is_none")]
    pub tasks_per_node: Option<i32>,
    /// Request the maximum ntasks be invoked on each socket.
    #[serde(rename = "tasks_per_socket", skip_serializing_if = "Option::is_none")]
    pub tasks_per_socket: Option<i32>,
    /// Count of specialized threads per node reserved by the job for system operations and not used by the application.
    #[serde(rename = "thread_specification", skip_serializing_if = "Option::is_none")]
    pub thread_specification: Option<i32>,
    /// Restrict node selection to nodes with at least the specified number of threads per core.
    #[serde(rename = "threads_per_core", skip_serializing_if = "Option::is_none")]
    pub threads_per_core: Option<i32>,
    /// Step time limit.
    #[serde(rename = "time_limit", skip_serializing_if = "Option::is_none")]
    pub time_limit: Option<i32>,
    /// Minimum run time in minutes.
    #[serde(rename = "time_minimum", skip_serializing_if = "Option::is_none")]
    pub time_minimum: Option<i32>,
    /// Do not begin execution until all nodes are ready for use.
    #[serde(rename = "wait_all_nodes", skip_serializing_if = "Option::is_none")]
    pub wait_all_nodes: Option<bool>,
    /// Specify wckey to be used with job.
    #[serde(rename = "wckey", skip_serializing_if = "Option::is_none")]
    pub wckey: Option<String>,
}

impl V0Period0Period37JobProperties {
    pub fn new(environment: serde_json::Value) -> V0Period0Period37JobProperties {
        V0Period0Period37JobProperties {
            account: None,
            account_gather_frequency: None,
            argv: None,
            array: None,
            batch_features: None,
            begin_time: None,
            burst_buffer: None,
            cluster_constraint: None,
            comment: None,
            constraints: None,
            core_specification: None,
            cores_per_socket: None,
            cpu_binding: None,
            cpu_binding_hint: None,
            cpu_frequency: None,
            cpus_per_gpu: None,
            cpus_per_task: None,
            current_working_directory: None,
            deadline: None,
            delay_boot: None,
            dependency: None,
            distribution: None,
            environment,
            exclusive: None,
            get_user_environment: None,
            gres: None,
            gres_flags: None,
            gpu_binding: None,
            gpu_frequency: None,
            gpus: None,
            gpus_per_node: None,
            gpus_per_socket: None,
            gpus_per_task: None,
            hold: None,
            kill_on_invalid_dependency: None,
            licenses: None,
            mail_type: None,
            mail_user: None,
            mcs_label: None,
            memory_binding: None,
            memory_per_cpu: None,
            memory_per_gpu: None,
            memory_per_node: None,
            minimum_cpus_per_node: None,
            minimum_nodes: None,
            name: None,
            nice: None,
            no_kill: None,
            nodes: None,
            open_mode: None,
            partition: None,
            priority: None,
            qos: None,
            requeue: None,
            reservation: None,
            signal: None,
            sockets_per_node: None,
            spread_job: None,
            standard_error: None,
            standard_input: None,
            standard_output: None,
            tasks: None,
            tasks_per_core: None,
            tasks_per_node: None,
            tasks_per_socket: None,
            thread_specification: None,
            threads_per_core: None,
            time_limit: None,
            time_minimum: None,
            wait_all_nodes: None,
            wckey: None,
        }
    }
}

/// The job allocation can share nodes just other users with the \"user\" option or with the \"mcs\" option).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Exclusive {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "mcs")]
    Mcs,
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for Exclusive {
    fn default() -> Exclusive {
        Self::User
    }
}
/// Specify generic resource task binding options.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GresFlags {
    #[serde(rename = "disable-binding")]
    DisableBinding,
    #[serde(rename = "enforce-binding")]
    EnforceBinding,
}

impl Default for GresFlags {
    fn default() -> GresFlags {
        Self::DisableBinding
    }
}
/// Open the output and error files using append or truncate mode as specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OpenMode {
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "truncate")]
    Truncate,
}

impl Default for OpenMode {
    fn default() -> OpenMode {
        Self::Append
    }
}

