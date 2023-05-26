# \SlurmApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**slurm_v0039_cancel_job**](SlurmApi.md#slurm_v0039_cancel_job) | **DELETE** /slurm/v0.0.39/job/{job_id} | cancel or signal job
[**slurm_v0039_delete_node**](SlurmApi.md#slurm_v0039_delete_node) | **DELETE** /slurm/v0.0.39/node/{node_name} | delete node
[**slurm_v0039_diag**](SlurmApi.md#slurm_v0039_diag) | **GET** /slurm/v0.0.39/diag | get diagnostics
[**slurm_v0039_get_job**](SlurmApi.md#slurm_v0039_get_job) | **GET** /slurm/v0.0.39/job/{job_id} | get job info
[**slurm_v0039_get_jobs**](SlurmApi.md#slurm_v0039_get_jobs) | **GET** /slurm/v0.0.39/jobs | get list of jobs
[**slurm_v0039_get_node**](SlurmApi.md#slurm_v0039_get_node) | **GET** /slurm/v0.0.39/node/{node_name} | get node info
[**slurm_v0039_get_nodes**](SlurmApi.md#slurm_v0039_get_nodes) | **GET** /slurm/v0.0.39/nodes | get all node info
[**slurm_v0039_get_partition**](SlurmApi.md#slurm_v0039_get_partition) | **GET** /slurm/v0.0.39/partition/{partition_name} | get partition info
[**slurm_v0039_get_partitions**](SlurmApi.md#slurm_v0039_get_partitions) | **GET** /slurm/v0.0.39/partitions | get all partition info
[**slurm_v0039_get_reservation**](SlurmApi.md#slurm_v0039_get_reservation) | **GET** /slurm/v0.0.39/reservation/{reservation_name} | get reservation info
[**slurm_v0039_get_reservations**](SlurmApi.md#slurm_v0039_get_reservations) | **GET** /slurm/v0.0.39/reservations | get all reservation info
[**slurm_v0039_ping**](SlurmApi.md#slurm_v0039_ping) | **GET** /slurm/v0.0.39/ping | ping test
[**slurm_v0039_slurmctld_get_licenses**](SlurmApi.md#slurm_v0039_slurmctld_get_licenses) | **GET** /slurm/v0.0.39/licenses | get all Slurm tracked license info
[**slurm_v0039_submit_job**](SlurmApi.md#slurm_v0039_submit_job) | **POST** /slurm/v0.0.39/job/submit | submit new job
[**slurm_v0039_update_job**](SlurmApi.md#slurm_v0039_update_job) | **POST** /slurm/v0.0.39/job/{job_id} | update job
[**slurm_v0039_update_node**](SlurmApi.md#slurm_v0039_update_node) | **POST** /slurm/v0.0.39/node/{node_name} | update node properties
[**slurmdb_v0039_add_clusters**](SlurmApi.md#slurmdb_v0039_add_clusters) | **POST** /slurmdb/v0.0.39/clusters | Add clusters
[**slurmdb_v0039_add_wckeys**](SlurmApi.md#slurmdb_v0039_add_wckeys) | **POST** /slurmdb/v0.0.39/wckeys | Add wckeys
[**slurmdb_v0039_delete_account**](SlurmApi.md#slurmdb_v0039_delete_account) | **DELETE** /slurmdb/v0.0.39/account/{account_name} | Delete account
[**slurmdb_v0039_delete_association**](SlurmApi.md#slurmdb_v0039_delete_association) | **DELETE** /slurmdb/v0.0.39/association | Delete association
[**slurmdb_v0039_delete_associations**](SlurmApi.md#slurmdb_v0039_delete_associations) | **DELETE** /slurmdb/v0.0.39/associations | Delete associations
[**slurmdb_v0039_delete_cluster**](SlurmApi.md#slurmdb_v0039_delete_cluster) | **DELETE** /slurmdb/v0.0.39/cluster/{cluster_name} | Delete cluster
[**slurmdb_v0039_delete_qos**](SlurmApi.md#slurmdb_v0039_delete_qos) | **DELETE** /slurmdb/v0.0.39/qos/{qos_name} | Delete QOS
[**slurmdb_v0039_delete_user**](SlurmApi.md#slurmdb_v0039_delete_user) | **DELETE** /slurmdb/v0.0.39/user/{user_name} | Delete user
[**slurmdb_v0039_delete_wckey**](SlurmApi.md#slurmdb_v0039_delete_wckey) | **DELETE** /slurmdb/v0.0.39/wckey/{wckey} | Delete wckey
[**slurmdb_v0039_diag**](SlurmApi.md#slurmdb_v0039_diag) | **GET** /slurmdb/v0.0.39/diag | Get slurmdb diagnostics
[**slurmdb_v0039_get_account**](SlurmApi.md#slurmdb_v0039_get_account) | **GET** /slurmdb/v0.0.39/account/{account_name} | Get account info
[**slurmdb_v0039_get_accounts**](SlurmApi.md#slurmdb_v0039_get_accounts) | **GET** /slurmdb/v0.0.39/accounts | Get account list
[**slurmdb_v0039_get_association**](SlurmApi.md#slurmdb_v0039_get_association) | **GET** /slurmdb/v0.0.39/association | Get association info
[**slurmdb_v0039_get_associations**](SlurmApi.md#slurmdb_v0039_get_associations) | **GET** /slurmdb/v0.0.39/associations | Get association list
[**slurmdb_v0039_get_cluster**](SlurmApi.md#slurmdb_v0039_get_cluster) | **GET** /slurmdb/v0.0.39/cluster/{cluster_name} | Get cluster info
[**slurmdb_v0039_get_clusters**](SlurmApi.md#slurmdb_v0039_get_clusters) | **GET** /slurmdb/v0.0.39/clusters | Get cluster list
[**slurmdb_v0039_get_config**](SlurmApi.md#slurmdb_v0039_get_config) | **GET** /slurmdb/v0.0.39/config | Dump all configuration information
[**slurmdb_v0039_get_job**](SlurmApi.md#slurmdb_v0039_get_job) | **GET** /slurmdb/v0.0.39/job/{job_id} | Get job info
[**slurmdb_v0039_get_jobs**](SlurmApi.md#slurmdb_v0039_get_jobs) | **GET** /slurmdb/v0.0.39/jobs | Get job list
[**slurmdb_v0039_get_qos**](SlurmApi.md#slurmdb_v0039_get_qos) | **GET** /slurmdb/v0.0.39/qos | Get QOS list
[**slurmdb_v0039_get_single_qos**](SlurmApi.md#slurmdb_v0039_get_single_qos) | **GET** /slurmdb/v0.0.39/qos/{qos_name} | Get QOS info
[**slurmdb_v0039_get_tres**](SlurmApi.md#slurmdb_v0039_get_tres) | **GET** /slurmdb/v0.0.39/tres | Get TRES info
[**slurmdb_v0039_get_user**](SlurmApi.md#slurmdb_v0039_get_user) | **GET** /slurmdb/v0.0.39/user/{user_name} | Get user info
[**slurmdb_v0039_get_users**](SlurmApi.md#slurmdb_v0039_get_users) | **GET** /slurmdb/v0.0.39/users | Get user list
[**slurmdb_v0039_get_wckey**](SlurmApi.md#slurmdb_v0039_get_wckey) | **GET** /slurmdb/v0.0.39/wckey/{wckey} | Get wckey info
[**slurmdb_v0039_get_wckeys**](SlurmApi.md#slurmdb_v0039_get_wckeys) | **GET** /slurmdb/v0.0.39/wckeys | Get wckey list
[**slurmdb_v0039_set_config**](SlurmApi.md#slurmdb_v0039_set_config) | **POST** /slurmdb/v0.0.39/config | Load all configuration information
[**slurmdb_v0039_update_accounts**](SlurmApi.md#slurmdb_v0039_update_accounts) | **POST** /slurmdb/v0.0.39/accounts | Update accounts
[**slurmdb_v0039_update_associations**](SlurmApi.md#slurmdb_v0039_update_associations) | **POST** /slurmdb/v0.0.39/associations | Set associations info
[**slurmdb_v0039_update_qos**](SlurmApi.md#slurmdb_v0039_update_qos) | **POST** /slurmdb/v0.0.39/qos | Set QOS info
[**slurmdb_v0039_update_tres**](SlurmApi.md#slurmdb_v0039_update_tres) | **POST** /slurmdb/v0.0.39/tres | Set TRES info
[**slurmdb_v0039_update_users**](SlurmApi.md#slurmdb_v0039_update_users) | **POST** /slurmdb/v0.0.39/users | Update user



## slurm_v0039_cancel_job

> crate::models::Status slurm_v0039_cancel_job(job_id, signal)
cancel or signal job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Slurm Job ID | [required] |
**signal** | Option<**String**> | signal to send to job |  |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_delete_node

> crate::models::Status slurm_v0039_delete_node(node_name)
delete node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_name** | **String** | Slurm Node Name | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_diag

> crate::models::V0Period0Period39Diag slurm_v0039_diag()
get diagnostics

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V0Period0Period39Diag**](v0.0.39_diag.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_get_job

> crate::models::V0Period0Period39JobsResponse slurm_v0039_get_job(job_id)
get job info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Slurm JobID | [required] |

### Return type

[**crate::models::V0Period0Period39JobsResponse**](v0.0.39_jobs_response.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_get_jobs

> crate::models::V0Period0Period39JobsResponse slurm_v0039_get_jobs(update_time)
get list of jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period39JobsResponse**](v0.0.39_jobs_response.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_get_node

> crate::models::V0Period0Period39NodesResponse slurm_v0039_get_node(node_name)
get node info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_name** | **String** | Slurm Node Name | [required] |

### Return type

[**crate::models::V0Period0Period39NodesResponse**](v0.0.39_nodes_response.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_get_nodes

> crate::models::V0Period0Period39NodesResponse slurm_v0039_get_nodes(update_time)
get all node info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period39NodesResponse**](v0.0.39_nodes_response.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_get_partition

> crate::models::V0Period0Period39PartitionsResponse slurm_v0039_get_partition(partition_name, update_time)
get partition info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition_name** | **String** | Slurm Partition Name | [required] |
**update_time** | Option<**i64**> | Filter if there were no partition changes (not limited to partition in URL endpoint) since update_time. |  |

### Return type

[**crate::models::V0Period0Period39PartitionsResponse**](v0.0.39_partitions_response.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_get_partitions

> crate::models::V0Period0Period39PartitionsResponse slurm_v0039_get_partitions(update_time)
get all partition info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period39PartitionsResponse**](v0.0.39_partitions_response.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_get_reservation

> crate::models::V0Period0Period39ReservationsResponse slurm_v0039_get_reservation(reservation_name, update_time)
get reservation info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reservation_name** | **String** | Slurm Reservation Name | [required] |
**update_time** | Option<**i64**> | Filter if no reservation (not limited to reservation in URL) changed since update_time. |  |

### Return type

[**crate::models::V0Period0Period39ReservationsResponse**](v0.0.39_reservations_response.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_get_reservations

> crate::models::V0Period0Period39ReservationsResponse slurm_v0039_get_reservations(update_time)
get all reservation info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period39ReservationsResponse**](v0.0.39_reservations_response.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_ping

> crate::models::V0Period0Period39Pings slurm_v0039_ping()
ping test

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V0Period0Period39Pings**](v0.0.39_pings.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_slurmctld_get_licenses

> crate::models::V0Period0Period39LicensesInfo slurm_v0039_slurmctld_get_licenses()
get all Slurm tracked license info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V0Period0Period39LicensesInfo**](v0.0.39_licenses_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_submit_job

> crate::models::V0Period0Period39JobSubmissionResponse slurm_v0039_submit_job(v0_period0_period39_job_submission)
submit new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v0_period0_period39_job_submission** | [**V0Period0Period39JobSubmission**](V0Period0Period39JobSubmission.md) | submit new job | [required] |

### Return type

[**crate::models::V0Period0Period39JobSubmissionResponse**](v0.0.39_job_submission_response.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_update_job

> crate::models::V0Period0Period39JobUpdateResponse slurm_v0039_update_job(job_id, v0_period0_period39_job_desc_msg)
update job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Slurm Job ID | [required] |
**v0_period0_period39_job_desc_msg** | [**V0Period0Period39JobDescMsg**](V0Period0Period39JobDescMsg.md) | update job | [required] |

### Return type

[**crate::models::V0Period0Period39JobUpdateResponse**](v0.0.39_job_update_response.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0039_update_node

> crate::models::Status slurm_v0039_update_node(node_name, v0_period0_period39_update_node_msg)
update node properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_name** | **String** | Slurm Node Name | [required] |
**v0_period0_period39_update_node_msg** | [**V0Period0Period39UpdateNodeMsg**](V0Period0Period39UpdateNodeMsg.md) | update node | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_add_clusters

> crate::models::Status slurmdb_v0039_add_clusters(dbv0_period0_period39_clusters_info)
Add clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period39_clusters_info** | [**Dbv0Period0Period39ClustersInfo**](Dbv0Period0Period39ClustersInfo.md) | Add or update clusters | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_add_wckeys

> crate::models::Status slurmdb_v0039_add_wckeys(dbv0_period0_period39_wckey_info)
Add wckeys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period39_wckey_info** | Option<[**Dbv0Period0Period39WckeyInfo**](Dbv0Period0Period39WckeyInfo.md)> | add wckeys |  |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_delete_account

> crate::models::Status slurmdb_v0039_delete_account(account_name)
Delete account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** | Slurm Account Name | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_delete_association

> crate::models::Dbv0Period0Period39ResponseAssociationsDelete slurmdb_v0039_delete_association(cluster, account, user, partition)
Delete association

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | Option<**String**> | Cluster name |  |
**account** | Option<**String**> | Account name |  |
**user** | Option<**String**> | User name |  |
**partition** | Option<**String**> | Partition Name |  |

### Return type

[**crate::models::Dbv0Period0Period39ResponseAssociationsDelete**](dbv0.0.39_response_associations_delete.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_delete_associations

> crate::models::Dbv0Period0Period39ResponseAssociationsDelete slurmdb_v0039_delete_associations(cluster, account, user, partition)
Delete associations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | Option<**String**> | Cluster name |  |
**account** | Option<**String**> | Account name |  |
**user** | Option<**String**> | User name |  |
**partition** | Option<**String**> | Partition Name |  |

### Return type

[**crate::models::Dbv0Period0Period39ResponseAssociationsDelete**](dbv0.0.39_response_associations_delete.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_delete_cluster

> crate::models::Status slurmdb_v0039_delete_cluster(cluster_name)
Delete cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** | Slurm cluster name | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_delete_qos

> crate::models::Status slurmdb_v0039_delete_qos(qos_name)
Delete QOS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qos_name** | **String** | Slurm QOS Name | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_delete_user

> crate::models::Status slurmdb_v0039_delete_user(user_name)
Delete user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_name** | **String** | Slurm User Name | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_delete_wckey

> crate::models::Status slurmdb_v0039_delete_wckey(wckey)
Delete wckey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wckey** | **String** | Slurm wckey name | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_diag

> crate::models::Dbv0Period0Period39Diag slurmdb_v0039_diag()
Get slurmdb diagnostics

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period39Diag**](dbv0.0.39_diag.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_account

> crate::models::Dbv0Period0Period39AccountInfo slurmdb_v0039_get_account(account_name, with_deleted)
Get account info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** | Slurm Account Name | [required] |
**with_deleted** | Option<**String**> | Include deleted accounts. False by default. |  |[default to false]

### Return type

[**crate::models::Dbv0Period0Period39AccountInfo**](dbv0.0.39_account_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_accounts

> crate::models::Dbv0Period0Period39AccountInfo slurmdb_v0039_get_accounts(with_deleted)
Get account list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**with_deleted** | Option<**String**> | Include deleted accounts. False by default. |  |[default to false]

### Return type

[**crate::models::Dbv0Period0Period39AccountInfo**](dbv0.0.39_account_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_association

> crate::models::Dbv0Period0Period39AssociationsInfo slurmdb_v0039_get_association(cluster, account, user, partition)
Get association info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | Option<**String**> | Cluster name |  |
**account** | Option<**String**> | Account name |  |
**user** | Option<**String**> | User name |  |
**partition** | Option<**String**> | Partition Name |  |

### Return type

[**crate::models::Dbv0Period0Period39AssociationsInfo**](dbv0.0.39_associations_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_associations

> crate::models::Dbv0Period0Period39AssociationsInfo slurmdb_v0039_get_associations(cluster, account, user, partition)
Get association list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | Option<**String**> | Cluster name |  |
**account** | Option<**String**> | Account name |  |
**user** | Option<**String**> | User name |  |
**partition** | Option<**String**> | Partition Name |  |

### Return type

[**crate::models::Dbv0Period0Period39AssociationsInfo**](dbv0.0.39_associations_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_cluster

> crate::models::Dbv0Period0Period39ClustersInfo slurmdb_v0039_get_cluster(cluster_name)
Get cluster info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** | Slurm cluster name | [required] |

### Return type

[**crate::models::Dbv0Period0Period39ClustersInfo**](dbv0.0.39_clusters_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_clusters

> crate::models::Dbv0Period0Period39ClustersInfo slurmdb_v0039_get_clusters()
Get cluster list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period39ClustersInfo**](dbv0.0.39_clusters_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_config

> crate::models::Dbv0Period0Period39ConfigInfo slurmdb_v0039_get_config()
Dump all configuration information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period39ConfigInfo**](dbv0.0.39_config_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_job

> crate::models::Dbv0Period0Period39JobInfo slurmdb_v0039_get_job(job_id)
Get job info

This endpoint may return multiple job entries since job_id is not a unique key - only the tuple (cluster, job_id, start_time) is unique. If the requested job_id is a component of a heterogeneous job all components are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Slurm JobID | [required] |

### Return type

[**crate::models::Dbv0Period0Period39JobInfo**](dbv0.0.39_job_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_jobs

> crate::models::Dbv0Period0Period39JobInfo slurmdb_v0039_get_jobs(users, submit_time, start_time, end_time, account, association, cluster, constraints, cpus_max, cpus_min, skip_steps, disable_wait_for_result, exit_code, format, group, job_name, nodes_max, nodes_min, partition, qos, reason, reservation, state, step, node, wckey)
Get job list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users** | Option<**String**> | Filter by comma delimited list of user names |  |
**submit_time** | Option<**String**> | Filter by submission time  Accepted formats:  HH:MM[:SS] [AM|PM]  MMDD[YY] or MM/DD[/YY] or MM.DD[.YY]  MM/DD[/YY]-HH:MM[:SS]  YYYY-MM-DD[THH:MM[:SS]] |  |
**start_time** | Option<**String**> | Filter by start time  Accepted formats:  HH:MM[:SS] [AM|PM]  MMDD[YY] or MM/DD[/YY] or MM.DD[.YY]  MM/DD[/YY]-HH:MM[:SS]  YYYY-MM-DD[THH:MM[:SS]] |  |
**end_time** | Option<**String**> | Filter by end time  Accepted formats:  HH:MM[:SS] [AM|PM]  MMDD[YY] or MM/DD[/YY] or MM.DD[.YY]  MM/DD[/YY]-HH:MM[:SS]  YYYY-MM-DD[THH:MM[:SS]] |  |
**account** | Option<**String**> | Comma delimited list of accounts to match |  |
**association** | Option<**String**> | Comma delimited list of associations to match |  |
**cluster** | Option<**String**> | Comma delimited list of cluster to match |  |
**constraints** | Option<**String**> | Comma delimited list of constraints to match |  |
**cpus_max** | Option<**String**> | Number of CPUs high range |  |
**cpus_min** | Option<**String**> | Number of CPUs low range |  |
**skip_steps** | Option<**String**> | Report job step information |  |[default to false]
**disable_wait_for_result** | Option<**String**> | Disable waiting for result from slurmdbd |  |[default to false]
**exit_code** | Option<**String**> | Exit code of job |  |
**format** | Option<**String**> | Comma delimited list of formats to match |  |
**group** | Option<**String**> | Comma delimited list of groups to match |  |
**job_name** | Option<**String**> | Comma delimited list of job names to match |  |
**nodes_max** | Option<**String**> | Number of nodes high range |  |
**nodes_min** | Option<**String**> | Number of nodes low range |  |
**partition** | Option<**String**> | Comma delimited list of partitions to match |  |
**qos** | Option<**String**> | Comma delimited list of QOS to match |  |
**reason** | Option<**String**> | Comma delimited list of job reasons to match |  |
**reservation** | Option<**String**> | Comma delimited list of reservations to match |  |
**state** | Option<**String**> | Comma delimited list of states to match |  |
**step** | Option<**String**> | Comma delimited list of job steps to match |  |
**node** | Option<**String**> | Comma delimited list of used nodes to match |  |
**wckey** | Option<**String**> | Comma delimited list of wckeys to match |  |

### Return type

[**crate::models::Dbv0Period0Period39JobInfo**](dbv0.0.39_job_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_qos

> crate::models::Dbv0Period0Period39QosInfo slurmdb_v0039_get_qos(with_deleted)
Get QOS list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**with_deleted** | Option<**String**> | Include deleted QOSs. False by default. |  |[default to false]

### Return type

[**crate::models::Dbv0Period0Period39QosInfo**](dbv0.0.39_qos_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_single_qos

> crate::models::Dbv0Period0Period39QosInfo slurmdb_v0039_get_single_qos(qos_name, with_deleted)
Get QOS info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qos_name** | **String** | Slurm QOS Name | [required] |
**with_deleted** | Option<**String**> | Include deleted QOSs. False by default. |  |[default to false]

### Return type

[**crate::models::Dbv0Period0Period39QosInfo**](dbv0.0.39_qos_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_tres

> crate::models::Dbv0Period0Period39TresInfo slurmdb_v0039_get_tres()
Get TRES info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period39TresInfo**](dbv0.0.39_tres_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_user

> crate::models::Dbv0Period0Period39UserInfo slurmdb_v0039_get_user(user_name, with_deleted)
Get user info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_name** | **String** | Slurm User Name | [required] |
**with_deleted** | Option<**String**> | Include deleted users. False by default. |  |[default to false]

### Return type

[**crate::models::Dbv0Period0Period39UserInfo**](dbv0.0.39_user_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_users

> crate::models::Dbv0Period0Period39UserInfo slurmdb_v0039_get_users(with_deleted)
Get user list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**with_deleted** | Option<**String**> | Include deleted users. False by default. |  |[default to false]

### Return type

[**crate::models::Dbv0Period0Period39UserInfo**](dbv0.0.39_user_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_wckey

> crate::models::Dbv0Period0Period39WckeyInfo slurmdb_v0039_get_wckey(wckey)
Get wckey info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wckey** | **String** | Slurm wckey name | [required] |

### Return type

[**crate::models::Dbv0Period0Period39WckeyInfo**](dbv0.0.39_wckey_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_get_wckeys

> crate::models::Dbv0Period0Period39WckeyInfo slurmdb_v0039_get_wckeys()
Get wckey list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period39WckeyInfo**](dbv0.0.39_wckey_info.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_set_config

> crate::models::Status slurmdb_v0039_set_config(dbv0_period0_period39_set_config)
Load all configuration information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period39_set_config** | Option<[**Dbv0Period0Period39SetConfig**](Dbv0Period0Period39SetConfig.md)> | Add or update config |  |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_update_accounts

> crate::models::Status slurmdb_v0039_update_accounts(dbv0_period0_period39_account_info)
Update accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period39_account_info** | [**Dbv0Period0Period39AccountInfo**](Dbv0Period0Period39AccountInfo.md) | update/create accounts | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_update_associations

> crate::models::Status slurmdb_v0039_update_associations(dbv0_period0_period39_associations_info)
Set associations info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period39_associations_info** | [**Dbv0Period0Period39AssociationsInfo**](Dbv0Period0Period39AssociationsInfo.md) | Add or update associations | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_update_qos

> crate::models::Status slurmdb_v0039_update_qos(dbv0_period0_period39_update_qos)
Set QOS info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period39_update_qos** | [**Dbv0Period0Period39UpdateQos**](Dbv0Period0Period39UpdateQos.md) | Add or update QOSs | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_update_tres

> crate::models::Status slurmdb_v0039_update_tres(dbv0_period0_period39_tres_update)
Set TRES info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period39_tres_update** | [**Dbv0Period0Period39TresUpdate**](Dbv0Period0Period39TresUpdate.md) | Add or Update TRES | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0039_update_users

> crate::models::Status slurmdb_v0039_update_users(dbv0_period0_period39_update_users)
Update user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period39_update_users** | [**Dbv0Period0Period39UpdateUsers**](Dbv0Period0Period39UpdateUsers.md) | add or update user | [required] |

### Return type

[**crate::models::Status**](status.md)

### Authorization

[user](../README.md#user), [bearerAuth](../README.md#bearerAuth), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

