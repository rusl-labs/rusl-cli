# \MetricsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_metrics_controller_lookup**](MetricsApi.md#rusl_web_api_metrics_controller_lookup) | **GET** /api/metrics/lookup | Batch lookup metrics
[**rusl_web_api_metrics_controller_show**](MetricsApi.md#rusl_web_api_metrics_controller_show) | **GET** /api/metrics/{subject_guid} | Get metrics for an entity



## rusl_web_api_metrics_controller_lookup

> models::RuslWebApiMetricsControllerLookup200Response rusl_web_api_metrics_controller_lookup(subject_guids)
Batch lookup metrics

Returns metrics for multiple entities in one call. Max 50 GUIDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_guids** | **String** | Comma-separated subject GUIDs (max 50) | [required] |

### Return type

[**models::RuslWebApiMetricsControllerLookup200Response**](RuslWeb_Api_MetricsController_lookup_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_metrics_controller_show

> models::RuslWebApiMetricsControllerShow200Response rusl_web_api_metrics_controller_show(subject_guid)
Get metrics for an entity

Returns the aggregated counters for an entity. Returns empty counters if no metrics exist yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_guid** | **String** | GUID of the entity | [required] |

### Return type

[**models::RuslWebApiMetricsControllerShow200Response**](RuslWeb_Api_MetricsController_show_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

