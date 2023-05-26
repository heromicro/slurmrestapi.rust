# Dbv0Period0Period38Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | Option<**String**> | Account charged by job | [optional]
**comment** | Option<[**crate::models::Dbv0038JobComment**](dbv0_0_38_job_comment.md)> |  | [optional]
**allocation_nodes** | Option<**String**> | Nodes allocated to job | [optional]
**array** | Option<[**crate::models::Dbv0038JobArray**](dbv0_0_38_job_array.md)> |  | [optional]
**time** | Option<[**crate::models::Dbv0038JobTime**](dbv0_0_38_job_time.md)> |  | [optional]
**association** | Option<[**crate::models::Dbv0Period0Period38AssociationShortInfo**](dbv0.0.38_association_short_info.md)> |  | [optional]
**cluster** | Option<**String**> | Assigned cluster | [optional]
**constraints** | Option<**String**> | Constraints on job | [optional]
**derived_exit_code** | Option<[**crate::models::Dbv0Period0Period38JobExitCode**](dbv0.0.38_job_exit_code.md)> |  | [optional]
**exit_code** | Option<[**crate::models::Dbv0Period0Period38JobExitCode**](dbv0.0.38_job_exit_code.md)> |  | [optional]
**flags** | Option<**Vec<String>**> | List of properties of job | [optional]
**group** | Option<**String**> | User's group to run job | [optional]
**het** | Option<[**crate::models::Dbv0038JobHet**](dbv0_0_38_job_het.md)> |  | [optional]
**job_id** | Option<**i32**> | Job id | [optional]
**name** | Option<**String**> | Assigned job name | [optional]
**mcs** | Option<[**crate::models::Dbv0038JobMcs**](dbv0_0_38_job_mcs.md)> |  | [optional]
**nodes** | Option<**String**> | List of nodes allocated for job | [optional]
**partition** | Option<**String**> | Assigned job's partition | [optional]
**priority** | Option<**i32**> | Priority | [optional]
**qos** | Option<**String**> | Assigned qos name | [optional]
**required** | Option<[**crate::models::Dbv0038JobRequired**](dbv0_0_38_job_required.md)> |  | [optional]
**kill_request_user** | Option<**String**> | User who requested job killed | [optional]
**reservation** | Option<[**crate::models::Dbv0038JobReservation**](dbv0_0_38_job_reservation.md)> |  | [optional]
**state** | Option<[**crate::models::Dbv0038JobState**](dbv0_0_38_job_state.md)> |  | [optional]
**steps** | Option<[**Vec<crate::models::Dbv0Period0Period38JobStep>**](dbv0.0.38_job_step.md)> | Job step description | [optional]
**tres** | Option<[**crate::models::Dbv0038JobTres**](dbv0_0_38_job_tres.md)> |  | [optional]
**user** | Option<**String**> | Job user | [optional]
**wckey** | Option<[**crate::models::Dbv0038JobWckey**](dbv0_0_38_job_wckey.md)> |  | [optional]
**working_directory** | Option<**String**> | Directory where job was initially started | [optional]
**container** | Option<**String**> | absolute path to OCI container bundle | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


