# SessionAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: accounts) | 
**avatar_asset_id** | Option<**uuid::Uuid**> | Avatar asset ID | [optional]
**display_name** | Option<**String**> | Display name | [optional]
**guid** | **String** | Account global ID | 
**owner_user_id** | **uuid::Uuid** | Owner user ID | 
**permissions** | [**models::SessionAccount1Permissions**](SessionAccount1Permissions.md) |  | 
**roles** | **Vec<Roles>** | User's roles in this account (enum: OWNER, CONTRIBUTOR) | 
**slug** | **String** | Account slug | 
**r#type** | **Type** | Account type (enum: user, organization) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


