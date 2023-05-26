# V0Period0Period37Node

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**architecture** | Option<**String**> | computer architecture | [optional]
**burstbuffer_network_address** | Option<**String**> | BcastAddr | [optional]
**boards** | Option<**i32**> | total number of boards per node | [optional]
**boot_time** | Option<**i64**> | timestamp of node boot | [optional]
**cores** | Option<**i32**> | number of cores per socket | [optional]
**cpu_binding** | Option<**i32**> | Default task binding | [optional]
**cpu_load** | Option<**i64**> | CPU load * 100 | [optional]
**free_memory** | Option<**i32**> | free memory in MiB | [optional]
**cpus** | Option<**i32**> | configured count of cpus running on the node | [optional]
**features** | Option<**String**> |  | [optional]
**active_features** | Option<**String**> | list of a node's available features | [optional]
**gres** | Option<**String**> | list of a node's generic resources | [optional]
**gres_drained** | Option<**String**> | list of drained GRES | [optional]
**gres_used** | Option<**String**> | list of GRES in current use | [optional]
**mcs_label** | Option<**String**> | mcs label if mcs plugin in use | [optional]
**name** | Option<**String**> | node name to slurm | [optional]
**next_state_after_reboot** | Option<**String**> | state after reboot | [optional]
**next_state_after_reboot_flags** | Option<**Vec<String>**> | node state flags | [optional]
**address** | Option<**String**> | state after reboot | [optional]
**hostname** | Option<**String**> | node's hostname | [optional]
**state** | Option<**String**> | current node state | [optional]
**state_flags** | Option<**Vec<String>**> | node state flags | [optional]
**operating_system** | Option<**String**> | operating system | [optional]
**owner** | Option<**String**> | User allowed to use this node | [optional]
**partitions** | Option<**Vec<String>**> | assigned partitions | [optional]
**port** | Option<**i32**> | TCP port number of the slurmd | [optional]
**real_memory** | Option<**i32**> | configured MB of real memory on the node | [optional]
**reason** | Option<**String**> | reason for node being DOWN or DRAINING | [optional]
**reason_changed_at** | Option<**i32**> | Time stamp when reason was set | [optional]
**reason_set_by_user** | Option<**String**> | User that set the reason | [optional]
**slurmd_start_time** | Option<**i64**> | timestamp of slurmd startup | [optional]
**sockets** | Option<**i32**> | total number of sockets per node | [optional]
**threads** | Option<**i32**> | number of threads per core | [optional]
**temporary_disk** | Option<**i32**> | configured MB of total disk in TMP_FS | [optional]
**weight** | Option<**i32**> | arbitrary priority of node for scheduling | [optional]
**tres** | Option<**String**> | TRES on node | [optional]
**tres_used** | Option<**String**> | TRES used on node | [optional]
**tres_weighted** | Option<**f64**> | TRES weight used on node | [optional]
**slurmd_version** | Option<**String**> | Slurmd version | [optional]
**alloc_cpus** | Option<**i64**> | Allocated CPUs | [optional]
**idle_cpus** | Option<**i64**> | Idle CPUs | [optional]
**alloc_memory** | Option<**i64**> | Allocated memory (MB) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


