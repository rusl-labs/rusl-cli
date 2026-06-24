# SessionAccount1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: accounts) |
**available_seats** | **i32** | Total number of available seats (included + paid) |
**avatar_asset_id** | Option<**uuid::Uuid**> | Avatar asset ID | [optional]
**billing_subscription_id** | Option<**uuid::Uuid**> | Internal subscription ID | [optional]
**consumed_seats** | **i32** | Number of consumed seats (member count) |
**display_name** | Option<**String**> | Display name | [optional]
**guid** | **String** | Account global ID |
**owner_user_id** | **uuid::Uuid** | Owner user ID |
**permissions** | [**models::SessionAccountPermissions**](SessionAccountPermissions.md) |  |
**plan_slug** | Option<**String**> | Active billing plan slug | [optional]
**roles** | **Vec<Roles>** | User's roles in this account (enum: OWNER, CONTRIBUTOR) |
**slug** | **String** | Account slug |
**stripe_price_id** | Option<**String**> | Stripe price ID of the active subscription | [optional]
**stripe_subscription_id** | Option<**String**> | Stripe subscription ID | [optional]
**r#type** | **Type** | Account type (enum: user, organization) |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
