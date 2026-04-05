# \RawBundlesApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_raw_bundle_controller_show**](RawBundlesApi.md#rusl_web_raw_bundle_controller_show) | **GET** /bundles/{account_slug}/{bundle_slug_and_version} | Serve raw bundle manifest content
[**rusl_web_raw_bundle_metadata_controller_show**](RawBundlesApi.md#rusl_web_raw_bundle_metadata_controller_show) | **GET** /bundles/{account_slug}/{bundle_slug}/metadata | Bundle resolution metadata index



## rusl_web_raw_bundle_controller_show

> String rusl_web_raw_bundle_controller_show(account_slug, bundle_slug_and_version)
Serve raw bundle manifest content

Serves the raw bundle manifest at its canonical URL. Supports versioned access via `@v1.0.0` suffix for pinned, immutable content.  - Public bundles: no authentication required, CDN-cacheable - Private bundles: requires authenticated user with account membership - Pinned versions (`@vX.Y.Z`): immutable, long-lived cache - Latest (no version suffix): short-lived cache, busted on version changes 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug_and_version** | **String** | Bundle slug, optionally with pinned version (e.g. `my-bundle` or `my-bundle@v1.0.0`) | [required] |

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

Returns every resolvable version of a bundle and its dependency constraints in a single payload, enabling the PubGrub resolver to evaluate the full dependency graph without additional network round-trips.  Includes ACTIVE and DEPRECATED versions. DRAFT and YANKED versions are excluded. 

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

