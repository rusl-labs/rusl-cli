# Bundle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: bundles) | 
**account_slug** | **String** | Account Slug | 
**archived_at** | Option<**String**> | Archived At | [optional]
**bundle_identifier** | **String** | Full identifier in account_slug/bundles/slug form | 
**current_version** | Option<[**models::BundleVersion1**](BundleVersion1.md)> |  | [optional]
**description** | Option<**String**> | Bundle Description | [optional]
**guid** | Option<**String**> | Global ID | [optional]
**id** | **String** | Bundle ID | 
**inserted_at** | **String** | Inserted At | 
**slug** | **String** | Bundle Slug | 
**status** | **Status** | Bundle Lifecycle Status (enum: ACTIVE, ARCHIVED) | [default to Active]
**subject_description** | Option<[**models::Annotation1SubjectDescription**](Annotation1SubjectDescription.md)> |  | [optional]
**updated_at** | **String** | Updated At | 
**visibility** | Option<**Visibility**> | Bundle Visibility (enum: PUBLIC, PRIVATE) | [optional][default to Public]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


