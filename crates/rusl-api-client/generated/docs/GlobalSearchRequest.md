# GlobalSearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_slugs** | Option<**Vec<String>**> | Restrict results to account slugs. | [optional]
**discovery_profile_status** | Option<**DiscoveryProfileStatus**> | Restrict results by discovery profile status. (enum: pending, ready, failed) | [optional]
**include** | Option<**Vec<Include>**> | Optional response groups to add to the selected view. (enum: metrics) | [optional]
**metric_names** | Option<**Vec<String>**> | Restrict results to resources with any of these metric names. | [optional]
**page** | Option<**i32**> |  | [optional][default to 1]
**per_page** | Option<**i32**> |  | [optional][default to 20]
**q** | Option<**String**> | Search text. Omit for all visible results. | [optional]
**types** | Option<**Vec<Types>**> | Restrict global search to specific result types. (enum: schema, annotation_type) | [optional]
**view** | Option<**View**> | Response shape. full returns projection-owned search data; compact returns summary discovery fields. (enum: full, compact) | [optional][default to Full]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


