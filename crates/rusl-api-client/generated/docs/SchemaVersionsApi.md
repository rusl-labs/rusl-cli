# \SchemaVersionsApi

All URIs are relative to *https://resources.rusl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_schema_version_controller_example_data_index**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_example_data_index) | **GET** /api/{account_slug}/schemas/{schema_slug}/example_data | List schema example data
[**rusl_web_api_schema_version_controller_example_data_index_0**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_example_data_index_0) | **GET** /api/{account_slug}/schemas/{schema_slug}/example_data | List schema example data
[**rusl_web_api_schema_version_controller_show**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_show) | **GET** /api/{account_slug}/schemas/{schema_slug}/versions/{version} | Show a Schema Version
[**rusl_web_api_schema_version_controller_show_0**](SchemaVersionsApi.md#rusl_web_api_schema_version_controller_show_0) | **GET** /api/{account_slug}/schemas/{schema_slug}/versions/{version} | Show a Schema Version



## rusl_web_api_schema_version_controller_example_data_index

> models::RuslWebApiSchemaVersionControllerExampleDataIndex200Response rusl_web_api_schema_version_controller_example_data_index(account_slug, schema_slug, version, filters, order_by, order_directions, first, after, last, before, limit, offset, page, page_size)
List schema example data

Paginate committed example data for a schema across all committed versions.  Supports: - `version` prefix filtering against canonical schema version ordering (`1`, `1.2`, `1.2.3`) - Flop filters on `version`, `schema_version_id`, `position`, `title`, `inserted_at`, and `updated_at` - Flop ordering on `canonical_version`, `position`, `inserted_at`, and `updated_at`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |
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


## rusl_web_api_schema_version_controller_example_data_index_0

> models::RuslWebApiSchemaVersionControllerExampleDataIndex200Response rusl_web_api_schema_version_controller_example_data_index_0(account_slug, schema_slug, version, filters, order_by, order_directions, first, after, last, before, limit, offset, page, page_size)
List schema example data

Paginate committed example data for a schema across all committed versions.  Supports: - `version` prefix filtering against canonical schema version ordering (`1`, `1.2`, `1.2.3`) - Flop filters on `version`, `schema_version_id`, `position`, `title`, `inserted_at`, and `updated_at` - Flop ordering on `canonical_version`, `position`, `inserted_at`, and `updated_at`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |
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


## rusl_web_api_schema_version_controller_show

> models::RuslWebApiSchemaVersionControllerShow200Response rusl_web_api_schema_version_controller_show(version, account_slug, schema_slug)
Show a Schema Version

Get a specific schema version by version string

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | Version string (e.g., '1.2.3') | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |

### Return type

[**models::RuslWebApiSchemaVersionControllerShow200Response**](RuslWeb_Api_SchemaVersionController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_version_controller_show_0

> models::RuslWebApiSchemaVersionControllerShow200Response rusl_web_api_schema_version_controller_show_0(version, account_slug, schema_slug)
Show a Schema Version

Get a specific schema version by version string

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | Version string (e.g., '1.2.3') | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |

### Return type

[**models::RuslWebApiSchemaVersionControllerShow200Response**](RuslWeb_Api_SchemaVersionController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
