# \SchemaVersionsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_schema_version_controller_index**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_index) | **GET** /api/{account_slug}/schemas/{schema_slug}/versions | List Schema Versions
[**rusl_web_api_schema_version_controller_lookup**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_lookup) | **GET** /api/schema_versions/lookup | Lookup schema versions by IDs
[**rusl_web_api_schema_version_controller_show**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_show) | **GET** /api/{account_slug}/schemas/{schema_slug}/versions/{version} | Show a Schema Version
[**rusl_web_api_schema_version_controller_update_status**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_update_status) | **PATCH** /api/{account_slug}/schemas/{schema_slug}/versions/{version}/status | Update Schema Version Status



## rusl_web_api_schema_version_controller_index

> models::RuslWebApiSchemaVersionControllerIndex200Response rusl_web_api_schema_version_controller_index(account_slug, schema_slug, status, version, stability, first, after)
List Schema Versions

Paginate schema versions for a schema with optional filtering by status and version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**status** | Option<**String**> | Status filter (DRAFT, ACTIVE, DEPRECATED, or YANKED) |  |
**version** | Option<**String**> | Version filter - supports prefix matching (e.g., '1' matches 1.*.*, '1.2' matches 1.2.*, '1.2.3' matches exactly) |  |
**stability** | Option<**String**> | Optional stability filter (experimental, beta, stable, frozen). Accepts comma-separated values. |  |
**first** | Option<**i32**> | Page size |  |
**after** | Option<**String**> | Pagination cursor |  |

### Return type

[**models::RuslWebApiSchemaVersionControllerIndex200Response**](RuslWeb_Api_SchemaVersionController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_version_controller_lookup

> models::RuslWebApiSchemaVersionControllerLookup200Response rusl_web_api_schema_version_controller_lookup(ids)
Lookup schema versions by IDs

Returns schema versions matching the given comma-separated IDs (max 100).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma-separated schema version IDs | [required] |

### Return type

[**models::RuslWebApiSchemaVersionControllerLookup200Response**](RuslWeb_Api_SchemaVersionController_lookup_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_version_controller_show

> models::RuslWebApiSchemaVersionControllerShow200Response rusl_web_api_schema_version_controller_show(version, account_slug, schema_slug)
Show a Schema Version

Get a specific schema version by version string

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | Version string (e.g., '1.2.3') | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiSchemaVersionControllerShow200Response**](RuslWeb_Api_SchemaVersionController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_version_controller_update_status

> models::RuslWebApiSchemaVersionControllerShow200Response rusl_web_api_schema_version_controller_update_status(version, account_slug, schema_slug, rusl_web_api_schema_version_controller_update_status_request)
Update Schema Version Status

Update a schema version status. Allowed statuses are ACTIVE, DEPRECATED, and YANKED.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | Version string (e.g., '1.2.3') | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**rusl_web_api_schema_version_controller_update_status_request** | Option<[**RuslWebApiSchemaVersionControllerUpdateStatusRequest**](RuslWebApiSchemaVersionControllerUpdateStatusRequest.md)> | Schema Version Status Update |  |

### Return type

[**models::RuslWebApiSchemaVersionControllerShow200Response**](RuslWeb_Api_SchemaVersionController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

