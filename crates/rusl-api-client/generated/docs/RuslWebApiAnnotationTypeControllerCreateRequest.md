# RuslWebApiAnnotationTypeControllerCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_immutable** | Option<**bool**> |  | [optional][default to false]
**description** | Option<**String**> | Annotation type description | [optional]
**pinned_schema_version_id** | Option<**String**> |  | [optional]
**schema_identifier** | **String** | Validation schema identifier | 
**schema_mode** | **SchemaMode** |  (enum: CURRENT, PINNED) | 
**slug** | **String** | Annotation type slug | 
**subject_description** | Option<[**models::OpenApiSchema2SubjectDescription**](OpenApiSchema2SubjectDescription.md)> |  | [optional]
**visibility** | Option<**Visibility**> |  (enum: PUBLIC, PRIVATE) | [optional][default to Public]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


