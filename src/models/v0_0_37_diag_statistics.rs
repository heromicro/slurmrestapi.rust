/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.37
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// V0037DiagStatistics : Slurm statistics



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V0037DiagStatistics {
    /// partition records packed
    #[serde(rename = "parts_packed", skip_serializing_if = "Option::is_none")]
    pub parts_packed: Option<i32>,
    /// generation time
    #[serde(rename = "req_time", skip_serializing_if = "Option::is_none")]
    pub req_time: Option<i32>,
    /// data since
    #[serde(rename = "req_time_start", skip_serializing_if = "Option::is_none")]
    pub req_time_start: Option<i32>,
    /// Server thread count
    #[serde(rename = "server_thread_count", skip_serializing_if = "Option::is_none")]
    pub server_thread_count: Option<i32>,
    /// Agent queue size
    #[serde(rename = "agent_queue_size", skip_serializing_if = "Option::is_none")]
    pub agent_queue_size: Option<i32>,
    /// Agent count
    #[serde(rename = "agent_count", skip_serializing_if = "Option::is_none")]
    pub agent_count: Option<i32>,
    /// Agent thread count
    #[serde(rename = "agent_thread_count", skip_serializing_if = "Option::is_none")]
    pub agent_thread_count: Option<i32>,
    /// DBD Agent queue size
    #[serde(rename = "dbd_agent_queue_size", skip_serializing_if = "Option::is_none")]
    pub dbd_agent_queue_size: Option<i32>,
    /// Latency for 1000 calls to gettimeofday()
    #[serde(rename = "gettimeofday_latency", skip_serializing_if = "Option::is_none")]
    pub gettimeofday_latency: Option<i32>,
    /// Main Schedule max cycle
    #[serde(rename = "schedule_cycle_max", skip_serializing_if = "Option::is_none")]
    pub schedule_cycle_max: Option<i32>,
    /// Main Schedule last cycle
    #[serde(rename = "schedule_cycle_last", skip_serializing_if = "Option::is_none")]
    pub schedule_cycle_last: Option<i32>,
    /// Main Schedule cycle iterations
    #[serde(rename = "schedule_cycle_total", skip_serializing_if = "Option::is_none")]
    pub schedule_cycle_total: Option<i32>,
    /// Average time for Schedule Max cycle
    #[serde(rename = "schedule_cycle_mean", skip_serializing_if = "Option::is_none")]
    pub schedule_cycle_mean: Option<i32>,
    /// Average depth for Schedule Max cycle
    #[serde(rename = "schedule_cycle_mean_depth", skip_serializing_if = "Option::is_none")]
    pub schedule_cycle_mean_depth: Option<i32>,
    /// Main Schedule Cycles per minute
    #[serde(rename = "schedule_cycle_per_minute", skip_serializing_if = "Option::is_none")]
    pub schedule_cycle_per_minute: Option<i32>,
    /// Main Schedule Last queue length
    #[serde(rename = "schedule_queue_length", skip_serializing_if = "Option::is_none")]
    pub schedule_queue_length: Option<i32>,
    /// Job submitted
    #[serde(rename = "jobs_submitted", skip_serializing_if = "Option::is_none")]
    pub jobs_submitted: Option<i32>,
    /// Job started
    #[serde(rename = "jobs_started", skip_serializing_if = "Option::is_none")]
    pub jobs_started: Option<i32>,
    /// Job completed
    #[serde(rename = "jobs_completed", skip_serializing_if = "Option::is_none")]
    pub jobs_completed: Option<i32>,
    /// Job cancelled
    #[serde(rename = "jobs_canceled", skip_serializing_if = "Option::is_none")]
    pub jobs_canceled: Option<i32>,
    /// Job failed
    #[serde(rename = "jobs_failed", skip_serializing_if = "Option::is_none")]
    pub jobs_failed: Option<i32>,
    /// Job pending
    #[serde(rename = "jobs_pending", skip_serializing_if = "Option::is_none")]
    pub jobs_pending: Option<i32>,
    /// Job running
    #[serde(rename = "jobs_running", skip_serializing_if = "Option::is_none")]
    pub jobs_running: Option<i32>,
    /// Job states timestamp
    #[serde(rename = "job_states_ts", skip_serializing_if = "Option::is_none")]
    pub job_states_ts: Option<i32>,
    /// Total backfilled jobs (since last slurm start)
    #[serde(rename = "bf_backfilled_jobs", skip_serializing_if = "Option::is_none")]
    pub bf_backfilled_jobs: Option<i32>,
    /// Total backfilled jobs (since last stats cycle start)
    #[serde(rename = "bf_last_backfilled_jobs", skip_serializing_if = "Option::is_none")]
    pub bf_last_backfilled_jobs: Option<i32>,
    /// Total backfilled heterogeneous job components
    #[serde(rename = "bf_backfilled_het_jobs", skip_serializing_if = "Option::is_none")]
    pub bf_backfilled_het_jobs: Option<i32>,
    /// Backfill Schedule Total cycles
    #[serde(rename = "bf_cycle_counter", skip_serializing_if = "Option::is_none")]
    pub bf_cycle_counter: Option<i32>,
    /// Backfill Schedule Mean cycle
    #[serde(rename = "bf_cycle_mean", skip_serializing_if = "Option::is_none")]
    pub bf_cycle_mean: Option<i32>,
    /// Backfill Schedule Max cycle time
    #[serde(rename = "bf_cycle_max", skip_serializing_if = "Option::is_none")]
    pub bf_cycle_max: Option<i32>,
    /// Backfill Schedule Last depth cycle
    #[serde(rename = "bf_last_depth", skip_serializing_if = "Option::is_none")]
    pub bf_last_depth: Option<i32>,
    /// Backfill Schedule Mean cycle (try sched)
    #[serde(rename = "bf_last_depth_try", skip_serializing_if = "Option::is_none")]
    pub bf_last_depth_try: Option<i32>,
    /// Backfill Schedule Depth Mean
    #[serde(rename = "bf_depth_mean", skip_serializing_if = "Option::is_none")]
    pub bf_depth_mean: Option<i32>,
    /// Backfill Schedule Depth Mean (try sched)
    #[serde(rename = "bf_depth_mean_try", skip_serializing_if = "Option::is_none")]
    pub bf_depth_mean_try: Option<i32>,
    /// Backfill Schedule Last cycle time
    #[serde(rename = "bf_cycle_last", skip_serializing_if = "Option::is_none")]
    pub bf_cycle_last: Option<i32>,
    /// Backfill Schedule Last queue length
    #[serde(rename = "bf_queue_len", skip_serializing_if = "Option::is_none")]
    pub bf_queue_len: Option<i32>,
    /// Backfill Schedule Mean queue length
    #[serde(rename = "bf_queue_len_mean", skip_serializing_if = "Option::is_none")]
    pub bf_queue_len_mean: Option<i32>,
    /// Last cycle timestamp
    #[serde(rename = "bf_when_last_cycle", skip_serializing_if = "Option::is_none")]
    pub bf_when_last_cycle: Option<i32>,
    /// Backfill Schedule currently active
    #[serde(rename = "bf_active", skip_serializing_if = "Option::is_none")]
    pub bf_active: Option<bool>,
}

impl V0037DiagStatistics {
    /// Slurm statistics
    pub fn new() -> V0037DiagStatistics {
        V0037DiagStatistics {
            parts_packed: None,
            req_time: None,
            req_time_start: None,
            server_thread_count: None,
            agent_queue_size: None,
            agent_count: None,
            agent_thread_count: None,
            dbd_agent_queue_size: None,
            gettimeofday_latency: None,
            schedule_cycle_max: None,
            schedule_cycle_last: None,
            schedule_cycle_total: None,
            schedule_cycle_mean: None,
            schedule_cycle_mean_depth: None,
            schedule_cycle_per_minute: None,
            schedule_queue_length: None,
            jobs_submitted: None,
            jobs_started: None,
            jobs_completed: None,
            jobs_canceled: None,
            jobs_failed: None,
            jobs_pending: None,
            jobs_running: None,
            job_states_ts: None,
            bf_backfilled_jobs: None,
            bf_last_backfilled_jobs: None,
            bf_backfilled_het_jobs: None,
            bf_cycle_counter: None,
            bf_cycle_mean: None,
            bf_cycle_max: None,
            bf_last_depth: None,
            bf_last_depth_try: None,
            bf_depth_mean: None,
            bf_depth_mean_try: None,
            bf_cycle_last: None,
            bf_queue_len: None,
            bf_queue_len_mean: None,
            bf_when_last_cycle: None,
            bf_active: None,
        }
    }
}


