# Schema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: schemas) | 
**account_slug** | **String** | Account Slug | 
**archived_at** | Option<**String**> | Archived At | [optional]
**current_version** | Option<[**models::SchemaVersion1**](SchemaVersion1.md)> |  | [optional]
**description** | Option<**String**> | Schema Description | [optional]
**guid** | Option<**String**> | Global ID | [optional]
**id** | **String** | Schema ID | 
**inserted_at** | **String** | Inserted At | 
**schema_format** | **SchemaFormat** | Schema format (enum: JSON_SCHEMA) | [default to JsonSchema]
**slug** | **String** | Schema Slug | 
**status** | **Status** | Schema Lifecycle Status (enum: ACTIVE, ARCHIVED) | [default to Active]
**updated_at** | **String** | Updated At | 
**visibility** | Option<**Visibility**> | Schema Visibility (enum: PUBLIC, PRIVATE) | [optional][default to Public]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


