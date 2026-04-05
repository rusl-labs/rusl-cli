# \RawSchemasApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_raw_schema_controller_show**](RawSchemasApi.md#rusl_web_raw_schema_controller_show) | **GET** /schemas/{account_slug}/{schema_slug_and_version} | Serve raw JSON schema content
[**rusl_web_raw_schema_metadata_controller_show**](RawSchemasApi.md#rusl_web_raw_schema_metadata_controller_show) | **GET** /schemas/{account_slug}/{schema_slug}/metadata | Schema resolution metadata index



## rusl_web_raw_schema_controller_show

> serde_json::Value rusl_web_raw_schema_controller_show(account_slug, schema_slug_and_version)
Serve raw JSON schema content

Serves the raw JSON schema content at the schema's `$id` URL. Supports versioned access via `@v0.2.3` suffix for pinned, immutable content.  - Public schemas: no authentication required, CDN-cacheable - Private schemas: requires authenticated user with account membership - Pinned versions (`@vX.Y.Z`): immutable, long-lived cache - Latest (no version suffix): short-lived cache, busted on version changes 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug_and_version** | **String** | Schema slug, optionally with pinned version (e.g. `us-address` or `us-address@v1.2.3`) | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_raw_schema_metadata_controller_show

> models::RuslWebRawSchemaMetadataControllerShow200Response rusl_web_raw_schema_metadata_controller_show(account_slug, schema_slug)
Schema resolution metadata index

Returns every resolvable version of a schema and its dependency constraints in a single payload, enabling the PubGrub resolver to evaluate the full dependency graph without additional network round-trips.  Includes ACTIVE and DEPRECATED versions. DRAFT and YANKED versions are excluded. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebRawSchemaMetadataControllerShow200Response**](RuslWeb_RawSchemaMetadataController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

