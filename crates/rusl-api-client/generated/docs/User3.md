# User3

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: users) |
**confirmed_at** | Option<**String**> | Confirmed At | [optional]
**email** | Option<**String**> | User Email | [optional]
**guid** | **String** | Global ID |
**id** | **String** | User ID |
**inserted_at** | **String** | Inserted At |
**slug** | **String** | The user's account slug — set when the user account is created |
**updated_at** | **String** | Updated At |
**user_account** | Option<[**models::UserAccount2**](UserAccount2.md)> |  | [optional]
**user_type** | **UserType** | Self-reported principal type. Defaults to \"human\". (enum: agent, human, unknown) |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
