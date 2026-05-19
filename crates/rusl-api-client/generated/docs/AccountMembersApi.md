# \AccountMembersApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_account_member_controller_delete**](AccountMembersApi.md#rusl_web_api_account_member_controller_delete) | **DELETE** /api/{account_slug}/members/{id} | Remove a member from an account
[**rusl_web_api_account_member_controller_index**](AccountMembersApi.md#rusl_web_api_account_member_controller_index) | **GET** /api/{account_slug}/members | List members of an account
[**rusl_web_api_account_member_controller_lookup**](AccountMembersApi.md#rusl_web_api_account_member_controller_lookup) | **GET** /api/account_members/lookup | Lookup account members by IDs
[**rusl_web_api_account_member_controller_update**](AccountMembersApi.md#rusl_web_api_account_member_controller_update) | **PUT** /api/{account_slug}/members/{id} | Update a member's roles



## rusl_web_api_account_member_controller_delete

> models::AccountMember rusl_web_api_account_member_controller_delete(account_slug, id)
Remove a member from an account

Removes a member from the account. Requires OWNER role. Cannot remove the last owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Membership ID | [required] |

### Return type

[**models::AccountMember**](AccountMember.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_member_controller_index

> models::AccountMemberListResponse rusl_web_api_account_member_controller_index(account_slug)
List members of an account

Returns all members of the account with their roles and user details.  For accounts with `team_visibility: public`, no authentication is required. For accounts with `team_visibility: private`, the caller must be an account member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |

### Return type

[**models::AccountMemberListResponse**](AccountMemberListResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_member_controller_lookup

> models::AccountMemberListResponse rusl_web_api_account_member_controller_lookup(ids)
Lookup account members by IDs

Returns account memberships matching the given comma-separated IDs (max 100). Results are filtered by account team visibility — memberships from private teams are only returned if the caller is a member of that account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma-separated membership IDs | [required] |

### Return type

[**models::AccountMemberListResponse**](AccountMemberListResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_member_controller_update

> models::AccountMember rusl_web_api_account_member_controller_update(account_slug, id, update_member_roles_request)
Update a member's roles

Changes a member's roles in the account. Requires OWNER role. Cannot demote the last owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Membership ID | [required] |
**update_member_roles_request** | Option<[**UpdateMemberRolesRequest**](UpdateMemberRolesRequest.md)> | Update Roles |  |

### Return type

[**models::AccountMember**](AccountMember.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
