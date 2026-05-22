# Notification1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: notifications) |
**content** | [**models::Notification1Content**](Notification1Content.md) |  |
**event** | Option<[**models::NotificationEvent1**](NotificationEvent1.md)> |  | [optional]
**event_id** | **uuid::Uuid** |  |
**group_key** | Option<**String**> | Grouping key for collapsing related notifications | [optional]
**id** | **uuid::Uuid** |  |
**inserted_at** | **String** |  |
**subject_guid** | Option<**String**> |  | [optional]
**subject_type** | Option<**String**> | Entity type prefix derived from subject_guid | [optional]
**title** | **String** | Human-readable notification title |
**updated_at** | **String** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
