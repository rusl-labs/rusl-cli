# \SchemasApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_schema_controller_archive**](SchemasApi.md#rusl_web_api_schema_controller_archive) | **POST** /api/{account_slug}/schemas/{schema_slug}/archive | Archive a schema
[**rusl_web_api_schema_controller_create**](SchemasApi.md#rusl_web_api_schema_controller_create) | **POST** /api/{account_slug}/schemas | Create a new schema
[**rusl_web_api_schema_controller_index**](SchemasApi.md#rusl_web_api_schema_controller_index) | **GET** /api/schemas | Search schemas across accounts
[**rusl_web_api_schema_controller_lookup**](SchemasApi.md#rusl_web_api_schema_controller_lookup) | **GET** /api/schemas/lookup | Lookup schemas by IDs
[**rusl_web_api_schema_controller_paginate**](SchemasApi.md#rusl_web_api_schema_controller_paginate) | **POST** /api/{account_slug}/schemas/filter | Paginate schemas
[**rusl_web_api_schema_controller_schema_identifier_typeahead**](SchemasApi.md#rusl_web_api_schema_controller_schema_identifier_typeahead) | **GET** /api/schemas/schema_identifier_typeahead | Typeahead for schema identifiers
[**rusl_web_api_schema_controller_show**](SchemasApi.md#rusl_web_api_schema_controller_show) | **GET** /api/{account_slug}/schemas/{schema_slug} | Fetch a schema
[**rusl_web_api_schema_controller_unarchive**](SchemasApi.md#rusl_web_api_schema_controller_unarchive) | **POST** /api/{account_slug}/schemas/{schema_slug}/unarchive | Unarchive a schema
[**rusl_web_api_schema_controller_update**](SchemasApi.md#rusl_web_api_schema_controller_update) | **PATCH** /api/{account_slug}/schemas/{schema_slug} | Update a schema



## rusl_web_api_schema_controller_archive

> models::RuslWebApiSchemaControllerShow200Response rusl_web_api_schema_controller_archive(account_slug, schema_slug)
Archive a schema

Archive a schema. Archived schemas are excluded from default discovery queries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiSchemaControllerShow200Response**](RuslWeb_Api_SchemaController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_create

> models::RuslWebApiSchemaControllerShow200Response rusl_web_api_schema_controller_create(account_slug, open_api_schema5)
Create a new schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**open_api_schema5** | Option<[**OpenApiSchema5**](OpenApiSchema5.md)> | Create Schema Request |  |

### Return type

[**models::RuslWebApiSchemaControllerShow200Response**](RuslWeb_Api_SchemaController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_index

> models::RuslWebApiSchemaControllerIndex200Response rusl_web_api_schema_controller_index(filters, order_by, order_directions, first, after, last, before, limit, offset, page, page_size)
Search schemas across accounts

Search schemas across all accounts with full Flop pagination and filtering support.  Supports filtering by: - q (text search across account slug, schema slug, schema identifier, and description) - identifier (canonical schema identifier, supports exact and in filters) - schema_identifier (resource-specific storage field) - account_slug (string match) - slug (string match) - visibility (PUBLIC, PRIVATE) - schema_format (JSON_SCHEMA)  Results are scoped by user permissions - anonymous users see only PUBLIC schemas, authenticated users see PUBLIC schemas plus PRIVATE schemas from accounts they belong to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<[**std::collections::HashMap<String, models::RuslWebApiSchemaControllerIndexFiltersParameterValue>**](Models__RuslWebApiSchemaControllerIndexFiltersParameterValue.md)> | Flop filters. Supports q text search via field=q and op=ilike_or. See https://hexdocs.pm/flop/readme.html#parameter-format |  |
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

[**models::RuslWebApiSchemaControllerIndex200Response**](RuslWeb_Api_SchemaController_index_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_lookup

> models::RuslWebApiSchemaControllerLookup200Response rusl_web_api_schema_controller_lookup(ids)
Lookup schemas by IDs

Returns schemas matching the given comma-separated IDs (max 100).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma-separated schema IDs | [required] |

### Return type

[**models::RuslWebApiSchemaControllerLookup200Response**](RuslWeb_Api_SchemaController_lookup_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_paginate

> models::RuslWebApiSchemaControllerIndex200Response rusl_web_api_schema_controller_paginate(account_slug, rusl_web_api_schema_controller_paginate_request)
Paginate schemas

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**rusl_web_api_schema_controller_paginate_request** | Option<[**RuslWebApiSchemaControllerPaginateRequest**](RuslWebApiSchemaControllerPaginateRequest.md)> | Pagination Input |  |

### Return type

[**models::RuslWebApiSchemaControllerIndex200Response**](RuslWeb_Api_SchemaController_index_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_schema_identifier_typeahead

> models::RuslWebApiSchemaControllerSchemaIdentifierTypeahead200Response rusl_web_api_schema_controller_schema_identifier_typeahead(q, limit)
Typeahead for schema identifiers

Returns a small set of visible active schema identifier suggestions for a typeahead query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Schema identifier prefix to match |  |
**limit** | Option<**i32**> | Optional result limit. Defaults to 5 and is capped at 10. |  |

### Return type

[**models::RuslWebApiSchemaControllerSchemaIdentifierTypeahead200Response**](RuslWeb_Api_SchemaController_schema_identifier_typeahead_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_show

> models::RuslWebApiSchemaControllerShow200Response rusl_web_api_schema_controller_show(account_slug, schema_slug)
Fetch a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiSchemaControllerShow200Response**](RuslWeb_Api_SchemaController_show_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_unarchive

> models::RuslWebApiSchemaControllerShow200Response rusl_web_api_schema_controller_unarchive(account_slug, schema_slug)
Unarchive a schema

Restore an archived schema to ACTIVE status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiSchemaControllerShow200Response**](RuslWeb_Api_SchemaController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_update

> models::RuslWebApiSchemaControllerShow200Response rusl_web_api_schema_controller_update(account_slug, schema_slug, open_api_schema2)
Update a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**open_api_schema2** | Option<[**OpenApiSchema2**](OpenApiSchema2.md)> | Update Schema Request |  |

### Return type

[**models::RuslWebApiSchemaControllerShow200Response**](RuslWeb_Api_SchemaController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
