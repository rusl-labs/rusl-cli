# \SchemaVersionsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_schema_version_controller_example_data_index**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_example_data_index) | **GET** /api/{account_slug}/schemas/{schema_slug}/example_data | List schema example data
[**rusl_web_api_schema_version_controller_index**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_index) | **GET** /api/{account_slug}/schemas/{schema_slug}/versions | List Schema Versions
[**rusl_web_api_schema_version_controller_lookup**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_lookup) | **GET** /api/schema_versions/lookup | Lookup schema versions by IDs
[**rusl_web_api_schema_version_controller_show**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_show) | **GET** /api/{account_slug}/schemas/{schema_slug}/versions/{version} | Show a Schema Version
[**rusl_web_api_schema_version_controller_update_status**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_update_status) | **PATCH** /api/{account_slug}/schemas/{schema_slug}/versions/{version}/status | Update Schema Version Status
[**rusl_web_api_schema_version_controller_version_examples**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_version_examples) | **GET** /api/{account_slug}/schemas/{schema_slug}/version_examples | List committed example data by version



## rusl_web_api_schema_version_controller_example_data_index

> models::RuslWebApiSchemaVersionControllerExampleDataIndex200Response rusl_web_api_schema_version_controller_example_data_index(account_slug, schema_slug, version, filters, order_by, order_directions, first, after, last, before, limit, offset, page, page_size)
List schema example data

Paginate committed example data for a schema across all committed versions.  Supports: - `version` prefix filtering against canonical schema version ordering (`1`, `1.2`, `1.2.3`) - Flop filters on `version`, `schema_version_id`, `position`, `title`, `inserted_at`, and `updated_at` - Flop ordering on `canonical_version`, `position`, `inserted_at`, and `updated_at` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**version** | Option<**String**> | Version filter - supports prefix matching (e.g., '1' matches 1.*.*, '1.2' matches 1.2.*, '1.2.3' matches exactly) |  |
**filters** | Option<[**std::collections::HashMap<String, models::RuslWebApiSchemaVersionControllerExampleDataIndexFiltersParameterValue>**](Models__RuslWebApiSchemaVersionControllerExampleDataIndexFiltersParameterValue.md)> | Flop filters for example data fields. See https://hexdocs.pm/flop/readme.html#parameter-format |  |
**order_by** | Option<[**Vec<String>**](String.md)> | Fields to order by |  |
**order_directions** | Option<[**Vec<String>**](String.md)> | Order directions |  |
**first** | Option<**i32**> | Cursor pagination: number of items to return from the start. |  |[default to 20]
**after** | Option<**String**> | Cursor pagination: return items after this cursor. |  |
**last** | Option<**i32**> | Cursor pagination: number of items to return from the end. |  |[default to 20]
**before** | Option<**String**> | Cursor pagination: return items before this cursor. |  |
**limit** | Option<**i32**> | Offset pagination: maximum number of items to return. This is the default pagination mode when no pagination params are provided. |  |[default to 20]
**offset** | Option<**i32**> | Offset pagination: zero-based starting offset. |  |[default to 0]
**page** | Option<**i32**> | Page pagination: 1-based page number. |  |[default to 1]
**page_size** | Option<**i32**> | Page pagination: number of items per page. |  |[default to 20]

### Return type

[**models::RuslWebApiSchemaVersionControllerExampleDataIndex200Response**](RuslWeb_Api_SchemaVersionController_example_data_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_version_controller_index

> models::RuslWebApiSchemaVersionControllerIndex200Response rusl_web_api_schema_version_controller_index(account_slug, schema_slug, status, version, first, after, last, before, limit, offset, page, page_size)
List Schema Versions

Paginate schema versions for a schema with optional filtering by status and version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**status** | Option<**String**> | Status filter (DRAFT, ACTIVE, DEPRECATED, or YANKED) |  |
**version** | Option<**String**> | Version filter - supports prefix matching (e.g., '1' matches 1.*.*, '1.2' matches 1.2.*, '1.2.3' matches exactly) |  |
**first** | Option<**i32**> | Cursor pagination: number of items to return from the start. |  |[default to 20]
**after** | Option<**String**> | Cursor pagination: return items after this cursor. |  |
**last** | Option<**i32**> | Cursor pagination: number of items to return from the end. |  |[default to 20]
**before** | Option<**String**> | Cursor pagination: return items before this cursor. |  |
**limit** | Option<**i32**> | Offset pagination: maximum number of items to return. This is the default pagination mode when no pagination params are provided. |  |[default to 20]
**offset** | Option<**i32**> | Offset pagination: zero-based starting offset. |  |[default to 0]
**page** | Option<**i32**> | Page pagination: 1-based page number. |  |[default to 1]
**page_size** | Option<**i32**> | Page pagination: number of items per page. |  |[default to 20]

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


## rusl_web_api_schema_version_controller_version_examples

> models::RuslWebApiSchemaVersionControllerVersionExamples200Response rusl_web_api_schema_version_controller_version_examples(account_slug, schema_slug, versions)
List committed example data by version

Returns committed example data for a schema grouped by exact version. Use the optional versions query parameter to limit results to exact semver versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**versions** | Option<**String**> | Optional comma-separated exact versions (for example, 1.0.0,1.1.0) |  |

### Return type

[**models::RuslWebApiSchemaVersionControllerVersionExamples200Response**](RuslWeb_Api_SchemaVersionController_version_examples_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

