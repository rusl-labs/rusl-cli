# \NotificationsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_notification_controller_index**](NotificationsApi.md#rusl_web_api_notification_controller_index) | **GET** /api/notifications | List notifications inbox
[**rusl_web_api_notification_controller_mark_all_read**](NotificationsApi.md#rusl_web_api_notification_controller_mark_all_read) | **POST** /api/notifications/read_all | Mark all notifications as read
[**rusl_web_api_notification_controller_mark_read**](NotificationsApi.md#rusl_web_api_notification_controller_mark_read) | **POST** /api/notifications/{notification_id}/read | Mark one notification as read



## rusl_web_api_notification_controller_index

> models::RuslWebApiNotificationControllerIndex200Response rusl_web_api_notification_controller_index(status, first, after)
List notifications inbox

List the authenticated user's notification inbox with pagination. Use `status` to filter by read state (unread, read). Omit for all.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Filter by status: unread, read |  |
**first** | Option<**i32**> | Page size (1..100) |  |
**after** | Option<**String**> | Cursor for pagination |  |

### Return type

[**models::RuslWebApiNotificationControllerIndex200Response**](RuslWeb_Api_NotificationController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_notification_controller_mark_all_read

> models::RuslWebApiNotificationControllerMarkAllRead200Response rusl_web_api_notification_controller_mark_all_read()
Mark all notifications as read

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RuslWebApiNotificationControllerMarkAllRead200Response**](RuslWeb_Api_NotificationController_mark_all_read_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_notification_controller_mark_read

> models::RuslWebApiNotificationControllerMarkRead200Response rusl_web_api_notification_controller_mark_read(notification_id)
Mark one notification as read

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | Notification recipient id | [required] |

### Return type

[**models::RuslWebApiNotificationControllerMarkRead200Response**](RuslWeb_Api_NotificationController_mark_read_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

