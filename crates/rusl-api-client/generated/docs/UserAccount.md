# UserAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: accounts) | 
**avatar_asset_id** | Option<**uuid::Uuid**> | ID of the account's avatar asset | [optional]
**bio** | Option<**String**> | Account Bio | [optional]
**display_name** | Option<**String**> |  | [optional]
**guid** | **String** |  | 
**inserted_at** | **String** | Inserted At | 
**member_count** | Option<**i32**> | Number of members in the account | [optional]
**owner_user** | Option<[**models::Account1OwnerUser**](Account1OwnerUser.md)> |  | [optional]
**owner_user_id** | **String** | Owner User ID | 
**plan** | **String** | The plan slug that gates this account's capabilities (entitlements / quotas / rate limits). Defaults to \"free\". | 
**slug** | **String** | A URL safe and unique identifier for the account | 
**team_visibility** | Option<**TeamVisibility**> | Whether the account's team members are publicly visible (enum: public, private) | [optional]
**r#type** | **Type** |  (enum: user) | 
**updated_at** | **String** | Updated At | 
**website** | Option<**String**> | Account Website | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


