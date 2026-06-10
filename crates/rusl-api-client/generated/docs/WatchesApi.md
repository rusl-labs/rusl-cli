# \WatchesApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_watch_controller_unwatch_account**](WatchesApi.md#rusl_web_api_watch_controller_unwatch_account) | **DELETE** /api/accounts/{slug}/watch | Unwatch an account
[**rusl_web_api_watch_controller_unwatch_bundle**](WatchesApi.md#rusl_web_api_watch_controller_unwatch_bundle) | **DELETE** /api/{account_slug}/bundles/{bundle_slug}/watch | Unwatch a bundle
[**rusl_web_api_watch_controller_unwatch_proposal**](WatchesApi.md#rusl_web_api_watch_controller_unwatch_proposal) | **DELETE** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/watch | Unwatch a proposal
[**rusl_web_api_watch_controller_unwatch_schema**](WatchesApi.md#rusl_web_api_watch_controller_unwatch_schema) | **DELETE** /api/{account_slug}/schemas/{schema_slug}/watch | Unwatch a schema
[**rusl_web_api_watch_controller_watch_account**](WatchesApi.md#rusl_web_api_watch_controller_watch_account) | **POST** /api/accounts/{slug}/watch | Watch an account
[**rusl_web_api_watch_controller_watch_bundle**](WatchesApi.md#rusl_web_api_watch_controller_watch_bundle) | **POST** /api/{account_slug}/bundles/{bundle_slug}/watch | Watch a bundle
[**rusl_web_api_watch_controller_watch_proposal**](WatchesApi.md#rusl_web_api_watch_controller_watch_proposal) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/watch | Watch a proposal
[**rusl_web_api_watch_controller_watch_schema**](WatchesApi.md#rusl_web_api_watch_controller_watch_schema) | **POST** /api/{account_slug}/schemas/{schema_slug}/watch | Watch a schema



## rusl_web_api_watch_controller_unwatch_account

> models::RuslWebApiWatchControllerUnwatchAccount200Response rusl_web_api_watch_controller_unwatch_account(slug)
Unwatch an account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Account slug | [required] |

### Return type

[**models::RuslWebApiWatchControllerUnwatchAccount200Response**](RuslWeb_Api_WatchController_unwatch_account_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_watch_controller_unwatch_bundle

> models::RuslWebApiWatchControllerUnwatchBundle200Response rusl_web_api_watch_controller_unwatch_bundle(account_slug, bundle_slug)
Unwatch a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |

### Return type

[**models::RuslWebApiWatchControllerUnwatchBundle200Response**](RuslWeb_Api_WatchController_unwatch_bundle_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_watch_controller_unwatch_proposal

> models::RuslWebApiWatchControllerUnwatchProposal200Response rusl_web_api_watch_controller_unwatch_proposal(proposal_number, account_slug, schema_slug)
Unwatch a proposal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiWatchControllerUnwatchProposal200Response**](RuslWeb_Api_WatchController_unwatch_proposal_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_watch_controller_unwatch_schema

> models::RuslWebApiWatchControllerUnwatchSchema200Response rusl_web_api_watch_controller_unwatch_schema(account_slug, schema_slug)
Unwatch a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiWatchControllerUnwatchSchema200Response**](RuslWeb_Api_WatchController_unwatch_schema_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_watch_controller_watch_account

> models::RuslWebApiWatchControllerWatchBundle201Response rusl_web_api_watch_controller_watch_account(slug, watch_account_request1)
Watch an account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Account slug | [required] |
**watch_account_request1** | Option<[**WatchAccountRequest1**](WatchAccountRequest1.md)> | Watch Account Request |  |

### Return type

[**models::RuslWebApiWatchControllerWatchBundle201Response**](RuslWeb_Api_WatchController_watch_bundle_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_watch_controller_watch_bundle

> models::RuslWebApiWatchControllerWatchBundle201Response rusl_web_api_watch_controller_watch_bundle(account_slug, bundle_slug, watch_bundle_request1)
Watch a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**watch_bundle_request1** | Option<[**WatchBundleRequest1**](WatchBundleRequest1.md)> | Watch Bundle Request |  |

### Return type

[**models::RuslWebApiWatchControllerWatchBundle201Response**](RuslWeb_Api_WatchController_watch_bundle_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_watch_controller_watch_proposal

> models::RuslWebApiWatchControllerWatchBundle201Response rusl_web_api_watch_controller_watch_proposal(proposal_number, account_slug, schema_slug, watch_proposal_request1)
Watch a proposal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**watch_proposal_request1** | Option<[**WatchProposalRequest1**](WatchProposalRequest1.md)> | Watch Proposal Request |  |

### Return type

[**models::RuslWebApiWatchControllerWatchBundle201Response**](RuslWeb_Api_WatchController_watch_bundle_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_watch_controller_watch_schema

> models::RuslWebApiWatchControllerWatchBundle201Response rusl_web_api_watch_controller_watch_schema(account_slug, schema_slug, watch_schema_request1)
Watch a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**watch_schema_request1** | Option<[**WatchSchemaRequest1**](WatchSchemaRequest1.md)> | Watch Schema Request |  |

### Return type

[**models::RuslWebApiWatchControllerWatchBundle201Response**](RuslWeb_Api_WatchController_watch_bundle_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
