# AccountMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: account_memberships) |
**display_name** | Option<**String**> | User display name | [optional]
**email** | Option<**String**> | User email | [optional]
**id** | **uuid::Uuid** | Membership ID |
**joined_at** | **String** | When the user joined |
**roles** | **Vec<Roles>** |  (enum: OWNER, CONTRIBUTOR) |
**slug** | Option<**String**> | User account slug | [optional]
**user_id** | **uuid::Uuid** | User ID |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
