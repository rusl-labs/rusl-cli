# BundleEntry1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: bundle_entries) | 
**bundle_account_slug** | Option<**String**> | Referenced bundle account slug (BUNDLE only) | [optional]
**bundle_id** | Option<**String**> | Referenced bundle ID (BUNDLE only, denormalized) | [optional]
**bundle_slug** | Option<**String**> | Referenced bundle slug (BUNDLE only) | [optional]
**id** | **String** | Entry ID | 
**inserted_at** | Option<**String**> | Inserted At | [optional]
**kind** | **Kind** | MANAGED for Rusl schemas, UNMANAGED for external URLs, BUNDLE for other bundles (enum: MANAGED, UNMANAGED, BUNDLE) | 
**ref_url** | Option<**String**> | External schema URL (UNMANAGED only) | [optional]
**schema_account_slug** | Option<**String**> | Target schema account slug (MANAGED only) | [optional]
**schema_id** | Option<**String**> | Target schema ID (MANAGED only, denormalized) | [optional]
**schema_slug** | Option<**String**> | Target schema slug (MANAGED only) | [optional]
**updated_at** | Option<**String**> | Updated At | [optional]
**version_requirement** | Option<**String**> | SemVer version requirement (e.g. \"~> 1.0\") | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


