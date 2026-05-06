# \AccountsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_account_controller_create**](AccountsApi.md#rusl_web_api_account_controller_create) | **POST** /api/accounts | Create an Organization Account
[**rusl_web_api_account_controller_index**](AccountsApi.md#rusl_web_api_account_controller_index) | **GET** /api/accounts | List Accounts
[**rusl_web_api_account_controller_lookup**](AccountsApi.md#rusl_web_api_account_controller_lookup) | **GET** /api/accounts/lookup | Lookup accounts by slugs
[**rusl_web_api_account_controller_show**](AccountsApi.md#rusl_web_api_account_controller_show) | **GET** /api/accounts/{slug} | Show an Account
[**rusl_web_api_account_controller_update**](AccountsApi.md#rusl_web_api_account_controller_update) | **PATCH** /api/accounts/{slug} | Update an Account



## rusl_web_api_account_controller_create

> models::RuslWebApiAccountControllerCreate201Response rusl_web_api_account_controller_create(open_api_schema7)
Create an Organization Account

Create a new organization account. Requires authentication.  The authenticated user becomes the owner of the organization. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**open_api_schema7** | Option<[**OpenApiSchema7**](OpenApiSchema7.md)> | Create Organization Account Request |  |

### Return type

[**models::RuslWebApiAccountControllerCreate201Response**](RuslWeb_Api_AccountController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_controller_index

> models::RuslWebApiAccountControllerIndex200Response rusl_web_api_account_controller_index(filters, order_by, order_directions, first, after, last, before, limit, offset, page, page_size)
List Accounts

List accounts with full Flop pagination and filtering support.  Supports filtering by: - slug (string match) - type (user, organization) - owner_user_id (UUID) - created_order (integer) - mine (boolean - filters to accounts owned by current user, requires authentication)  Supports ordering by: - slug (default) - created_order 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<[**std::collections::HashMap<String, models::RuslWebApiAccountControllerIndexFiltersParameterValue>**](Models__RuslWebApiAccountControllerIndexFiltersParameterValue.md)> | Flop filters. See https://hexdocs.pm/flop/readme.html#parameter-format |  |
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

[**models::RuslWebApiAccountControllerIndex200Response**](RuslWeb_Api_AccountController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_controller_lookup

> models::RuslWebApiAccountControllerLookup200Response rusl_web_api_account_controller_lookup(slugs)
Lookup accounts by slugs

Returns accounts matching the given comma-separated slugs (max 100).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slugs** | **String** | Comma-separated account slugs | [required] |

### Return type

[**models::RuslWebApiAccountControllerLookup200Response**](RuslWeb_Api_AccountController_lookup_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_controller_show

> models::RuslWebApiAccountControllerShow200Response rusl_web_api_account_controller_show(slug)
Show an Account

Get details for a specific account by slug. Public endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Account slug | [required] |

### Return type

[**models::RuslWebApiAccountControllerShow200Response**](RuslWeb_Api_AccountController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_controller_update

> models::RuslWebApiAccountControllerShow200Response rusl_web_api_account_controller_update(slug, open_api_schema3)
Update an Account

Update account details. Requires account manage access (`account.manage` permission).  Updatable fields: - display_name - bio - website 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Account slug | [required] |
**open_api_schema3** | Option<[**OpenApiSchema3**](OpenApiSchema3.md)> | Update Account Request |  |

### Return type

[**models::RuslWebApiAccountControllerShow200Response**](RuslWeb_Api_AccountController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

