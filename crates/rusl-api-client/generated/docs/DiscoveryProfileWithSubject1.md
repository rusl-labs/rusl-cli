# DiscoveryProfileWithSubject1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: discovery_profile_with_subject) |
**metrics** | **std::collections::HashMap<String, i32>** | Runtime discoverability counts for the subject. Keys vary by subject type — schemas carry `favourite_count`, `watcher_count`, `endorsement_count`, `attached_annotation_count`, `dependent_schema_count`, `bundle_inclusion_count`, and `validation_schema_usage_count`; bundles carry the universal counts plus `bundle_inclusion_count`. Empty `{}` means no counts have been recorded yet. Hydrated live on every read from the `discoverability_snapshots` view, not frozen into the generated profile content. See docs/features/discoverability.md for the metric registry. |
**profile** | [**models::DiscoveryProfile**](DiscoveryProfile.md) |  |
**subject** | Option<[**models::DiscoveryProfileWithSubject1Subject**](DiscoveryProfileWithSubject1Subject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
