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
**identifier** | **String** | Full opaque identifier in account_slug/schemas/leaf form; packaged schemas fold the package path into the final segment (account_slug/schemas/package.path.leaf). Route by this; never rebuild it from parts. |
**inserted_at** | **String** | Inserted At |
**package_path** | Option<**String**> | Dotted package path this schema lives under, or null when the schema is not packaged. Segments never contain dots. | [optional]
**package_segments** | **Vec<String>** | Server-split package path segments for breadcrumbs. Empty when the schema is not packaged. Clients never split the compound themselves. |
**resource_origins** | Option<[**Vec<models::ResourceOrigin1>**](ResourceOrigin1.md)> | Durable provenance edges for this schema | [optional]
**schema_format** | **SchemaFormat** | Schema format (enum: JSON_SCHEMA) | [default to JsonSchema]
**slug** | **String** | Leaf schema slug (never contains dots) |
**status** | **Status** | Schema Lifecycle Status (enum: ACTIVE, ARCHIVED) | [default to Active]
**subject_description** | Option<[**models::AnnotationType1SubjectDescription**](AnnotationType1SubjectDescription.md)> |  | [optional]
**updated_at** | **String** | Updated At |
**visibility** | Option<**Visibility**> | Schema Visibility (enum: PUBLIC, PRIVATE) | [optional][default to Public]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
