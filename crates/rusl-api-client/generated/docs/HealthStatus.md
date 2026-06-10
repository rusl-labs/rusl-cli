# HealthStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**checks** | Option<[**std::collections::HashMap<String, models::HealthStatusChecksValue>**](HealthStatusChecksValue.md)> | Per-component health results (keyed by stable identifier). | [optional]
**generated_at** | Option<**String**> |  | [optional]
**status** | **Status** | Aggregate status. :initializing means the first probes have not completed yet. (enum: initializing, healthy, degraded, unhealthy) |
**uptime_seconds** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
