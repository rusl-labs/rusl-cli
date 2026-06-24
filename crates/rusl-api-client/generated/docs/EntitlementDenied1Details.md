# EntitlementDenied1Details

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current** | Option<**i32**> | Current count toward the limit for quota gates, or `null` for non-quota gates. | [optional]
**gate** | **String** | The entitlement gate that denied the request |
**limit** | Option<**i32**> | The limit value for a quota gate. `-1` means unlimited; non-negative integers are the cap. `null` for non-quota gates (feature / rate limit). | [optional]
**plan** | **String** | Plan slug of the account that was checked |
**reason** | **Reason** | Why the gate denied (enum: feature_disabled, quota_exceeded, rate_limited) |
**upgrade_to** | **Vec<String>** | Plan slugs the account can upgrade to for this gate |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
