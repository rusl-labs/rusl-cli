# SchemaVersion2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: schema_versions) |
**content** | **serde_json::Value** | JSON Schema |
**description** | Option<**String**> | Description of this version | [optional]
**guid** | **String** | Global ID |
**id** | **String** | Schema Version ID |
**inserted_at** | Option<**String**> | Inserted At | [optional]
**published_at** | Option<**String**> | Published At | [optional]
**root_instance_types** | **Vec<String>** | Explicit JSON Schema root instance types declared by content.type |
**schema_format** | Option<**SchemaFormat**> | Schema format (enum: JSON_SCHEMA) | [optional][default to JsonSchema]
**status** | **Status** | Schema Status (enum: DRAFT, ACTIVE, DEPRECATED, YANKED) |
**updated_at** | Option<**String**> | Updated At | [optional]
**version** | **String** | SemVer Version |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
