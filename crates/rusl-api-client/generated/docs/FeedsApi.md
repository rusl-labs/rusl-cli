# \FeedsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_feed_token_controller_create**](FeedsApi.md#rusl_web_api_feed_token_controller_create) | **POST** /api/feeds/token | Mint a short-lived S2 feed read token



## rusl_web_api_feed_token_controller_create

> models::Response rusl_web_api_feed_token_controller_create(request)
Mint a short-lived S2 feed read token

Exchanges the caller's session for a short-lived S2 read token. Requires the :subscribe_s2_feed entitlement. A public token reads all public feeds; a private token reads one account's feeds and requires membership of that account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | Option<[**Request**](Request.md)> | Feed token request |  |

### Return type

[**models::Response**](Response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
