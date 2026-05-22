# SchemaSearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_slugs** | Option<**Vec<String>**> | Restrict results to account slugs. | [optional]
**current_version_root_instance_types** | Option<**Vec<CurrentVersionRootInstanceTypes>**> | Restrict results to schemas whose current version declares any root JSON instance type. (enum: array, boolean, integer, null, number, object, string) | [optional]
**current_version_status** | Option<**CurrentVersionStatus**> |  (enum: DRAFT, ACTIVE, DEPRECATED, YANKED) | [optional]
**identifier_prefix** | Option<**String**> | Restrict results to identifiers beginning with this prefix. | [optional]
**identifiers** | Option<**Vec<String>**> | Restrict results to exact schema identifiers. | [optional]
**include** | Option<**Vec<Include>**> | Optional response groups to add to the selected view. (enum: metrics) | [optional]
**page** | Option<**i32**> |  | [optional][default to 1]
**per_page** | Option<**i32**> |  | [optional][default to 20]
**q** | Option<**String**> | Search text. Omit for all visible schemas. | [optional]
**schema_format** | Option<**SchemaFormat**> |  (enum: JSON_SCHEMA) | [optional]
**status** | Option<**Status**> |  (enum: ACTIVE, ARCHIVED) | [optional]
**view** | Option<**View**> | Response shape. full returns projection-owned search data; compact returns summary discovery fields. (enum: full, compact) | [optional][default to Full]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
