# \BundleEntriesApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_bundle_entry_controller_index**](BundleEntriesApi.md#rusl_web_api_bundle_entry_controller_index) | **GET** /api/{account_slug}/bundles/{bundle_slug}/versions/{version}/entries | List entries for a published bundle version



## rusl_web_api_bundle_entry_controller_index

> models::RuslWebApiBundleEntryControllerIndex200Response rusl_web_api_bundle_entry_controller_index(account_slug, bundle_slug, version)
List entries for a published bundle version

Returns the materialized entries for a published bundle version (read-only).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**version** | **String** | SemVer version | [required] |

### Return type

[**models::RuslWebApiBundleEntryControllerIndex200Response**](RuslWeb_Api_BundleEntryController_index_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
