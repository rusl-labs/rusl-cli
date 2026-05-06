# EntitlementDenied1Details

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current** | Option<**i32**> | Current count toward the limit for quota gates, or `null` for non-quota gates. | [optional]
**gate** | **String** | The entitlement gate that denied the request | 
**limit** | Option<[**models::EntitlementDenied1DetailsLimit**](EntitlementDenied1DetailsLimit.md)> |  | [optional]
**plan** | **String** | Plan slug of the account that was checked | 
**reason** | **Reason** | Why the gate denied (enum: feature_disabled, quota_exceeded, rate_limited) | 
**upgrade_to** | **Vec<String>** | Plan slugs the account can upgrade to for this gate | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


