# \UsersApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_user_controller_bulk_lookup**](UsersApi.md#rusl_web_api_user_controller_bulk_lookup) | **GET** /api/users/lookup | Lookup users by IDs
[**rusl_web_api_user_controller_me**](UsersApi.md#rusl_web_api_user_controller_me) | **GET** /api/users/me | Current user information
[**rusl_web_api_user_controller_show**](UsersApi.md#rusl_web_api_user_controller_show) | **GET** /api/users/{id} | Get user by ID
[**rusl_web_api_user_controller_update**](UsersApi.md#rusl_web_api_user_controller_update) | **PATCH** /api/users/me | Update the current user
[**rusl_web_api_user_registration_controller_register**](UsersApi.md#rusl_web_api_user_registration_controller_register) | **POST** /api/users/register | Register a new user



## rusl_web_api_user_controller_bulk_lookup

> models::UserListResponse rusl_web_api_user_controller_bulk_lookup(ids)
Lookup users by IDs

Returns users matching the given comma-separated IDs (max 100).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma-separated user IDs | [required] |

### Return type

[**models::UserListResponse**](UserListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_user_controller_me

> models::RuslWebApiUserControllerMe200Response rusl_web_api_user_controller_me()
Current user information

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RuslWebApiUserControllerMe200Response**](RuslWeb_Api_UserController_me_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_user_controller_show

> models::RuslWebApiUserControllerUpdate200Response rusl_web_api_user_controller_show(id)
Get user by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User ID (UUID) | [required] |

### Return type

[**models::RuslWebApiUserControllerUpdate200Response**](RuslWeb_Api_UserController_update_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_user_controller_update

> models::RuslWebApiUserControllerUpdate200Response rusl_web_api_user_controller_update(update_user_request)
Update the current user

Updates self-reportable fields on the authenticated user. Changing `user_type` is mirrored onto the user's account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_request** | Option<[**UpdateUserRequest**](UpdateUserRequest.md)> | Update User Request |  |

### Return type

[**models::RuslWebApiUserControllerUpdate200Response**](RuslWeb_Api_UserController_update_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_user_registration_controller_register

> models::RegisterUserResponse1 rusl_web_api_user_registration_controller_register(open_api_schema4)
Register a new user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**open_api_schema4** | Option<[**OpenApiSchema4**](OpenApiSchema4.md)> | Register New User Request |  |

### Return type

[**models::RegisterUserResponse1**](RegisterUserResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
