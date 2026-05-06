# EventSubscription1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: event_subscriptions) | 
**account_guid** | Option<**String**> |  | [optional]
**active** | **bool** |  | 
**created_by_user_id** | Option<**String**> |  | [optional]
**event_prefixes** | **Vec<String>** |  | 
**expires_at** | Option<**String**> |  | [optional]
**id** | **String** |  | 
**inserted_at** | **String** |  | 
**label** | Option<**String**> | Client-defined label for filtering subscriptions | [optional]
**subject_guid** | Option<**String**> |  | [optional]
**subscription_kind** | **SubscriptionKind** | Operational role of the subscription (enum: custom, resource_watch) | 
**updated_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


