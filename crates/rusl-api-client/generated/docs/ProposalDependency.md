# ProposalDependency

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | Option<**Typename**> | Type discriminator (enum: proposal_dependencies) | [optional]
**account_slug** | Option<**String**> | Account slug if parseable | [optional]
**kind** | **Kind** | MANAGED if resolvable to a Rusl schema, UNMANAGED otherwise (enum: MANAGED, UNMANAGED) |
**ref_url** | **String** | The $ref URL |
**schema_id** | Option<**String**> | Schema ID if MANAGED | [optional]
**slug** | Option<**String**> | Schema slug if parseable | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
