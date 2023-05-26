# \SlurmApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**slurmctld_cancel_job**](SlurmApi.md#slurmctld_cancel_job) | **DELETE** /slurm/v0.0.37/job/{job_id} | cancel or signal job
[**slurmctld_diag**](SlurmApi.md#slurmctld_diag) | **GET** /slurm/v0.0.37/diag | get diagnostics
[**slurmctld_get_job**](SlurmApi.md#slurmctld_get_job) | **GET** /slurm/v0.0.37/job/{job_id} | get job info
[**slurmctld_get_jobs**](SlurmApi.md#slurmctld_get_jobs) | **GET** /slurm/v0.0.37/jobs | get list of jobs
[**slurmctld_get_node**](SlurmApi.md#slurmctld_get_node) | **GET** /slurm/v0.0.37/node/{node_name} | get node info
[**slurmctld_get_nodes**](SlurmApi.md#slurmctld_get_nodes) | **GET** /slurm/v0.0.37/nodes | get all node info
[**slurmctld_get_partition**](SlurmApi.md#slurmctld_get_partition) | **GET** /slurm/v0.0.37/partition/{partition_name} | get partition info
[**slurmctld_get_partitions**](SlurmApi.md#slurmctld_get_partitions) | **GET** /slurm/v0.0.37/partitions | get all partition info
[**slurmctld_get_reservation**](SlurmApi.md#slurmctld_get_reservation) | **GET** /slurm/v0.0.37/reservation/{reservation_name} | get reservation info
[**slurmctld_get_reservations**](SlurmApi.md#slurmctld_get_reservations) | **GET** /slurm/v0.0.37/reservations | get all reservation info
[**slurmctld_ping**](SlurmApi.md#slurmctld_ping) | **GET** /slurm/v0.0.37/ping | ping test
[**slurmctld_submit_job**](SlurmApi.md#slurmctld_submit_job) | **POST** /slurm/v0.0.37/job/submit | submit new job
[**slurmctld_update_job**](SlurmApi.md#slurmctld_update_job) | **POST** /slurm/v0.0.37/job/{job_id} | update job
[**slurmdbd_add_clusters**](SlurmApi.md#slurmdbd_add_clusters) | **POST** /slurmdb/v0.0.37/clusters | Add clusters
[**slurmdbd_add_wckeys**](SlurmApi.md#slurmdbd_add_wckeys) | **POST** /slurmdb/v0.0.37/wckeys | Add wckeys
[**slurmdbd_delete_account**](SlurmApi.md#slurmdbd_delete_account) | **DELETE** /slurmdb/v0.0.37/account/{account_name} | Delete account
[**slurmdbd_delete_association**](SlurmApi.md#slurmdbd_delete_association) | **DELETE** /slurmdb/v0.0.37/association | Delete association
[**slurmdbd_delete_cluster**](SlurmApi.md#slurmdbd_delete_cluster) | **DELETE** /slurmdb/v0.0.37/cluster/{cluster_name} | Delete cluster
[**slurmdbd_delete_qos**](SlurmApi.md#slurmdbd_delete_qos) | **DELETE** /slurmdb/v0.0.37/qos/{qos_name} | Delete QOS
[**slurmdbd_delete_user**](SlurmApi.md#slurmdbd_delete_user) | **DELETE** /slurmdb/v0.0.37/user/{user_name} | Delete user
[**slurmdbd_delete_wckey**](SlurmApi.md#slurmdbd_delete_wckey) | **DELETE** /slurmdb/v0.0.37/wckey/{wckey} | Delete wckey
[**slurmdbd_diag**](SlurmApi.md#slurmdbd_diag) | **GET** /slurmdb/v0.0.37/diag | Get slurmdb diagnostics
[**slurmdbd_get_account**](SlurmApi.md#slurmdbd_get_account) | **GET** /slurmdb/v0.0.37/account/{account_name} | Get account info
[**slurmdbd_get_accounts**](SlurmApi.md#slurmdbd_get_accounts) | **GET** /slurmdb/v0.0.37/accounts | Get account list
[**slurmdbd_get_association**](SlurmApi.md#slurmdbd_get_association) | **GET** /slurmdb/v0.0.37/association | Get association info
[**slurmdbd_get_associations**](SlurmApi.md#slurmdbd_get_associations) | **GET** /slurmdb/v0.0.37/associations | Get association list
[**slurmdbd_get_cluster**](SlurmApi.md#slurmdbd_get_cluster) | **GET** /slurmdb/v0.0.37/cluster/{cluster_name} | Get cluster info
[**slurmdbd_get_clusters**](SlurmApi.md#slurmdbd_get_clusters) | **GET** /slurmdb/v0.0.37/clusters | Get cluster list
[**slurmdbd_get_db_config**](SlurmApi.md#slurmdbd_get_db_config) | **GET** /slurmdb/v0.0.37/config | Dump all configuration information
[**slurmdbd_get_job**](SlurmApi.md#slurmdbd_get_job) | **GET** /slurmdb/v0.0.37/job/{job_id} | Get job info
[**slurmdbd_get_jobs**](SlurmApi.md#slurmdbd_get_jobs) | **GET** /slurmdb/v0.0.37/jobs | Get job list
[**slurmdbd_get_qos**](SlurmApi.md#slurmdbd_get_qos) | **GET** /slurmdb/v0.0.37/qos | Get QOS list
[**slurmdbd_get_single_qos**](SlurmApi.md#slurmdbd_get_single_qos) | **GET** /slurmdb/v0.0.37/qos/{qos_name} | Get QOS info
[**slurmdbd_get_tres**](SlurmApi.md#slurmdbd_get_tres) | **GET** /slurmdb/v0.0.37/tres | Get TRES info
[**slurmdbd_get_user**](SlurmApi.md#slurmdbd_get_user) | **GET** /slurmdb/v0.0.37/user/{user_name} | Get user info
[**slurmdbd_get_users**](SlurmApi.md#slurmdbd_get_users) | **GET** /slurmdb/v0.0.37/users | Get user list
[**slurmdbd_get_wckey**](SlurmApi.md#slurmdbd_get_wckey) | **GET** /slurmdb/v0.0.37/wckey/{wckey} | Get wckey info
[**slurmdbd_get_wckeys**](SlurmApi.md#slurmdbd_get_wckeys) | **GET** /slurmdb/v0.0.37/wckeys | Get wckey list
[**slurmdbd_set_db_config**](SlurmApi.md#slurmdbd_set_db_config) | **POST** /slurmdb/v0.0.37/config | Load all configuration information
[**slurmdbd_update_account**](SlurmApi.md#slurmdbd_update_account) | **POST** /slurmdb/v0.0.37/accounts | Update accounts
[**slurmdbd_update_associations**](SlurmApi.md#slurmdbd_update_associations) | **POST** /slurmdb/v0.0.37/associations | Set associations info
[**slurmdbd_update_tres**](SlurmApi.md#slurmdbd_update_tres) | **POST** /slurmdb/v0.0.37/tres | Set TRES info
[**slurmdbd_update_users**](SlurmApi.md#slurmdbd_update_users) | **POST** /slurmdb/v0.0.37/users | Update user



## slurmctld_cancel_job

> slurmctld_cancel_job(job_id, signal)
cancel or signal job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Slurm Job ID | [required] |
**signal** | Option<[**V0Period0Period37Signal**](.md)> | signal to send to job |  |

### Return type

 (empty response body)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_diag

> crate::models::V0Period0Period37Diag slurmctld_diag()
get diagnostics

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V0Period0Period37Diag**](v0.0.37_diag.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_get_job

> crate::models::V0Period0Period37JobsResponse slurmctld_get_job(job_id)
get job info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Slurm JobID | [required] |

### Return type

[**crate::models::V0Period0Period37JobsResponse**](v0.0.37_jobs_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_get_jobs

> crate::models::V0Period0Period37JobsResponse slurmctld_get_jobs(update_time)
get list of jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period37JobsResponse**](v0.0.37_jobs_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_get_node

> crate::models::V0Period0Period37NodesResponse slurmctld_get_node(node_name)
get node info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_name** | **String** | Slurm Node Name | [required] |

### Return type

[**crate::models::V0Period0Period37NodesResponse**](v0.0.37_nodes_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_get_nodes

> crate::models::V0Period0Period37NodesResponse slurmctld_get_nodes(update_time)
get all node info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period37NodesResponse**](v0.0.37_nodes_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_get_partition

> crate::models::V0Period0Period37PartitionsResponse slurmctld_get_partition(partition_name, update_time)
get partition info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition_name** | **String** | Slurm Partition Name | [required] |
**update_time** | Option<**i64**> | Filter if there were no partition changes (not limited to partition in URL endpoint) since update_time. |  |

### Return type

[**crate::models::V0Period0Period37PartitionsResponse**](v0.0.37_partitions_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_get_partitions

> crate::models::V0Period0Period37PartitionsResponse slurmctld_get_partitions(update_time)
get all partition info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period37PartitionsResponse**](v0.0.37_partitions_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_get_reservation

> crate::models::V0Period0Period37ReservationsResponse slurmctld_get_reservation(reservation_name, update_time)
get reservation info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reservation_name** | **String** | Slurm Reservation Name | [required] |
**update_time** | Option<**i64**> | Filter if no reservation (not limited to reservation in URL) changed since update_time. |  |

### Return type

[**crate::models::V0Period0Period37ReservationsResponse**](v0.0.37_reservations_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_get_reservations

> crate::models::V0Period0Period37ReservationsResponse slurmctld_get_reservations(update_time)
get all reservation info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_time** | Option<**i64**> | Filter if changed since update_time. Use of this parameter can result in faster replies. |  |

### Return type

[**crate::models::V0Period0Period37ReservationsResponse**](v0.0.37_reservations_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_ping

> crate::models::V0Period0Period37Pings slurmctld_ping()
ping test

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V0Period0Period37Pings**](v0.0.37_pings.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_submit_job

> crate::models::V0Period0Period37JobSubmissionResponse slurmctld_submit_job(v0_period0_period37_job_submission)
submit new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v0_period0_period37_job_submission** | [**V0Period0Period37JobSubmission**](V0Period0Period37JobSubmission.md) | submit new job | [required] |

### Return type

[**crate::models::V0Period0Period37JobSubmissionResponse**](v0.0.37_job_submission_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmctld_update_job

> slurmctld_update_job(job_id, v0_period0_period37_job_properties)
update job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Slurm Job ID | [required] |
**v0_period0_period37_job_properties** | [**V0Period0Period37JobProperties**](V0Period0Period37JobProperties.md) | update job | [required] |

### Return type

 (empty response body)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json, application/x-yaml
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_add_clusters

> crate::models::Dbv0Period0Period37ResponseClusterAdd slurmdbd_add_clusters()
Add clusters

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37ResponseClusterAdd**](dbv0.0.37_response_cluster_add.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_add_wckeys

> crate::models::Dbv0Period0Period37ResponseWckeyAdd slurmdbd_add_wckeys()
Add wckeys

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37ResponseWckeyAdd**](dbv0.0.37_response_wckey_add.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_delete_account

> crate::models::Dbv0Period0Period37ResponseAccountDelete slurmdbd_delete_account(account_name)
Delete account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** | Slurm Account Name | [required] |

### Return type

[**crate::models::Dbv0Period0Period37ResponseAccountDelete**](dbv0.0.37_response_account_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_delete_association

> crate::models::Dbv0Period0Period37ResponseAssociationDelete slurmdbd_delete_association(account, user, cluster, partition)
Delete association

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account** | **String** | Account name | [required] |
**user** | **String** | User name | [required] |
**cluster** | Option<**String**> | Cluster name |  |
**partition** | Option<**String**> | Partition Name |  |

### Return type

[**crate::models::Dbv0Period0Period37ResponseAssociationDelete**](dbv0.0.37_response_association_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_delete_cluster

> crate::models::Dbv0Period0Period37ResponseClusterDelete slurmdbd_delete_cluster(cluster_name)
Delete cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** | Slurm cluster name | [required] |

### Return type

[**crate::models::Dbv0Period0Period37ResponseClusterDelete**](dbv0.0.37_response_cluster_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_delete_qos

> crate::models::Dbv0Period0Period37ResponseQosDelete slurmdbd_delete_qos(qos_name)
Delete QOS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qos_name** | **String** | Slurm QOS Name | [required] |

### Return type

[**crate::models::Dbv0Period0Period37ResponseQosDelete**](dbv0.0.37_response_qos_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_delete_user

> crate::models::Dbv0Period0Period37ResponseUserDelete slurmdbd_delete_user(user_name)
Delete user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_name** | **String** | Slurm User Name | [required] |

### Return type

[**crate::models::Dbv0Period0Period37ResponseUserDelete**](dbv0.0.37_response_user_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_delete_wckey

> crate::models::Dbv0Period0Period37ResponseWckeyDelete slurmdbd_delete_wckey(wckey)
Delete wckey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wckey** | **String** | Slurm wckey name | [required] |

### Return type

[**crate::models::Dbv0Period0Period37ResponseWckeyDelete**](dbv0.0.37_response_wckey_delete.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_diag

> crate::models::Dbv0Period0Period37Diag slurmdbd_diag()
Get slurmdb diagnostics

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37Diag**](dbv0.0.37_diag.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_account

> crate::models::Dbv0Period0Period37AccountInfo slurmdbd_get_account(account_name)
Get account info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** | Slurm Account Name | [required] |

### Return type

[**crate::models::Dbv0Period0Period37AccountInfo**](dbv0.0.37_account_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_accounts

> crate::models::Dbv0Period0Period37AccountInfo slurmdbd_get_accounts()
Get account list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37AccountInfo**](dbv0.0.37_account_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_association

> crate::models::Dbv0Period0Period37AssociationsInfo slurmdbd_get_association(cluster, account, user, partition)
Get association info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | Option<**String**> | Cluster name |  |
**account** | Option<**String**> | Account name |  |
**user** | Option<**String**> | User name |  |
**partition** | Option<**String**> | Partition Name |  |

### Return type

[**crate::models::Dbv0Period0Period37AssociationsInfo**](dbv0.0.37_associations_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_associations

> crate::models::Dbv0Period0Period37AssociationsInfo slurmdbd_get_associations()
Get association list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37AssociationsInfo**](dbv0.0.37_associations_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_cluster

> crate::models::Dbv0Period0Period37ClusterInfo slurmdbd_get_cluster(cluster_name)
Get cluster info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** | Slurm cluster name | [required] |

### Return type

[**crate::models::Dbv0Period0Period37ClusterInfo**](dbv0.0.37_cluster_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_clusters

> crate::models::Dbv0Period0Period37ClusterInfo slurmdbd_get_clusters()
Get cluster list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37ClusterInfo**](dbv0.0.37_cluster_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_db_config

> crate::models::Dbv0Period0Period37ConfigInfo slurmdbd_get_db_config()
Dump all configuration information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37ConfigInfo**](dbv0.0.37_config_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_job

> crate::models::Dbv0Period0Period37JobInfo slurmdbd_get_job(job_id)
Get job info

This endpoint may return multiple job entries since job_id is not a unique key - only the tuple (cluster, job_id, start_time) is unique. If the requested job_id is a component of a heterogeneous job all components are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **i64** | Slurm Job ID | [required] |

### Return type

[**crate::models::Dbv0Period0Period37JobInfo**](dbv0.0.37_job_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_jobs

> crate::models::Dbv0Period0Period37JobInfo slurmdbd_get_jobs(submit_time, start_time, end_time, account, association, cluster, constraints, cpus_max, cpus_min, skip_steps, disable_wait_for_result, exit_code, format, group, job_name, nodes_max, nodes_min, partition, qos, reason, reservation, state, step, node, wckey)
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

[**crate::models::Dbv0Period0Period37JobInfo**](dbv0.0.37_job_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_qos

> crate::models::Dbv0Period0Period37QosInfo slurmdbd_get_qos()
Get QOS list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37QosInfo**](dbv0.0.37_qos_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_single_qos

> crate::models::Dbv0Period0Period37QosInfo slurmdbd_get_single_qos(qos_name)
Get QOS info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qos_name** | **String** | Slurm QOS Name | [required] |

### Return type

[**crate::models::Dbv0Period0Period37QosInfo**](dbv0.0.37_qos_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_tres

> crate::models::Dbv0Period0Period37TresInfo slurmdbd_get_tres()
Get TRES info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37TresInfo**](dbv0.0.37_tres_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_user

> crate::models::Dbv0Period0Period37UserInfo slurmdbd_get_user(user_name)
Get user info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_name** | **String** | Slurm User Name | [required] |

### Return type

[**crate::models::Dbv0Period0Period37UserInfo**](dbv0.0.37_user_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_users

> crate::models::Dbv0Period0Period37UserInfo slurmdbd_get_users()
Get user list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37UserInfo**](dbv0.0.37_user_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_wckey

> crate::models::Dbv0Period0Period37WckeyInfo slurmdbd_get_wckey(wckey)
Get wckey info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wckey** | **String** | Slurm wckey name | [required] |

### Return type

[**crate::models::Dbv0Period0Period37WckeyInfo**](dbv0.0.37_wckey_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_get_wckeys

> crate::models::Dbv0Period0Period37WckeyInfo slurmdbd_get_wckeys()
Get wckey list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37WckeyInfo**](dbv0.0.37_wckey_info.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_set_db_config

> crate::models::Dbv0Period0Period37ConfigResponse slurmdbd_set_db_config()
Load all configuration information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37ConfigResponse**](dbv0.0.37_config_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_update_account

> crate::models::Dbv0Period0Period37AccountResponse slurmdbd_update_account()
Update accounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37AccountResponse**](dbv0.0.37_account_response.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_update_associations

> crate::models::Dbv0Period0Period37ResponseAssociations slurmdbd_update_associations()
Set associations info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37ResponseAssociations**](dbv0.0.37_response_associations.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_update_tres

> crate::models::Dbv0Period0Period37ResponseTres slurmdbd_update_tres()
Set TRES info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37ResponseTres**](dbv0.0.37_response_tres.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slurmdbd_update_users

> crate::models::Dbv0Period0Period37ResponseUserUpdate slurmdbd_update_users()
Update user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Dbv0Period0Period37ResponseUserUpdate**](dbv0.0.37_response_user_update.md)

### Authorization

[user](../README.md#user), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

