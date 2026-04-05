# Asset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: assets) | 
**created_by_user_id** | Option<**uuid::Uuid**> |  | [optional]
**created_order** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**file_size** | Option<**i32**> |  | [optional]
**guid** | **String** | Global ID | 
**id** | **uuid::Uuid** | Asset ID | 
**inserted_at** | **String** |  | 
**metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**mime_type** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**owner_guid** | Option<**String**> |  | [optional]
**public_url** | Option<**String**> | Resolved URL for fetching the asset | [optional]
**scope** | **Scope** | Access scope (enum: public, private) | 
**status** | **Status** | Lifecycle state (enum: pending, ready, error) | 
**storage_key** | Option<**String**> | S3 storage path | [optional]
**r#type** | **Type** | Asset type (enum: managed, reference) | 
**updated_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


