# RuslWebApiAnnotationControllerCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | **serde_json::Value** |  |
**label** | Option<**String**> |  | [optional]
**subject_description** | Option<[**models::RuslWebApiAnnotationControllerCreateRequestSubjectDescription**](RuslWebApiAnnotationControllerCreateRequestSubjectDescription.md)> |  | [optional]
**subject_guid** | **String** |  |
**r#type** | **String** | Registered annotation type identifier |
**visibility** | Option<**Visibility**> | Requested visibility. Defaults to the annotation type's visibility. PRIVATE on a public subject requires the private_annotations_on_public entitlement and a type that does not lock visibility. (enum: PUBLIC, PRIVATE) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
