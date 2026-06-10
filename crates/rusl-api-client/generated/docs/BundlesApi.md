# \BundlesApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_bundle_controller_archive**](BundlesApi.md#rusl_web_api_bundle_controller_archive) | **POST** /api/{account_slug}/bundles/{bundle_slug}/archive | Archive a bundle
[**rusl_web_api_bundle_controller_create**](BundlesApi.md#rusl_web_api_bundle_controller_create) | **POST** /api/{account_slug}/bundles | Create a new bundle
[**rusl_web_api_bundle_controller_create_provenance**](BundlesApi.md#rusl_web_api_bundle_controller_create_provenance) | **POST** /api/{account_slug}/bundles/{bundle_slug}/provenance | Create bundle provenance
[**rusl_web_api_bundle_controller_fork**](BundlesApi.md#rusl_web_api_bundle_controller_fork) | **POST** /api/{account_slug}/bundles/forks | Fork a bundle
[**rusl_web_api_bundle_controller_index**](BundlesApi.md#rusl_web_api_bundle_controller_index) | **GET** /api/bundles | Search bundles across accounts
[**rusl_web_api_bundle_controller_lookup**](BundlesApi.md#rusl_web_api_bundle_controller_lookup) | **GET** /api/bundles/lookup | Lookup bundles by IDs
[**rusl_web_api_bundle_controller_paginate**](BundlesApi.md#rusl_web_api_bundle_controller_paginate) | **POST** /api/{account_slug}/bundles/filter | Paginate bundles
[**rusl_web_api_bundle_controller_show**](BundlesApi.md#rusl_web_api_bundle_controller_show) | **GET** /api/{account_slug}/bundles/{bundle_slug} | Fetch a bundle
[**rusl_web_api_bundle_controller_unarchive**](BundlesApi.md#rusl_web_api_bundle_controller_unarchive) | **POST** /api/{account_slug}/bundles/{bundle_slug}/unarchive | Unarchive a bundle
[**rusl_web_api_bundle_controller_update**](BundlesApi.md#rusl_web_api_bundle_controller_update) | **PATCH** /api/{account_slug}/bundles/{bundle_slug} | Update a bundle



## rusl_web_api_bundle_controller_archive

> models::RuslWebApiBundleControllerCreate201Response rusl_web_api_bundle_controller_archive(account_slug, bundle_slug)
Archive a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |

### Return type

[**models::RuslWebApiBundleControllerCreate201Response**](RuslWeb_Api_BundleController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_controller_create

> models::RuslWebApiBundleControllerCreate201Response rusl_web_api_bundle_controller_create(account_slug, rusl_web_api_bundle_controller_create_request)
Create a new bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**rusl_web_api_bundle_controller_create_request** | Option<[**RuslWebApiBundleControllerCreateRequest**](RuslWebApiBundleControllerCreateRequest.md)> | Create Bundle |  |

### Return type

[**models::RuslWebApiBundleControllerCreate201Response**](RuslWeb_Api_BundleController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_controller_create_provenance

> models::RuslWebApiBundleControllerCreateProvenance201Response rusl_web_api_bundle_controller_create_provenance(account_slug, bundle_slug, create_resource_origin_request1)
Create bundle provenance

Create ADAPTED_FROM or IMPORTED_FROM provenance for a bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**create_resource_origin_request1** | Option<[**CreateResourceOriginRequest1**](CreateResourceOriginRequest1.md)> | Create Bundle Provenance |  |

### Return type

[**models::RuslWebApiBundleControllerCreateProvenance201Response**](RuslWeb_Api_BundleController_create_provenance_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_controller_fork

> models::RuslWebApiBundleControllerCreate201Response rusl_web_api_bundle_controller_fork(account_slug, fork_bundle_request1)
Fork a bundle

Create a new bundle by copying one readable immutable source bundle version. Upstream history is not copied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**fork_bundle_request1** | Option<[**ForkBundleRequest1**](ForkBundleRequest1.md)> | Fork Bundle |  |

### Return type

[**models::RuslWebApiBundleControllerCreate201Response**](RuslWeb_Api_BundleController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_controller_index

> models::RuslWebApiBundleControllerIndex200Response rusl_web_api_bundle_controller_index(filters, order_by, order_directions, first, after, last, before, limit, offset, page, page_size)
Search bundles across accounts

Search bundles across all accounts with Flop pagination and filtering support.  Supports filtering by: - q (text search across account slug, bundle slug, bundle identifier, and description) - identifier (canonical bundle identifier, supports exact and in filters) - bundle_identifier (resource-specific storage field) - account_slug - slug - visibility - status  Results are scoped by user permissions. Anonymous users see only PUBLIC bundles, authenticated users also see PRIVATE bundles from accounts they can access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<[**std::collections::HashMap<String, models::RuslWebApiBundleControllerIndexFiltersParameterValue>**](Models__RuslWebApiBundleControllerIndexFiltersParameterValue.md)> | Flop filters. Supports q text search via field=q and op=ilike_or. See https://hexdocs.pm/flop/readme.html#parameter-format |  |
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

[**models::RuslWebApiBundleControllerIndex200Response**](RuslWeb_Api_BundleController_index_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_controller_lookup

> models::RuslWebApiBundleControllerLookup200Response rusl_web_api_bundle_controller_lookup(ids)
Lookup bundles by IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma-separated bundle IDs | [required] |

### Return type

[**models::RuslWebApiBundleControllerLookup200Response**](RuslWeb_Api_BundleController_lookup_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_controller_paginate

> models::RuslWebApiBundleControllerIndex200Response rusl_web_api_bundle_controller_paginate(account_slug, rusl_web_api_bundle_controller_paginate_request)
Paginate bundles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**rusl_web_api_bundle_controller_paginate_request** | Option<[**RuslWebApiBundleControllerPaginateRequest**](RuslWebApiBundleControllerPaginateRequest.md)> | Pagination Input |  |

### Return type

[**models::RuslWebApiBundleControllerIndex200Response**](RuslWeb_Api_BundleController_index_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_controller_show

> models::RuslWebApiBundleControllerCreate201Response rusl_web_api_bundle_controller_show(account_slug, bundle_slug)
Fetch a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |

### Return type

[**models::RuslWebApiBundleControllerCreate201Response**](RuslWeb_Api_BundleController_create_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_controller_unarchive

> models::RuslWebApiBundleControllerCreate201Response rusl_web_api_bundle_controller_unarchive(account_slug, bundle_slug)
Unarchive a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |

### Return type

[**models::RuslWebApiBundleControllerCreate201Response**](RuslWeb_Api_BundleController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_controller_update

> models::RuslWebApiBundleControllerCreate201Response rusl_web_api_bundle_controller_update(account_slug, bundle_slug, rusl_web_api_bundle_controller_update_request)
Update a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**rusl_web_api_bundle_controller_update_request** | Option<[**RuslWebApiBundleControllerUpdateRequest**](RuslWebApiBundleControllerUpdateRequest.md)> | Update Bundle |  |

### Return type

[**models::RuslWebApiBundleControllerCreate201Response**](RuslWeb_Api_BundleController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
