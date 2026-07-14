# \RawBundlesApi

All URIs are relative to *https://resources.rusl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_raw_bundle_metadata_controller_show**](RawBundlesApi.md#rusl_web_raw_bundle_metadata_controller_show) | **GET** /resources/{account_slug}/bundles/{bundle_slug}/metadata | Bundle resolution metadata index
[**rusl_web_raw_bundle_metadata_controller_show_0**](RawBundlesApi.md#rusl_web_raw_bundle_metadata_controller_show_0) | **GET** /resources/{account_slug}/bundles/{bundle_slug}/metadata | Bundle resolution metadata index



## rusl_web_raw_bundle_metadata_controller_show

> models::RuslWebRawBundleMetadataControllerShow200Response rusl_web_raw_bundle_metadata_controller_show(account_slug, bundle_slug)
Bundle resolution metadata index

Returns every resolvable version of a bundle and its dependency constraints in a single payload, enabling the PubGrub resolver to evaluate the full dependency graph without additional network round-trips.  Canonical metadata lives under `/resources/{account_slug}/bundles/{bundle_slug}/metadata`.  Caching behavior (via `RuslWeb.RawCacheHeaders` + `Rusl.Caching`): - Public metadata: cacheable with a configurable TTL (`Rusl.Caching.metadata_max_age`).   Kept fresh via event-driven CDN purges on version publish or status changes (uses `cache-tag`). - Private metadata: always `private, no-store`. Never cached. - Version suffixes (`@vX.Y.Z`) are ignored for lookup and caching — the response reflects the current resolvable set.  Private data is never cacheable — membership is re-checked on every request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |

### Return type

[**models::RuslWebRawBundleMetadataControllerShow200Response**](RuslWeb_RawBundleMetadataController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_raw_bundle_metadata_controller_show_0

> models::RuslWebRawBundleMetadataControllerShow200Response rusl_web_raw_bundle_metadata_controller_show_0(account_slug, bundle_slug)
Bundle resolution metadata index

Returns every resolvable version of a bundle and its dependency constraints in a single payload, enabling the PubGrub resolver to evaluate the full dependency graph without additional network round-trips.  Canonical metadata lives under `/resources/{account_slug}/bundles/{bundle_slug}/metadata`.  Caching behavior (via `RuslWeb.RawCacheHeaders` + `Rusl.Caching`): - Public metadata: cacheable with a configurable TTL (`Rusl.Caching.metadata_max_age`).   Kept fresh via event-driven CDN purges on version publish or status changes (uses `cache-tag`). - Private metadata: always `private, no-store`. Never cached. - Version suffixes (`@vX.Y.Z`) are ignored for lookup and caching — the response reflects the current resolvable set.  Private data is never cacheable — membership is re-checked on every request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |

### Return type

[**models::RuslWebRawBundleMetadataControllerShow200Response**](RuslWeb_RawBundleMetadataController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
