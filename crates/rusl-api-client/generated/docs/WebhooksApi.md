# \WebhooksApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_webhook_controller_storage**](WebhooksApi.md#rusl_web_api_webhook_controller_storage) | **POST** /api/webhooks/storage | Storage Webhook



## rusl_web_api_webhook_controller_storage

> rusl_web_api_webhook_controller_storage()
Storage Webhook

Receives object-created event notifications from the configured object store — RustFS (`records`), AWS S3-compatible (`Records`), or Cloudflare R2 (`events`).  Secured via an Authorization header token matching STORAGE_WEBHOOK_SECRET. Every event in the payload is processed, transitioning each matching asset from :pending to :ready.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
