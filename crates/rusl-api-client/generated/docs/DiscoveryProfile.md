# DiscoveryProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: discovery_profiles) |
**content** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Discovery payload conforming to the rusl/discovery-profile JSON Schema. Null for pending or failed profiles. Canonical schema: https://hassox.rusl-api.ngrok.io/resources/rusl/discovery-profile | [optional]
**generated_at** | Option<**String**> | When the current content was generated | [optional]
**inserted_at** | **String** |  |
**status** | **Status** | Profile generation status (enum: pending, ready, failed) |
**subject_guid** | **String** | GUID of the subject this profile describes |
**subject_type** | **String** | Subject GUID type prefix (e.g. \"schemas\", \"bundles\") |
**updated_at** | **String** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
