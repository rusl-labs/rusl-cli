# \DiscoveryApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_discovery_controller_search**](DiscoveryApi.md#rusl_web_api_discovery_controller_search) | **GET** /api/discovery/search | Search discovery profiles
[**rusl_web_api_discovery_controller_show**](DiscoveryApi.md#rusl_web_api_discovery_controller_show) | **GET** /api/discovery/profiles/{subject_guid} | Fetch a discovery profile by subject GUID



## rusl_web_api_discovery_controller_search

> models::RuslWebApiDiscoveryControllerSearch200Response rusl_web_api_discovery_controller_search(q, subject_type, status, order_by, first, after, last, before, limit, offset, page, page_size)
Search discovery profiles

Full-text search across discovery profiles (generated summaries, keywords, and facets for schemas and bundles). Results include the hydrated subject so agents can reason about the profile and the entity it describes in a single round-trip.  Scope-aware: anonymous callers see only public subjects; authenticated callers also see subjects in accounts they can read.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Search query. Uses PostgreSQL `websearch_to_tsquery` syntax over the profile's summary, keyword labels, and facet keys/values/labels. Empty or missing returns recent profiles. |  |
**subject_type** | Option<**String**> | Restrict results to a subject type. |  |
**status** | Option<**String**> | Filter by profile status. Defaults to `ready` so callers only see profiles with complete content. Pass `all` to include pending and failed profiles. |  |[default to ready]
**order_by** | Option<**String**> | Sort order. `relevance` requires `q` (falls back to `popularity` when `q` is empty). `popularity` orders by the reuse-first composite of `discoverability_snapshots` counts and is the default when `q` is empty. Column-based values (`generated_at`, `inserted_at`, `updated_at`, `subject_guid`) sort by the named field. |  |
**first** | Option<**i32**> | Cursor pagination: number of items to return from the start. |  |[default to 20]
**after** | Option<**String**> | Cursor pagination: return items after this cursor. |  |
**last** | Option<**i32**> | Cursor pagination: number of items to return from the end. |  |[default to 20]
**before** | Option<**String**> | Cursor pagination: return items before this cursor. |  |
**limit** | Option<**i32**> | Offset pagination: maximum number of items to return. This is the default pagination mode when no pagination params are provided. |  |[default to 20]
**offset** | Option<**i32**> | Offset pagination: zero-based starting offset. |  |[default to 0]
**page** | Option<**i32**> | Page pagination: 1-based page number. |  |[default to 1]
**page_size** | Option<**i32**> | Page pagination: number of items per page. |  |[default to 20]

### Return type

[**models::RuslWebApiDiscoveryControllerSearch200Response**](RuslWeb_Api_DiscoveryController_search_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_discovery_controller_show

> models::RuslWebApiDiscoveryControllerShow200Response rusl_web_api_discovery_controller_show(subject_guid)
Fetch a discovery profile by subject GUID

Returns the discovery profile for a single subject GUID, paired with the hydrated subject. Responds 404 if the profile does not exist or the caller cannot see the subject.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_guid** | **String** | GUID of the subject (e.g. `schemas.<id>`, `bundles.<id>`) | [required] |

### Return type

[**models::RuslWebApiDiscoveryControllerShow200Response**](RuslWeb_Api_DiscoveryController_show_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
