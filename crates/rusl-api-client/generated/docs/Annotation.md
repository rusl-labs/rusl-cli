# Annotation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: annotations) |
**account_slug** | **String** | Account that owns this annotation |
**annotation_type_id** | **String** | Registered annotation type ID |
**content** | **serde_json::Value** | Annotation content (shape depends on type) |
**guid** | **String** | Global ID |
**id** | **String** | Annotation ID |
**inserted_at** | Option<**String**> | Created at | [optional]
**label** | Option<**String**> | Optional human-readable label | [optional]
**set_by_user_id** | Option<**String**> | User who last created/updated this annotation | [optional]
**status** | **Status** | Annotation lifecycle status (enum: ACTIVE, DEPRECATED, REVOKED) |
**subject_account_slug** | Option<**String**> | Account slug that owns the annotated subject | [optional]
**subject_description** | Option<[**models::AnnotationType1SubjectDescription**](AnnotationType1SubjectDescription.md)> |  | [optional]
**subject_guid** | **String** | GUID of the annotated subject |
**subject_type** | Option<**String**> | Type of the annotated subject (e.g. schemas, bundles) | [optional]
**r#type** | **String** | Registered annotation type identifier |
**type_cardinality** | **TypeCardinality** | Cardinality policy copied from the registered annotation type for database enforcement (enum: ONE_PER_SUBJECT_PER_ACCOUNT, MANY_PER_SUBJECT_PER_ACCOUNT) |
**updated_at** | Option<**String**> | Updated at | [optional]
**validated_at_version** | Option<**String**> | Schema version content was validated against | [optional]
**validation_schema_identifier** | Option<**String**> | Schema identifier used to validate content at write time | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
