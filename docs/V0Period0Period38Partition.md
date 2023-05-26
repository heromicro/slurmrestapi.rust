# V0Period0Period38Partition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flags** | Option<**Vec<String>**> | partition options | [optional]
**preemption_mode** | Option<**Vec<String>**> | preemption type | [optional]
**allowed_allocation_nodes** | Option<**String**> | list names of allowed allocating nodes | [optional]
**allowed_accounts** | Option<**String**> | comma delimited list of accounts | [optional]
**allowed_groups** | Option<**String**> | comma delimited list of groups | [optional]
**allowed_qos** | Option<**String**> | comma delimited list of qos | [optional]
**alternative** | Option<**String**> | name of alternate partition | [optional]
**billing_weights** | Option<**String**> | TRES billing weights | [optional]
**default_memory_per_cpu** | Option<**i64**> | default MB memory per allocated CPU | [optional]
**default_time_limit** | Option<**i64**> | default time limit (minutes) | [optional]
**denied_accounts** | Option<**String**> | comma delimited list of denied accounts | [optional]
**denied_qos** | Option<**String**> | comma delimited list of denied qos | [optional]
**preemption_grace_time** | Option<**i64**> | preemption grace time (seconds) | [optional]
**maximum_cpus_per_node** | Option<**i32**> | maximum allocated CPUs per node | [optional]
**maximum_memory_per_node** | Option<**i64**> | maximum memory per allocated node (MiB) | [optional]
**maximum_nodes_per_job** | Option<**i32**> | Max nodes per job | [optional]
**max_time_limit** | Option<**i64**> | Max time limit per job | [optional]
**min_nodes_per_job** | Option<**i32**> | Min number of nodes per job | [optional]
**name** | Option<**String**> | Partition name | [optional]
**nodes** | Option<**String**> | list names of nodes in partition | [optional]
**over_time_limit** | Option<**i32**> | job's time limit can be exceeded by this number of minutes before cancellation | [optional]
**priority_job_factor** | Option<**i32**> | job priority weight factor | [optional]
**priority_tier** | Option<**i32**> | tier for scheduling and preemption | [optional]
**qos** | Option<**String**> | partition QOS name | [optional]
**state** | Option<**String**> | Partition state | [optional]
**total_cpus** | Option<**i32**> | Total cpus in partition | [optional]
**total_nodes** | Option<**i32**> | Total number of nodes in partition | [optional]
**tres** | Option<**String**> | configured TRES in partition | [optional]
**maximum_memory_per_cpu** | Option<**i64**> | maximum memory per allocated CPU (MiB) | [optional]
**default_memory_per_node** | Option<**i64**> | default MB memory per allocated node | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


