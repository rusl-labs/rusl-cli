# \BundlesApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_bundle_controller_archive**](BundlesApi.md#rusl_web_api_bundle_controller_archive) | **POST** /api/{account_slug}/bundles/{bundle_slug}/archive | Archive a bundle
[**rusl_web_api_bundle_controller_create**](BundlesApi.md#rusl_web_api_bundle_controller_create) | **POST** /api/{account_slug}/bundles | Create a new bundle
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

> models::RuslWebApiBundleControllerPaginate200Response rusl_web_api_bundle_controller_paginate(account_slug, rusl_web_api_bundle_controller_paginate_request)
Paginate bundles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**rusl_web_api_bundle_controller_paginate_request** | Option<[**RuslWebApiBundleControllerPaginateRequest**](RuslWebApiBundleControllerPaginateRequest.md)> | Pagination Input |  |

### Return type

[**models::RuslWebApiBundleControllerPaginate200Response**](RuslWeb_Api_BundleController_paginate_200_response.md)

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

