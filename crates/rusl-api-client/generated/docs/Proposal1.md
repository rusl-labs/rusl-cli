# Proposal1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: schema_proposals) |
**accepted_at** | Option<**String**> | Accepted At | [optional]
**accepted_by_user_id** | Option<**String**> | Closed By User ID | [optional]
**based_on_version** | Option<**String**> | Schema version this proposal was based on | [optional]
**closed_at** | Option<**String**> | Closed At | [optional]
**closed_by_user_id** | Option<**String**> | Closed By User ID | [optional]
**closed_reason** | Option<**String**> | Closed Reason | [optional]
**content** | **serde_json::Value** | JSON Schema proposal |
**dependencies** | Option<[**Vec<models::ProposalDependency1>**](ProposalDependency1.md)> | Resolved schema dependencies extracted from $ref URLs | [optional]
**description** | Option<**String**> | Description | [optional]
**guid** | **String** | Global ID |
**id** | **String** | Schema ID |
**inserted_at** | **String** | Inserted At |
**minimum_bump_type** | Option<**MinimumBumpType**> | Minimum version bump type required (enum: major, minor, patch) | [optional]
**outdated** | Option<**bool**> | Whether this proposal is outdated | [optional][default to false]
**proposal_number** | Option<**i32**> | Proposal number | [optional]
**proposed_by_user_id** | Option<**String**> | Proposed By User ID | [optional]
**proposed_version** | Option<**String**> | Proposed version | [optional]
**root_instance_types** | **Vec<String>** | Explicit JSON Schema root instance types declared by content.type |
**schema_format** | Option<**SchemaFormat**> | Schema format (enum: JSON_SCHEMA) | [optional][default to JsonSchema]
**schema_id** | **String** | Schema ID |
**schema_version_id** | Option<**String**> | Schema version created from this proposal | [optional]
**status** | **Status** | Proposal Status (enum: PENDING, ACCEPTED, REJECTED, CLOSED) | [default to Pending]
**updated_at** | **String** | Updated At |
**valid_data** | [**Vec<models::ExampleData1>**](ExampleData1.md) | Valid Data (may be empty for schemas that are not object- or array-rooted) |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
