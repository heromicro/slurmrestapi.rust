# \SlurmApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**slurm_v0038_cancel_job**](SlurmApi.md#slurm_v0038_cancel_job) | **DELETE** /slurm/v0.0.38/job/{job_id} | cancel or signal job
[**slurm_v0038_diag**](SlurmApi.md#slurm_v0038_diag) | **GET** /slurm/v0.0.38/diag | get diagnostics
[**slurm_v0038_get_job**](SlurmApi.md#slurm_v0038_get_job) | **GET** /slurm/v0.0.38/job/{job_id} | get job info
[**slurm_v0038_get_jobs**](SlurmApi.md#slurm_v0038_get_jobs) | **GET** /slurm/v0.0.38/jobs | get list of jobs
[**slurm_v0038_get_node**](SlurmApi.md#slurm_v0038_get_node) | **GET** /slurm/v0.0.38/node/{node_name} | get node info
[**slurm_v0038_get_nodes**](SlurmApi.md#slurm_v0038_get_nodes) | **GET** /slurm/v0.0.38/nodes | get all node info
[**slurm_v0038_get_partition**](SlurmApi.md#slurm_v0038_get_partition) | **GET** /slurm/v0.0.38/partition/{partition_name} | get partition info
[**slurm_v0038_get_partitions**](SlurmApi.md#slurm_v0038_get_partitions) | **GET** /slurm/v0.0.38/partitions | get all partition info
[**slurm_v0038_get_reservation**](SlurmApi.md#slurm_v0038_get_reservation) | **GET** /slurm/v0.0.38/reservation/{reservation_name} | get reservation info
[**slurm_v0038_get_reservations**](SlurmApi.md#slurm_v0038_get_reservations) | **GET** /slurm/v0.0.38/reservations | get all reservation info
[**slurm_v0038_ping**](SlurmApi.md#slurm_v0038_ping) | **GET** /slurm/v0.0.38/ping | ping test
[**slurm_v0038_slurmctld_get_licenses**](SlurmApi.md#slurm_v0038_slurmctld_get_licenses) | **GET** /slurm/v0.0.38/licenses | get all Slurm tracked license info
[**slurm_v0038_submit_job**](SlurmApi.md#slurm_v0038_submit_job) | **POST** /slurm/v0.0.38/job/submit | submit new job
[**slurm_v0038_update_job**](SlurmApi.md#slurm_v0038_update_job) | **POST** /slurm/v0.0.38/job/{job_id} | update job
[**slurmdb_v0038_add_clusters**](SlurmApi.md#slurmdb_v0038_add_clusters) | **POST** /slurmdb/v0.0.38/clusters | Add clusters
[**slurmdb_v0038_add_wckeys**](SlurmApi.md#slurmdb_v0038_add_wckeys) | **POST** /slurmdb/v0.0.38/wckeys | Add wckeys
[**slurmdb_v0038_delete_account**](SlurmApi.md#slurmdb_v0038_delete_account) | **DELETE** /slurmdb/v0.0.38/account/{account_name} | Delete account
[**slurmdb_v0038_delete_association**](SlurmApi.md#slurmdb_v0038_delete_association) | **DELETE** /slurmdb/v0.0.38/association | Delete association
[**slurmdb_v0038_delete_associations**](SlurmApi.md#slurmdb_v0038_delete_associations) | **DELETE** /slurmdb/v0.0.38/associations | Delete associations
[**slurmdb_v0038_delete_cluster**](SlurmApi.md#slurmdb_v0038_delete_cluster) | **DELETE** /slurmdb/v0.0.38/cluster/{cluster_name} | Delete cluster
[**slurmdb_v0038_delete_qos**](SlurmApi.md#slurmdb_v0038_delete_qos) | **DELETE** /slurmdb/v0.0.38/qos/{qos_name} | Delete QOS
[**slurmdb_v0038_delete_user**](SlurmApi.md#slurmdb_v0038_delete_user) | **DELETE** /slurmdb/v0.0.38/user/{user_name} | Delete user
[**slurmdb_v0038_delete_wckey**](SlurmApi.md#slurmdb_v0038_delete_wckey) | **DELETE** /slurmdb/v0.0.38/wckey/{wckey} | Delete wckey
[**slurmdb_v0038_diag**](SlurmApi.md#slurmdb_v0038_diag) | **GET** /slurmdb/v0.0.38/diag | Get slurmdb diagnostics
[**slurmdb_v0038_get_account**](SlurmApi.md#slurmdb_v0038_get_account) | **GET** /slurmdb/v0.0.38/account/{account_name} | Get account info
[**slurmdb_v0038_get_accounts**](SlurmApi.md#slurmdb_v0038_get_accounts) | **GET** /slurmdb/v0.0.38/accounts | Get account list
[**slurmdb_v0038_get_association**](SlurmApi.md#slurmdb_v0038_get_association) | **GET** /slurmdb/v0.0.38/association | Get association info
[**slurmdb_v0038_get_associations**](SlurmApi.md#slurmdb_v0038_get_associations) | **GET** /slurmdb/v0.0.38/associations | Get association list
[**slurmdb_v0038_get_cluster**](SlurmApi.md#slurmdb_v0038_get_cluster) | **GET** /slurmdb/v0.0.38/cluster/{cluster_name} | Get cluster info
[**slurmdb_v0038_get_clusters**](SlurmApi.md#slurmdb_v0038_get_clusters) | **GET** /slurmdb/v0.0.38/clusters | Get cluster list
[**slurmdb_v0038_get_config**](SlurmApi.md#slurmdb_v0038_get_config) | **GET** /slurmdb/v0.0.38/config | Dump all configuration information
[**slurmdb_v0038_get_job**](SlurmApi.md#slurmdb_v0038_get_job) | **GET** /slurmdb/v0.0.38/job/{job_id} | Get job info
[**slurmdb_v0038_get_jobs**](SlurmApi.md#slurmdb_v0038_get_jobs) | **GET** /slurmdb/v0.0.38/jobs | Get job list
[**slurmdb_v0038_get_qos**](SlurmApi.md#slurmdb_v0038_get_qos) | **GET** /slurmdb/v0.0.38/qos | Get QOS list
[**slurmdb_v0038_get_single_qos**](SlurmApi.md#slurmdb_v0038_get_single_qos) | **GET** /slurmdb/v0.0.38/qos/{qos_name} | Get QOS info
[**slurmdb_v0038_get_tres**](SlurmApi.md#slurmdb_v0038_get_tres) | **GET** /slurmdb/v0.0.38/tres | Get TRES info
[**slurmdb_v0038_get_user**](SlurmApi.md#slurmdb_v0038_get_user) | **GET** /slurmdb/v0.0.38/user/{user_name} | Get user info
[**slurmdb_v0038_get_users**](SlurmApi.md#slurmdb_v0038_get_users) | **GET** /slurmdb/v0.0.38/users | Get user list
[**slurmdb_v0038_get_wckey**](SlurmApi.md#slurmdb_v0038_get_wckey) | **GET** /slurmdb/v0.0.38/wckey/{wckey} | Get wckey info
[**slurmdb_v0038_get_wckeys**](SlurmApi.md#slurmdb_v0038_get_wckeys) | **GET** /slurmdb/v0.0.38/wckeys | Get wckey list
[**slurmdb_v0038_set_config**](SlurmApi.md#slurmdb_v0038_set_config) | **POST** /slurmdb/v0.0.38/config | Load all configuration information
[**slurmdb_v0038_update_account**](SlurmApi.md#slurmdb_v0038_update_account) | **POST** /slurmdb/v0.0.38/accounts | Update accounts
[**slurmdb_v0038_update_associations**](SlurmApi.md#slurmdb_v0038_update_associations) | **POST** /slurmdb/v0.0.38/associations | Set associations info
[**slurmdb_v0038_update_qos**](SlurmApi.md#slurmdb_v0038_update_qos) | **POST** /slurmdb/v0.0.38/qos | Set QOS info
[**slurmdb_v0038_update_tres**](SlurmApi.md#slurmdb_v0038_update_tres) | **POST** /slurmdb/v0.0.38/tres | Set TRES info
[**slurmdb_v0038_update_users**](SlurmApi.md#slurmdb_v0038_update_users) | **POST** /slurmdb/v0.0.38/users | Update user



## slurm_v0038_cancel_job

> slurm_v0038_cancel_job(job_id, signal)
cancel or signal job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Slurm Job ID | [required] |
**signal** | Option<[**V0Period0Period38Signal**](.md)> | signal to send to job |  |

### Return type

 (empty response body)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_diag

> crate::models::V0Period0Period38Diag slurm_v0038_diag()
get diagnostics

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V0Period0Period38Diag**](v0.0.38_diag.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_get_job

> crate::models::V0Period0Period38JobsResponse slurm_v0038_get_job(job_id)
get job info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Slurm JobID | [required] |

### Return type

[**crate::models::V0Period0Period38JobsResponse**](v0.0.38_jobs_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_get_jobs

> crate::models::V0Period0Period38JobsResponse slurm_v0038_get_jobs(update_time)
get list of jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period38JobsResponse**](v0.0.38_jobs_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_get_node

> crate::models::V0Period0Period38NodesResponse slurm_v0038_get_node(node_name)
get node info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_name** | **String** | Slurm Node Name | [required] |

### Return type

[**crate::models::V0Period0Period38NodesResponse**](v0.0.38_nodes_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_get_nodes

> crate::models::V0Period0Period38NodesResponse slurm_v0038_get_nodes(update_time)
get all node info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period38NodesResponse**](v0.0.38_nodes_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_get_partition

> crate::models::V0Period0Period38PartitionsResponse slurm_v0038_get_partition(partition_name, update_time)
get partition info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition_name** | **String** | Slurm Partition Name | [required] |
**update_time** | Option<**i64**> | Filter if there were no partition changes (not limited to partition in URL endpoint) since update_time. |  |

### Return type

[**crate::models::V0Period0Period38PartitionsResponse**](v0.0.38_partitions_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_get_partitions

> crate::models::V0Period0Period38PartitionsResponse slurm_v0038_get_partitions(update_time)
get all partition info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period38PartitionsResponse**](v0.0.38_partitions_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_get_reservation

> crate::models::V0Period0Period38ReservationsResponse slurm_v0038_get_reservation(reservation_name, update_time)
get reservation info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reservation_name** | **String** | Slurm Reservation Name | [required] |
**update_time** | Option<**i64**> | Filter if no reservation (not limited to reservation in URL) changed since update_time. |  |

### Return type

[**crate::models::V0Period0Period38ReservationsResponse**](v0.0.38_reservations_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_get_reservations

> crate::models::V0Period0Period38ReservationsResponse slurm_v0038_get_reservations(update_time)
get all reservation info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period38ReservationsResponse**](v0.0.38_reservations_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_ping

> crate::models::V0Period0Period38Pings slurm_v0038_ping()
ping test

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V0Period0Period38Pings**](v0.0.38_pings.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_slurmctld_get_licenses

> crate::models::V0Period0Period38Licenses slurm_v0038_slurmctld_get_licenses()
get all Slurm tracked license info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V0Period0Period38Licenses**](v0.0.38_licenses.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_submit_job

> crate::models::V0Period0Period38JobSubmissionResponse slurm_v0038_submit_job(v0_period0_period38_job_submission)
submit new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v0_period0_period38_job_submission** | [**V0Period0Period38JobSubmission**](V0Period0Period38JobSubmission.md) | submit new job | [required] |

### Return type

[**crate::models::V0Period0Period38JobSubmissionResponse**](v0.0.38_job_submission_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurm_v0038_update_job

> slurm_v0038_update_job(job_id, v0_period0_period38_job_properties)
update job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Slurm Job ID | [required] |
**v0_period0_period38_job_properties** | [**V0Period0Period38JobProperties**](V0Period0Period38JobProperties.md) | update job | [required] |

### Return type

 (empty response body)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_add_clusters

> crate::models::Dbv0Period0Period38ResponseClusterAdd slurmdb_v0038_add_clusters(dbv0_period0_period38_clusters_properties)
Add clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period38_clusters_properties** | [**Dbv0Period0Period38ClustersProperties**](Dbv0Period0Period38ClustersProperties.md) | Add or update clusters | [required] |

### Return type

[**crate::models::Dbv0Period0Period38ResponseClusterAdd**](dbv0.0.38_response_cluster_add.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_add_wckeys

> crate::models::Dbv0Period0Period38ResponseWckeyAdd slurmdb_v0038_add_wckeys(dbv0_period0_period38_wckey_info)
Add wckeys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period38_wckey_info** | Option<[**Dbv0Period0Period38WckeyInfo**](Dbv0Period0Period38WckeyInfo.md)> | add wckeys |  |

### Return type

[**crate::models::Dbv0Period0Period38ResponseWckeyAdd**](dbv0.0.38_response_wckey_add.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_delete_account

> crate::models::Dbv0Period0Period38ResponseAccountDelete slurmdb_v0038_delete_account(account_name)
Delete account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** | Slurm Account Name | [required] |

### Return type

[**crate::models::Dbv0Period0Period38ResponseAccountDelete**](dbv0.0.38_response_account_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_delete_association

> crate::models::Dbv0Period0Period38ResponseAssociationsDelete slurmdb_v0038_delete_association(cluster, account, user, partition)
Delete association

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | Option<**String**> | Cluster name |  |
**account** | Option<**String**> | Account name |  |
**user** | Option<**String**> | User name |  |
**partition** | Option<**String**> | Partition Name |  |

### Return type

[**crate::models::Dbv0Period0Period38ResponseAssociationsDelete**](dbv0.0.38_response_associations_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_delete_associations

> crate::models::Dbv0Period0Period38ResponseAssociationsDelete slurmdb_v0038_delete_associations(cluster, account, user, partition)
Delete associations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | Option<**String**> | Cluster name |  |
**account** | Option<**String**> | Account name |  |
**user** | Option<**String**> | User name |  |
**partition** | Option<**String**> | Partition Name |  |

### Return type

[**crate::models::Dbv0Period0Period38ResponseAssociationsDelete**](dbv0.0.38_response_associations_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_delete_cluster

> crate::models::Dbv0Period0Period38ResponseClusterDelete slurmdb_v0038_delete_cluster(cluster_name)
Delete cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** | Slurm cluster name | [required] |

### Return type

[**crate::models::Dbv0Period0Period38ResponseClusterDelete**](dbv0.0.38_response_cluster_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_delete_qos

> crate::models::Dbv0Period0Period38ResponseQosDelete slurmdb_v0038_delete_qos(qos_name)
Delete QOS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qos_name** | **String** | Slurm QOS Name | [required] |

### Return type

[**crate::models::Dbv0Period0Period38ResponseQosDelete**](dbv0.0.38_response_qos_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_delete_user

> crate::models::Dbv0Period0Period38ResponseUserDelete slurmdb_v0038_delete_user(user_name)
Delete user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_name** | **String** | Slurm User Name | [required] |

### Return type

[**crate::models::Dbv0Period0Period38ResponseUserDelete**](dbv0.0.38_response_user_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_delete_wckey

> crate::models::Dbv0Period0Period38ResponseWckeyDelete slurmdb_v0038_delete_wckey(wckey)
Delete wckey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wckey** | **String** | Slurm wckey name | [required] |

### Return type

[**crate::models::Dbv0Period0Period38ResponseWckeyDelete**](dbv0.0.38_response_wckey_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_diag

> crate::models::Dbv0Period0Period38Diag slurmdb_v0038_diag()
Get slurmdb diagnostics

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period38Diag**](dbv0.0.38_diag.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_account

> crate::models::Dbv0Period0Period38AccountInfo slurmdb_v0038_get_account(account_name, with_deleted)
Get account info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** | Slurm Account Name | [required] |
**with_deleted** | Option<**bool**> | Include deleted accounts. False by default. |  |

### Return type

[**crate::models::Dbv0Period0Period38AccountInfo**](dbv0.0.38_account_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_accounts

> crate::models::Dbv0Period0Period38AccountInfo slurmdb_v0038_get_accounts(with_deleted)
Get account list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**with_deleted** | Option<**bool**> | Include deleted accounts. False by default. |  |

### Return type

[**crate::models::Dbv0Period0Period38AccountInfo**](dbv0.0.38_account_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_association

> crate::models::Dbv0Period0Period38AssociationsInfo slurmdb_v0038_get_association(cluster, account, user, partition)
Get association info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | Option<**String**> | Cluster name |  |
**account** | Option<**String**> | Account name |  |
**user** | Option<**String**> | User name |  |
**partition** | Option<**String**> | Partition Name |  |

### Return type

[**crate::models::Dbv0Period0Period38AssociationsInfo**](dbv0.0.38_associations_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_associations

> crate::models::Dbv0Period0Period38AssociationsInfo slurmdb_v0038_get_associations(cluster, account, user, partition)
Get association list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | Option<**String**> | Cluster name |  |
**account** | Option<**String**> | Account name |  |
**user** | Option<**String**> | User name |  |
**partition** | Option<**String**> | Partition Name |  |

### Return type

[**crate::models::Dbv0Period0Period38AssociationsInfo**](dbv0.0.38_associations_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_cluster

> crate::models::Dbv0Period0Period38ClusterInfo slurmdb_v0038_get_cluster(cluster_name)
Get cluster info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** | Slurm cluster name | [required] |

### Return type

[**crate::models::Dbv0Period0Period38ClusterInfo**](dbv0.0.38_cluster_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_clusters

> crate::models::Dbv0Period0Period38ClusterInfo slurmdb_v0038_get_clusters()
Get cluster list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period38ClusterInfo**](dbv0.0.38_cluster_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_config

> crate::models::Dbv0Period0Period38ConfigInfo slurmdb_v0038_get_config()
Dump all configuration information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period38ConfigInfo**](dbv0.0.38_config_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_job

> crate::models::Dbv0Period0Period38JobInfo slurmdb_v0038_get_job(job_id)
Get job info

This endpoint may return multiple job entries since job_id is not a unique key - only the tuple (cluster, job_id, start_time) is unique. If the requested job_id is a component of a heterogeneous job all components are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Slurm JobID | [required] |

### Return type

[**crate::models::Dbv0Period0Period38JobInfo**](dbv0.0.38_job_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_jobs

> crate::models::Dbv0Period0Period38JobInfo slurmdb_v0038_get_jobs(submit_time, start_time, end_time, account, association, cluster, constraints, cpus_max, cpus_min, skip_steps, disable_wait_for_result, exit_code, format, group, job_name, nodes_max, nodes_min, partition, qos, reason, reservation, state, step, node, wckey)
Get job list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_time** | Option<**String**> | Filter by submission time  Accepted formats:  HH:MM[:SS] [AM|PM]  MMDD[YY] or MM/DD[/YY] or MM.DD[.YY]  MM/DD[/YY]-HH:MM[:SS]  YYYY-MM-DD[THH:MM[:SS]] |  |
**start_time** | Option<**String**> | Filter by start time  Accepted formats:  HH:MM[:SS] [AM|PM]  MMDD[YY] or MM/DD[/YY] or MM.DD[.YY]  MM/DD[/YY]-HH:MM[:SS]  YYYY-MM-DD[THH:MM[:SS]] |  |
**end_time** | Option<**String**> | Filter by end time  Accepted formats:  HH:MM[:SS] [AM|PM]  MMDD[YY] or MM/DD[/YY] or MM.DD[.YY]  MM/DD[/YY]-HH:MM[:SS]  YYYY-MM-DD[THH:MM[:SS]] |  |
**account** | Option<**String**> | Comma delimited list of accounts to match |  |
**association** | Option<**String**> | Comma delimited list of associations to match |  |
**cluster** | Option<**String**> | Comma delimited list of cluster to match |  |
**constraints** | Option<**String**> | Comma delimited list of constraints to match |  |
**cpus_max** | Option<**String**> | Number of CPUs high range |  |
**cpus_min** | Option<**String**> | Number of CPUs low range |  |
**skip_steps** | Option<**bool**> | Report job step information |  |
**disable_wait_for_result** | Option<**bool**> | Disable waiting for result from slurmdbd |  |
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

[**crate::models::Dbv0Period0Period38JobInfo**](dbv0.0.38_job_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_qos

> crate::models::Dbv0Period0Period38QosInfo slurmdb_v0038_get_qos(with_deleted)
Get QOS list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**with_deleted** | Option<**bool**> | Include deleted QOSs. False by default. |  |

### Return type

[**crate::models::Dbv0Period0Period38QosInfo**](dbv0.0.38_qos_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_single_qos

> crate::models::Dbv0Period0Period38QosInfo slurmdb_v0038_get_single_qos(qos_name, with_deleted)
Get QOS info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qos_name** | **String** | Slurm QOS Name | [required] |
**with_deleted** | Option<**bool**> | Include deleted QOSs. False by default. |  |

### Return type

[**crate::models::Dbv0Period0Period38QosInfo**](dbv0.0.38_qos_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_tres

> crate::models::Dbv0Period0Period38TresInfo slurmdb_v0038_get_tres()
Get TRES info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period38TresInfo**](dbv0.0.38_tres_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_user

> crate::models::Dbv0Period0Period38UserInfo slurmdb_v0038_get_user(user_name, with_deleted)
Get user info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_name** | **String** | Slurm User Name | [required] |
**with_deleted** | Option<**bool**> | Include deleted users. False by default. |  |

### Return type

[**crate::models::Dbv0Period0Period38UserInfo**](dbv0.0.38_user_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_users

> crate::models::Dbv0Period0Period38UserInfo slurmdb_v0038_get_users(with_deleted)
Get user list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**with_deleted** | Option<**bool**> | Include deleted users. False by default. |  |

### Return type

[**crate::models::Dbv0Period0Period38UserInfo**](dbv0.0.38_user_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_wckey

> crate::models::Dbv0Period0Period38WckeyInfo slurmdb_v0038_get_wckey(wckey)
Get wckey info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wckey** | **String** | Slurm wckey name | [required] |

### Return type

[**crate::models::Dbv0Period0Period38WckeyInfo**](dbv0.0.38_wckey_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_get_wckeys

> crate::models::Dbv0Period0Period38WckeyInfo slurmdb_v0038_get_wckeys()
Get wckey list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period38WckeyInfo**](dbv0.0.38_wckey_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_set_config

> crate::models::Dbv0Period0Period38ConfigResponse slurmdb_v0038_set_config(dbv0_period0_period38_set_config)
Load all configuration information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period38_set_config** | Option<[**Dbv0Period0Period38SetConfig**](Dbv0Period0Period38SetConfig.md)> | Add or update config |  |

### Return type

[**crate::models::Dbv0Period0Period38ConfigResponse**](dbv0.0.38_config_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_update_account

> crate::models::Dbv0Period0Period38AccountResponse slurmdb_v0038_update_account(dbv0_period0_period38_update_account)
Update accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period38_update_account** | [**Dbv0Period0Period38UpdateAccount**](Dbv0Period0Period38UpdateAccount.md) | update/create accounts | [required] |

### Return type

[**crate::models::Dbv0Period0Period38AccountResponse**](dbv0.0.38_account_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_update_associations

> crate::models::Dbv0Period0Period38ResponseAssociations slurmdb_v0038_update_associations(dbv0_period0_period38_associations_info)
Set associations info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period38_associations_info** | [**Dbv0Period0Period38AssociationsInfo**](Dbv0Period0Period38AssociationsInfo.md) | Add or update associations | [required] |

### Return type

[**crate::models::Dbv0Period0Period38ResponseAssociations**](dbv0.0.38_response_associations.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_update_qos

> crate::models::Dbv0Period0Period38ResponseQos slurmdb_v0038_update_qos(dbv0_period0_period38_update_qos)
Set QOS info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period38_update_qos** | [**Dbv0Period0Period38UpdateQos**](Dbv0Period0Period38UpdateQos.md) | Add or update QOSs | [required] |

### Return type

[**crate::models::Dbv0Period0Period38ResponseQos**](dbv0.0.38_response_qos.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_update_tres

> crate::models::Dbv0Period0Period38ResponseTres slurmdb_v0038_update_tres(dbv0_period0_period38_tres_update)
Set TRES info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period38_tres_update** | [**Dbv0Period0Period38TresUpdate**](Dbv0Period0Period38TresUpdate.md) | Add or Update TRES | [required] |

### Return type

[**crate::models::Dbv0Period0Period38ResponseTres**](dbv0.0.38_response_tres.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdb_v0038_update_users

> crate::models::Dbv0Period0Period38ResponseUserUpdate slurmdb_v0038_update_users(dbv0_period0_period38_update_users)
Update user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dbv0_period0_period38_update_users** | [**Dbv0Period0Period38UpdateUsers**](Dbv0Period0Period38UpdateUsers.md) | add or update user | [required] |

### Return type

[**crate::models::Dbv0Period0Period38ResponseUserUpdate**](dbv0.0.38_response_user_update.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

