# \SystemApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_health_controller_health**](SystemApi.md#rusl_web_health_controller_health) | **GET** /health | Health check endpoint



## rusl_web_health_controller_health

> models::HealthStatus rusl_web_health_controller_health()
Health check endpoint

Returns the health status of the application (including background dependency probes). Used for monitoring, load balancers, and orchestration (Render, Kubernetes, etc.). Returns 200 for both healthy and degraded states; only truly broken states may return 5xx from the coordinator itself.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HealthStatus**](HealthStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
