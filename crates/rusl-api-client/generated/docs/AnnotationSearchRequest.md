# AnnotationSearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_slugs** | Option<**Vec<String>**> | Restrict results to annotating account slugs. | [optional]
**annotation_type_guids** | Option<**Vec<String>**> | Restrict results to registered annotation type GUIDs. | [optional]
**include** | Option<**Vec<Include>**> | Optional response groups to add to the selected view. (enum: metrics) | [optional]
**page** | Option<**i32**> |  | [optional][default to 1]
**per_page** | Option<**i32**> |  | [optional][default to 20]
**q** | Option<**String**> | Search text. Omit for all visible annotations. | [optional]
**set_by_user_guids** | Option<**Vec<String>**> | Restrict results to annotations made by these user GUIDs. | [optional]
**sort** | Option<**Sort**> | Sort by query relevance or endorsement count. (enum: relevance, endorsements) | [optional][default to Relevance]
**status** | Option<**Status**> |  (enum: ACTIVE, DEPRECATED, REVOKED) | [optional]
**subject_account_slugs** | Option<**Vec<String>**> | Restrict results by the annotated subject account slug. | [optional]
**subject_guids** | Option<**Vec<String>**> | Restrict results to annotations attached to these subject GUIDs. | [optional]
**subject_identifier_prefix** | Option<**String**> | Restrict results to annotated subject identifiers beginning with this prefix. | [optional]
**subject_types** | Option<**Vec<SubjectTypes>**> | Restrict results by annotated subject type. (enum: annotations, bundle_versions, bundles, schema_proposals, schema_versions, schemas) | [optional]
**type_identifiers** | Option<**Vec<String>**> | Restrict results to registered annotation type identifiers. | [optional]
**view** | Option<**View**> | Response shape. full returns projection-owned search data including annotation content; compact excludes content and large folded text. (enum: full, compact) | [optional][default to Full]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
