# \AccountInvitationsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_account_invitation_controller_accept**](AccountInvitationsApi.md#rusl_web_api_account_invitation_controller_accept) | **POST** /api/account_invitations/{id}/accept | Accept an invitation
[**rusl_web_api_account_invitation_controller_accept_by_token**](AccountInvitationsApi.md#rusl_web_api_account_invitation_controller_accept_by_token) | **POST** /api/account_invitations/accept | Accept an invitation by one-time token
[**rusl_web_api_account_invitation_controller_account_index**](AccountInvitationsApi.md#rusl_web_api_account_invitation_controller_account_index) | **GET** /api/{account_slug}/invitations | List invitations for an account
[**rusl_web_api_account_invitation_controller_create**](AccountInvitationsApi.md#rusl_web_api_account_invitation_controller_create) | **POST** /api/{account_slug}/invitations | Invite a member to an account
[**rusl_web_api_account_invitation_controller_index**](AccountInvitationsApi.md#rusl_web_api_account_invitation_controller_index) | **GET** /api/account_invitations | List invitations for current user
[**rusl_web_api_account_invitation_controller_preview**](AccountInvitationsApi.md#rusl_web_api_account_invitation_controller_preview) | **GET** /api/account_invitations/preview | Preview an invitation by token (public, unauthenticated)
[**rusl_web_api_account_invitation_controller_reject**](AccountInvitationsApi.md#rusl_web_api_account_invitation_controller_reject) | **POST** /api/account_invitations/{id}/reject | Reject an invitation
[**rusl_web_api_account_invitation_controller_revoke**](AccountInvitationsApi.md#rusl_web_api_account_invitation_controller_revoke) | **POST** /api/{account_slug}/invitations/{id}/revoke | Revoke a pending account invitation



## rusl_web_api_account_invitation_controller_accept

> models::AccountInvitationResponse rusl_web_api_account_invitation_controller_accept(id)
Accept an invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Invitation ID | [required] |

### Return type

[**models::AccountInvitationResponse**](AccountInvitationResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_invitation_controller_accept_by_token

> models::AccountInvitationResponse rusl_web_api_account_invitation_controller_accept_by_token(accept_by_token_request)
Accept an invitation by one-time token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_by_token_request** | Option<[**AcceptByTokenRequest**](AcceptByTokenRequest.md)> | Accept Invitation Token |  |

### Return type

[**models::AccountInvitationResponse**](AccountInvitationResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_invitation_controller_account_index

> models::AccountInvitationListResponse rusl_web_api_account_invitation_controller_account_index(account_slug, status)
List invitations for an account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**status** | Option<**String**> | Optional invitation statuses (comma-separated). Defaults to PENDING. Supported: PENDING, ACCEPTED, REJECTED, REVOKED, EXPIRED |  |

### Return type

[**models::AccountInvitationListResponse**](AccountInvitationListResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_invitation_controller_create

> models::AccountInviteResponse rusl_web_api_account_invitation_controller_create(account_slug, invite_request)
Invite a member to an account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**invite_request** | Option<[**InviteRequest**](InviteRequest.md)> | Invite Request |  |

### Return type

[**models::AccountInviteResponse**](AccountInviteResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_invitation_controller_index

> models::AccountInvitationListResponse rusl_web_api_account_invitation_controller_index(status)
List invitations for current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Optional invitation statuses (comma-separated). Defaults to PENDING. Supported: PENDING, ACCEPTED, REJECTED, REVOKED, EXPIRED |  |

### Return type

[**models::AccountInvitationListResponse**](AccountInvitationListResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_invitation_controller_preview

> models::AccountInvitationPreviewResponse rusl_web_api_account_invitation_controller_preview(token)
Preview an invitation by token (public, unauthenticated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Raw one-time invitation token from the invitation email | [required] |

### Return type

[**models::AccountInvitationPreviewResponse**](AccountInvitationPreviewResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_invitation_controller_reject

> models::AccountInvitationResponse rusl_web_api_account_invitation_controller_reject(id)
Reject an invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Invitation ID | [required] |

### Return type

[**models::AccountInvitationResponse**](AccountInvitationResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_account_invitation_controller_revoke

> models::AccountInvitationResponse rusl_web_api_account_invitation_controller_revoke(account_slug, id)
Revoke a pending account invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Invitation ID | [required] |

### Return type

[**models::AccountInvitationResponse**](AccountInvitationResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

