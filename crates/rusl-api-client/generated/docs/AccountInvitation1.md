# AccountInvitation1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: account_invitations) |
**account_slug** | **String** |  |
**expires_at** | **String** |  |
**id** | **uuid::Uuid** |  |
**inserted_at** | **String** |  |
**roles** | **Vec<Roles>** |  (enum: OWNER, CONTRIBUTOR) |
**status** | **Status** |  (enum: PENDING, ACCEPTED, REJECTED, REVOKED, EXPIRED) |
**target_email** | **String** |  |
**updated_at** | **String** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
