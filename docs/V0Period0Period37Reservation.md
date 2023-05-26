# V0Period0Period37Reservation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounts** | Option<**String**> | Allowed accounts | [optional]
**burst_buffer** | Option<**String**> | Reserved burst buffer | [optional]
**core_count** | Option<**i32**> | Number of reserved cores | [optional]
**core_spec_cnt** | Option<**i32**> | Number of reserved specialized cores | [optional]
**end_time** | Option<**i32**> | End time of the reservation | [optional]
**features** | Option<**String**> | List of features | [optional]
**flags** | Option<**Vec<String>**> | Reservation options | [optional]
**groups** | Option<**String**> | List of groups permitted to use the reserved nodes | [optional]
**licenses** | Option<**String**> | List of licenses | [optional]
**max_start_delay** | Option<**i32**> | Maximum delay in which jobs outside of the reservation will be permitted to overlap once any jobs are queued for the reservation | [optional]
**name** | Option<**String**> | Reservationn name | [optional]
**node_count** | Option<**i32**> | Count of nodes reserved | [optional]
**node_list** | Option<**String**> | List of reserved nodes | [optional]
**partition** | Option<**String**> | Partition | [optional]
**purge_completed** | Option<[**crate::models::V0037ReservationPurgeCompleted**](v0_0_37_reservation_purge_completed.md)> |  | [optional]
**start_time** | Option<**i32**> | Start time of reservation | [optional]
**watts** | Option<**i32**> | amount of power to reserve in watts | [optional]
**tres** | Option<**String**> | List of TRES | [optional]
**users** | Option<**String**> | List of users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


