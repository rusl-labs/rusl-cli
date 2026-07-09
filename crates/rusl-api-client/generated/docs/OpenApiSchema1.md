# OpenApiSchema1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description | [optional]
**package** | Option<**String**> | Optional dotted package path this schema lives under (omit when the schema is not packaged). Each segment is 3-25 characters of [a-z0-9_-], at most 5 segments deep, lowercased on write. Slug uniqueness is per-package. | [optional]
**schema_format** | Option<**SchemaFormat**> | Schema format (enum: JSON_SCHEMA) | [optional][default to JsonSchema]
**slug** | **String** | Leaf schema slug (must not contain dots) |
**subject_description** | Option<[**models::RuslWebApiAnnotationControllerCreateRequestSubjectDescription**](RuslWebApiAnnotationControllerCreateRequestSubjectDescription.md)> |  | [optional]
**visibility** | Option<**Visibility**> | Visibility (enum: PUBLIC, PRIVATE) | [optional][default to Public]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
