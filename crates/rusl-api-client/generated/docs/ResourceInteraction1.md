# ResourceInteraction1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: resource_interactions) |
**id** | **uuid::Uuid** |  |
**inserted_at** | **String** |  |
**interaction_type** | **InteractionType** |  (enum: favourite, watch, endorse) |
**subject_guid** | **String** | GUID of the interacted-with resource |
**subject_type** | Option<**String**> | Auto-populated type prefix from the GUID | [optional]
**updated_at** | **String** |  |
**user_id** | **uuid::Uuid** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
