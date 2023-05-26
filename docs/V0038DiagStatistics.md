# V0038DiagStatistics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parts_packed** | Option<**i32**> | partition records packed | [optional]
**req_time** | Option<**i32**> | generation time | [optional]
**req_time_start** | Option<**i32**> | data since | [optional]
**server_thread_count** | Option<**i32**> | Server thread count | [optional]
**agent_queue_size** | Option<**i32**> | Agent queue size | [optional]
**agent_count** | Option<**i32**> | Agent count | [optional]
**agent_thread_count** | Option<**i32**> | Agent thread count | [optional]
**dbd_agent_queue_size** | Option<**i32**> | DBD Agent queue size | [optional]
**gettimeofday_latency** | Option<**i32**> | Latency for 1000 calls to gettimeofday() | [optional]
**schedule_cycle_max** | Option<**i32**> | Main Schedule max cycle | [optional]
**schedule_cycle_last** | Option<**i32**> | Main Schedule last cycle | [optional]
**schedule_cycle_total** | Option<**i32**> | Main Schedule cycle iterations | [optional]
**schedule_cycle_mean** | Option<**i32**> | Average time for Schedule Max cycle | [optional]
**schedule_cycle_mean_depth** | Option<**i32**> | Average depth for Schedule Max cycle | [optional]
**schedule_cycle_per_minute** | Option<**i32**> | Main Schedule Cycles per minute | [optional]
**schedule_queue_length** | Option<**i32**> | Main Schedule Last queue length | [optional]
**jobs_submitted** | Option<**i32**> | Job submitted | [optional]
**jobs_started** | Option<**i32**> | Job started | [optional]
**jobs_completed** | Option<**i32**> | Job completed | [optional]
**jobs_canceled** | Option<**i32**> | Job cancelled | [optional]
**jobs_failed** | Option<**i32**> | Job failed | [optional]
**jobs_pending** | Option<**i32**> | Job pending | [optional]
**jobs_running** | Option<**i32**> | Job running | [optional]
**job_states_ts** | Option<**i32**> | Job states timestamp | [optional]
**bf_backfilled_jobs** | Option<**i32**> | Total backfilled jobs (since last slurm start) | [optional]
**bf_last_backfilled_jobs** | Option<**i32**> | Total backfilled jobs (since last stats cycle start) | [optional]
**bf_backfilled_het_jobs** | Option<**i32**> | Total backfilled heterogeneous job components | [optional]
**bf_cycle_counter** | Option<**i32**> | Backfill Schedule Total cycles | [optional]
**bf_cycle_mean** | Option<**i32**> | Backfill Schedule Mean cycle | [optional]
**bf_cycle_max** | Option<**i32**> | Backfill Schedule Max cycle time | [optional]
**bf_last_depth** | Option<**i32**> | Backfill Schedule Last depth cycle | [optional]
**bf_last_depth_try** | Option<**i32**> | Backfill Schedule Mean cycle (try sched) | [optional]
**bf_depth_mean** | Option<**i32**> | Backfill Schedule Depth Mean | [optional]
**bf_depth_mean_try** | Option<**i32**> | Backfill Schedule Depth Mean (try sched) | [optional]
**bf_cycle_last** | Option<**i32**> | Backfill Schedule Last cycle time | [optional]
**bf_queue_len** | Option<**i32**> | Backfill Schedule Last queue length | [optional]
**bf_queue_len_mean** | Option<**i32**> | Backfill Schedule Mean queue length | [optional]
**bf_table_size** | Option<**i32**> | Backfill Schedule Last table size | [optional]
**bf_table_size_mean** | Option<**i32**> | Backfill Schedule Mean table size | [optional]
**bf_when_last_cycle** | Option<**i32**> | Last cycle timestamp | [optional]
**bf_active** | Option<**bool**> | Backfill Schedule currently active | [optional]
**rpcs_by_message_type** | Option<[**Vec<crate::models::V0Period0Period38DiagRpcm>**](v0.0.38_diag_rpcm.md)> | Remote Procedure Call statistics by message type | [optional]
**rpcs_by_user** | Option<[**Vec<crate::models::V0Period0Period38DiagRpcu>**](v0.0.38_diag_rpcu.md)> | Remote Procedure Call statistics by user | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


