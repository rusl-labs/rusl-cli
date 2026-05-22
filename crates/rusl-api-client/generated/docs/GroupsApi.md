# \GroupsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_group_controller_index**](GroupsApi.md#rusl_web_api_group_controller_index) | **GET** /api/groups | List Groups
[**rusl_web_api_group_controller_members**](GroupsApi.md#rusl_web_api_group_controller_members) | **GET** /api/groups/{id}/members | List Group Members
[**rusl_web_api_group_controller_show**](GroupsApi.md#rusl_web_api_group_controller_show) | **GET** /api/groups/{id} | Get Group



## rusl_web_api_group_controller_index

> models::ListResponse rusl_web_api_group_controller_index(page, page_size, order_by)
List Groups

Returns all groups the authenticated user is a member of

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number |  |
**page_size** | Option<**i32**> | Number of items per page |  |
**order_by** | Option<[**Vec<String>**](String.md)> | Sort order |  |

### Return type

[**models::ListResponse**](ListResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_group_controller_members

> models::MembersResponse rusl_web_api_group_controller_members(id)
List Group Members

Returns all members of a group with their roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Group ID | [required] |

### Return type

[**models::MembersResponse**](MembersResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_group_controller_show

> models::DetailResponse rusl_web_api_group_controller_show(id)
Get Group

Returns a single group with its members

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Group ID | [required] |

### Return type

[**models::DetailResponse**](DetailResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
