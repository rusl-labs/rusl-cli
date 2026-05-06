# \DiscoverabilityApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_discoverability_controller_batch**](DiscoverabilityApi.md#rusl_web_api_discoverability_controller_batch) | **POST** /api/discoverability/batch | Batch discoverability lookup
[**rusl_web_api_discoverability_controller_show**](DiscoverabilityApi.md#rusl_web_api_discoverability_controller_show) | **GET** /api/discoverability/{subject_guid} | Get discoverability for a resource



## rusl_web_api_discoverability_controller_batch

> models::RuslWebApiDiscoverabilityControllerBatch200Response rusl_web_api_discoverability_controller_batch(batch_discoverability_request1)
Batch discoverability lookup

Returns discoverability snapshots for multiple visible resources in one call. Max 50 GUIDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_discoverability_request1** | Option<[**BatchDiscoverabilityRequest1**](BatchDiscoverabilityRequest1.md)> | Batch Discoverability Request |  |

### Return type

[**models::RuslWebApiDiscoverabilityControllerBatch200Response**](RuslWeb_Api_DiscoverabilityController_batch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_discoverability_controller_show

> models::RuslWebApiDiscoverabilityControllerShow200Response rusl_web_api_discoverability_controller_show(subject_guid)
Get discoverability for a resource

Returns the discoverability snapshot for a single visible resource, including counts and viewer state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_guid** | **String** | GUID of the resource | [required] |

### Return type

[**models::RuslWebApiDiscoverabilityControllerShow200Response**](RuslWeb_Api_DiscoverabilityController_show_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

