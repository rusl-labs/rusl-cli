# VersionDependency

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | Option<**Typename**> | Type discriminator (enum: version_dependencies) | [optional]
**account_slug** | Option<**String**> | Target schema account slug | [optional]
**dependent_account_slug** | Option<**String**> | Source schema account slug | [optional]
**dependent_schema_id** | Option<**String**> | Source schema ID (the schema whose version has the $ref) | [optional]
**dependent_slug** | Option<**String**> | Source schema slug | [optional]
**id** | Option<**String**> | Dependency ID | [optional]
**kind** | **Kind** | MANAGED if resolvable to a Rusl schema, UNMANAGED otherwise (enum: MANAGED, UNMANAGED) | 
**ref_url** | **String** | The $ref URL | 
**schema_id** | Option<**String**> | Target schema ID (depended-on) if MANAGED | [optional]
**schema_version_id** | Option<**String**> | Schema Version ID | [optional]
**slug** | Option<**String**> | Target schema slug | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


