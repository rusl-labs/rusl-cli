# RuslWebApiEventSubscriptionControllerCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_guid** | Option<**String**> |  | [optional]
**event_prefixes** | Option<**Vec<String>**> | Event name prefixes to filter on | [optional]
**label** | Option<**String**> | Client-defined label for filtering subscriptions | [optional]
**subject_guid** | Option<**String**> | Full GUID (alternative to subject_type + subject_id) | [optional]
**subject_id** | Option<**String**> | Entity ID or slug (use with subject_type) | [optional]
**subject_type** | Option<**SubjectType**> | Subscribable __typename (use with subject_id) (enum: accounts, schemas, schema_proposals) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


