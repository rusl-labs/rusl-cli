# MeResponseAuthenticated

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounts** | [**std::collections::HashMap<String, models::SessionAccount1>**](SessionAccount1.md) | Map of accounts keyed by slug |
**authenticated** | **bool** |  |
**authentication_type** | **AuthenticationType** |  (enum: jwt, api_key) |
**invitations** | [**Vec<models::AccountInvitation1>**](AccountInvitation1.md) | Pending invitations for the current user |
**service_account** | Option<[**models::MeResponseAuthenticated1ServiceAccount**](MeResponseAuthenticated1ServiceAccount.md)> |  | [optional]
**user** | [**models::User1**](User1.md) |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
