# \EventSubscriptionsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_event_subscription_controller_create**](EventSubscriptionsApi.md#rusl_web_api_event_subscription_controller_create) | **POST** /api/event_subscriptions | Create or update an event subscription
[**rusl_web_api_event_subscription_controller_delete**](EventSubscriptionsApi.md#rusl_web_api_event_subscription_controller_delete) | **DELETE** /api/event_subscriptions/{id} | Delete an event subscription
[**rusl_web_api_event_subscription_controller_index**](EventSubscriptionsApi.md#rusl_web_api_event_subscription_controller_index) | **POST** /api/event_subscriptions/filter | Search event subscriptions
[**rusl_web_api_event_subscription_controller_lookup**](EventSubscriptionsApi.md#rusl_web_api_event_subscription_controller_lookup) | **GET** /api/event_subscriptions/lookup | Lookup event subscriptions by IDs or subject GUIDs
[**rusl_web_api_event_subscription_controller_subscribable_types**](EventSubscriptionsApi.md#rusl_web_api_event_subscription_controller_subscribable_types) | **GET** /api/subscribable_types | List subscribable types



## rusl_web_api_event_subscription_controller_create

> models::RuslWebApiWatchControllerWatchBundle201Response rusl_web_api_event_subscription_controller_create(rusl_web_api_event_subscription_controller_create_request)
Create or update an event subscription

Create a subscription to watch a subscribable entity. Accepts subject_type + subject_id or a raw subject_guid. If a subscription already exists for the same subject, event_prefixes are merged.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rusl_web_api_event_subscription_controller_create_request** | Option<[**RuslWebApiEventSubscriptionControllerCreateRequest**](RuslWebApiEventSubscriptionControllerCreateRequest.md)> | Subscription parameters |  |

### Return type

[**models::RuslWebApiWatchControllerWatchBundle201Response**](RuslWeb_Api_WatchController_watch_bundle_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_event_subscription_controller_delete

> models::RuslWebApiWatchControllerWatchBundle201Response rusl_web_api_event_subscription_controller_delete(id)
Delete an event subscription

Deletes the subscription by ID. Only the subscription owner can delete it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Subscription ID | [required] |

### Return type

[**models::RuslWebApiWatchControllerWatchBundle201Response**](RuslWeb_Api_WatchController_watch_bundle_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_event_subscription_controller_index

> models::RuslWebApiEventSubscriptionControllerIndex200Response rusl_web_api_event_subscription_controller_index(first, after, order_by, order_directions, rusl_web_api_event_subscription_controller_index_request)
Search event subscriptions

Search the authenticated user's event subscriptions with pagination and filtering.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**first** | Option<**i32**> | Page size (1..100) |  |
**after** | Option<**String**> | Cursor for pagination |  |
**order_by** | Option<[**Vec<String>**](String.md)> | Fields to order by |  |
**order_directions** | Option<[**Vec<String>**](String.md)> | Order directions |  |
**rusl_web_api_event_subscription_controller_index_request** | Option<[**RuslWebApiEventSubscriptionControllerIndexRequest**](RuslWebApiEventSubscriptionControllerIndexRequest.md)> | Filter parameters |  |

### Return type

[**models::RuslWebApiEventSubscriptionControllerIndex200Response**](RuslWeb_Api_EventSubscriptionController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_event_subscription_controller_lookup

> models::RuslWebApiEventSubscriptionControllerLookup200Response rusl_web_api_event_subscription_controller_lookup(ids, subject_guids)
Lookup event subscriptions by IDs or subject GUIDs

Lookup the authenticated user's event subscriptions by IDs and/or subject GUIDs. At least one filter is required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<**String**> | Comma-separated subscription IDs |  |
**subject_guids** | Option<**String**> | Comma-separated subject GUIDs |  |

### Return type

[**models::RuslWebApiEventSubscriptionControllerLookup200Response**](RuslWeb_Api_EventSubscriptionController_lookup_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_event_subscription_controller_subscribable_types

> models::RuslWebApiEventSubscriptionControllerSubscribableTypes200Response rusl_web_api_event_subscription_controller_subscribable_types()
List subscribable types

Returns the list of entity types that can be subscribed to for event notifications.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RuslWebApiEventSubscriptionControllerSubscribableTypes200Response**](RuslWeb_Api_EventSubscriptionController_subscribable_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
