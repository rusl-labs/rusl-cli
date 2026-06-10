# \RawBundlesApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_raw_bundle_controller_show**](RawBundlesApi.md#rusl_web_raw_bundle_controller_show) | **GET** /resources/{account_slug}/bundles/{bundle_slug_and_version} | Serve raw bundle manifest content
[**rusl_web_raw_bundle_metadata_controller_show**](RawBundlesApi.md#rusl_web_raw_bundle_metadata_controller_show) | **GET** /resources/{account_slug}/bundles/{bundle_slug}/metadata | Bundle resolution metadata index



## rusl_web_raw_bundle_controller_show

> String rusl_web_raw_bundle_controller_show(account_slug, bundle_slug_and_version, disposition)
Serve raw bundle manifest content

Serves the raw bundle manifest at its canonical `/resources/{account_slug}/bundles/{bundle_slug}` URL. Supports versioned access via `@v1.0.0` suffix for pinned, immutable content.  Caching behavior (via `RuslWeb.RawCacheHeaders`): - Public bundles: CDN-cacheable. Pinned versions (`@vX.Y.Z`) return `public, max-age=31536000, immutable` + ETag.   Clients and CDNs can use `If-None-Match` to receive 304 Not Modified responses. - Latest (no version suffix): shorter TTL (configurable via `Rusl.Caching.latest_max_age`). - Private bundles: always `private, no-store`. Never cached by CDNs or shared caches. - All public responses include `cache-tag` headers so the CDN can be purged on publish or status changes.  Private data is never cacheable — membership is re-checked on every request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug_and_version** | **String** | Bundle slug, optionally with pinned version (e.g. `my-bundle` or `my-bundle@v1.0.0`) | [required] |
**disposition** | Option<**String**> | Optional response disposition. Omit or use `inline` to inspect the raw content; use `attachment` to force a file download. |  |

### Return type

**String**

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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
