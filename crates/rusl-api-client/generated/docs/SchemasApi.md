# \SchemasApi

All URIs are relative to *https://resources.rusl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_schema_controller_create**](SchemasApi.md#rusl_web_api_schema_controller_create) | **POST** /api/{account_slug}/schemas | Create a new schema
[**rusl_web_api_schema_controller_create_0**](SchemasApi.md#rusl_web_api_schema_controller_create_0) | **POST** /api/{account_slug}/schemas | Create a new schema
[**rusl_web_api_schema_controller_show**](SchemasApi.md#rusl_web_api_schema_controller_show) | **GET** /api/{account_slug}/schemas/{schema_slug} | Fetch a schema
[**rusl_web_api_schema_controller_show_0**](SchemasApi.md#rusl_web_api_schema_controller_show_0) | **GET** /api/{account_slug}/schemas/{schema_slug} | Fetch a schema



## rusl_web_api_schema_controller_create

> models::RuslWebApiSchemaControllerCreate201Response rusl_web_api_schema_controller_create(account_slug, open_api_schema1)
Create a new schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**open_api_schema1** | Option<[**OpenApiSchema1**](OpenApiSchema1.md)> | Create Schema Request |  |

### Return type

[**models::RuslWebApiSchemaControllerCreate201Response**](RuslWeb_Api_SchemaController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_create_0

> models::RuslWebApiSchemaControllerCreate201Response rusl_web_api_schema_controller_create_0(account_slug, open_api_schema1)
Create a new schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**open_api_schema1** | Option<[**OpenApiSchema1**](OpenApiSchema1.md)> | Create Schema Request |  |

### Return type

[**models::RuslWebApiSchemaControllerCreate201Response**](RuslWeb_Api_SchemaController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_show

> models::RuslWebApiSchemaControllerCreate201Response rusl_web_api_schema_controller_show(account_slug, schema_slug)
Fetch a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |

### Return type

[**models::RuslWebApiSchemaControllerCreate201Response**](RuslWeb_Api_SchemaController_create_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_show_0

> models::RuslWebApiSchemaControllerCreate201Response rusl_web_api_schema_controller_show_0(account_slug, schema_slug)
Fetch a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |

### Return type

[**models::RuslWebApiSchemaControllerCreate201Response**](RuslWeb_Api_SchemaController_create_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
