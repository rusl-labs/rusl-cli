# \BundleVersionsApi

All URIs are relative to *https://resources.rusl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_bundle_version_controller_show**](BundleVersionsApi.md#rusl_web_api_bundle_version_controller_show) | **GET** /api/{account_slug}/bundles/{bundle_slug}/versions/{version} | Fetch a bundle version
[**rusl_web_api_bundle_version_controller_show_0**](BundleVersionsApi.md#rusl_web_api_bundle_version_controller_show_0) | **GET** /api/{account_slug}/bundles/{bundle_slug}/versions/{version} | Fetch a bundle version



## rusl_web_api_bundle_version_controller_show

> models::RuslWebApiBundleVersionControllerShow200Response rusl_web_api_bundle_version_controller_show(account_slug, bundle_slug, version)
Fetch a bundle version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**version** | **String** | SemVer version | [required] |

### Return type

[**models::RuslWebApiBundleVersionControllerShow200Response**](RuslWeb_Api_BundleVersionController_show_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_version_controller_show_0

> models::RuslWebApiBundleVersionControllerShow200Response rusl_web_api_bundle_version_controller_show_0(account_slug, bundle_slug, version)
Fetch a bundle version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**version** | **String** | SemVer version | [required] |

### Return type

[**models::RuslWebApiBundleVersionControllerShow200Response**](RuslWeb_Api_BundleVersionController_show_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
