# \AssetsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_asset_controller_index**](AssetsApi.md#rusl_web_api_asset_controller_index) | **GET** /api/assets | List Assets by IDs
[**rusl_web_api_asset_controller_lookup**](AssetsApi.md#rusl_web_api_asset_controller_lookup) | **GET** /api/assets/lookup | Lookup assets by IDs
[**rusl_web_api_asset_controller_resolve**](AssetsApi.md#rusl_web_api_asset_controller_resolve) | **GET** /api/assets/{id}/resolve | Resolve an Asset URL
[**rusl_web_api_asset_controller_show**](AssetsApi.md#rusl_web_api_asset_controller_show) | **GET** /api/assets/{id} | Get an Asset
[**rusl_web_api_asset_controller_upload_intent**](AssetsApi.md#rusl_web_api_asset_controller_upload_intent) | **POST** /api/assets/upload_intent | Create an Upload Intent



## rusl_web_api_asset_controller_index

> models::RuslWebApiAssetControllerIndex200Response rusl_web_api_asset_controller_index(ids)
List Assets by IDs

Retrieve multiple assets by a comma-separated list of IDs (max 100).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma-separated asset IDs | [required] |

### Return type

[**models::RuslWebApiAssetControllerIndex200Response**](RuslWeb_Api_AssetController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_asset_controller_lookup

> models::RuslWebApiAssetControllerIndex200Response rusl_web_api_asset_controller_lookup(ids)
Lookup assets by IDs

Returns assets matching the given comma-separated IDs (max 100).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma-separated asset IDs | [required] |

### Return type

[**models::RuslWebApiAssetControllerIndex200Response**](RuslWeb_Api_AssetController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_asset_controller_resolve

> rusl_web_api_asset_controller_resolve(id)
Resolve an Asset URL

Redirects to the asset's resolved URL (Imgix/storage for managed, external URL for reference).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |

### Return type

 (empty response body)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_asset_controller_show

> models::RuslWebApiAssetControllerShow200Response rusl_web_api_asset_controller_show(id)
Get an Asset

Retrieve asset details by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Asset ID | [required] |

### Return type

[**models::RuslWebApiAssetControllerShow200Response**](RuslWeb_Api_AssetController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_asset_controller_upload_intent

> models::UploadIntentResponse1 rusl_web_api_asset_controller_upload_intent(upload_intent_request1)
Create an Upload Intent

Request a presigned URL for direct-to-storage file upload.  Creates an asset in :pending status and returns a presigned PUT URL. The client uploads the file directly to RustFS using this URL. RustFS notifies the API via webhook when the upload completes, transitioning the asset to :ready status. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_intent_request1** | Option<[**UploadIntentRequest1**](UploadIntentRequest1.md)> | Upload Intent Request |  |

### Return type

[**models::UploadIntentResponse1**](UploadIntentResponse_1.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

