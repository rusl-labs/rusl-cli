# BundleVersion1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: bundle_versions) |
**approved_by_user_id** | Option<**String**> | User who published the version | [optional]
**created_by_user_id** | Option<**String**> | User who created the draft | [optional]
**description** | Option<**String**> | Release notes for this version | [optional]
**guid** | **String** | Global ID |
**id** | **String** | Bundle Version ID |
**inserted_at** | Option<**String**> | Inserted At | [optional]
**manifest** | **String** | Raw manifest content (JSON or TOML) |
**manifest_format** | **ManifestFormat** | Format of the manifest content (enum: JSON, TOML) |
**published_at** | Option<**String**> | Published At | [optional]
**status** | **Status** | Version Status (enum: DRAFT, ACTIVE, DEPRECATED, YANKED) |
**updated_at** | Option<**String**> | Updated At | [optional]
**version** | **String** | SemVer Version |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
