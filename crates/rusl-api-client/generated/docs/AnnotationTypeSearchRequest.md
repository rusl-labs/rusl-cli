# AnnotationTypeSearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_slugs** | Option<**Vec<String>**> | Restrict results to account slugs. | [optional]
**cardinality** | Option<**Cardinality**> |  (enum: ONE_PER_SUBJECT_PER_ACCOUNT, MANY_PER_SUBJECT_PER_ACCOUNT) | [optional]
**identifier_prefix** | Option<**String**> | Restrict results to identifiers beginning with this prefix. | [optional]
**identifiers** | Option<**Vec<String>**> | Restrict results to exact annotation type identifiers. | [optional]
**include** | Option<**Vec<Include>**> | Optional response groups to add to the selected view. (enum: metrics) | [optional]
**page** | Option<**i32**> |  | [optional][default to 1]
**per_page** | Option<**i32**> |  | [optional][default to 20]
**q** | Option<**String**> | Search text. Omit for all visible annotation types. | [optional]
**status** | Option<**Status**> |  (enum: ACTIVE, ARCHIVED) | [optional]
**view** | Option<**View**> | Response shape. full returns projection-owned search data; compact returns summary discovery fields. (enum: full, compact) | [optional][default to Full]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
