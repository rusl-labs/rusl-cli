# \AuthenticationApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_cli_auth_controller_create**](AuthenticationApi.md#rusl_web_api_cli_auth_controller_create) | **POST** /api/auth/cli/authorizations | Create CLI authorization code
[**rusl_web_api_cli_auth_controller_exchange**](AuthenticationApi.md#rusl_web_api_cli_auth_controller_exchange) | **POST** /api/auth/cli/token | Exchange CLI authorization code
[**rusl_web_api_magic_link_controller_confirm**](AuthenticationApi.md#rusl_web_api_magic_link_controller_confirm) | **POST** /api/auth/magic-link/confirm | Confirm a magic-link login
[**rusl_web_api_magic_link_controller_create**](AuthenticationApi.md#rusl_web_api_magic_link_controller_create) | **POST** /api/auth/magic-link | Request a magic-link login email
[**rusl_web_api_password_reset_controller_confirm**](AuthenticationApi.md#rusl_web_api_password_reset_controller_confirm) | **POST** /api/auth/forgot-password/confirm | Confirm a password reset
[**rusl_web_api_password_reset_controller_create**](AuthenticationApi.md#rusl_web_api_password_reset_controller_create) | **POST** /api/auth/forgot-password | Request a password reset email
[**rusl_web_api_session_controller_create**](AuthenticationApi.md#rusl_web_api_session_controller_create) | **POST** /api/login | Login
[**rusl_web_api_session_controller_delete**](AuthenticationApi.md#rusl_web_api_session_controller_delete) | **DELETE** /api/logout | Logout
[**rusl_web_api_session_controller_me**](AuthenticationApi.md#rusl_web_api_session_controller_me) | **GET** /api/auth/sessions/me | Get current session info
[**rusl_web_api_tokens_controller_exchange**](AuthenticationApi.md#rusl_web_api_tokens_controller_exchange) | **POST** /api/tokens/exchange | Exchange Token
[**rusl_web_api_tokens_controller_refresh**](AuthenticationApi.md#rusl_web_api_tokens_controller_refresh) | **POST** /api/tokens/refresh | Refresh Token
[**rusl_web_api_webauthn_controller_authentication_options**](AuthenticationApi.md#rusl_web_api_webauthn_controller_authentication_options) | **POST** /api/webauthn/authentication/options | Generate WebAuthn authentication options
[**rusl_web_api_webauthn_controller_authentication_verify**](AuthenticationApi.md#rusl_web_api_webauthn_controller_authentication_verify) | **POST** /api/webauthn/authentication/verify | Verify WebAuthn authentication
[**rusl_web_api_webauthn_controller_delete**](AuthenticationApi.md#rusl_web_api_webauthn_controller_delete) | **DELETE** /api/webauthn/credentials/{id} | Remove a passkey
[**rusl_web_api_webauthn_controller_index**](AuthenticationApi.md#rusl_web_api_webauthn_controller_index) | **GET** /api/webauthn/credentials | List passkey credentials
[**rusl_web_api_webauthn_controller_registration_options**](AuthenticationApi.md#rusl_web_api_webauthn_controller_registration_options) | **POST** /api/webauthn/registration/options | Generate WebAuthn registration options
[**rusl_web_api_webauthn_controller_registration_verify**](AuthenticationApi.md#rusl_web_api_webauthn_controller_registration_verify) | **POST** /api/webauthn/registration/verify | Verify WebAuthn registration
[**rusl_web_api_webauthn_controller_update**](AuthenticationApi.md#rusl_web_api_webauthn_controller_update) | **PATCH** /api/webauthn/credentials/{id} | Rename a passkey



## rusl_web_api_cli_auth_controller_create

> models::CliAuthorizationResponse1 rusl_web_api_cli_auth_controller_create(cli_authorization_request1)
Create CLI authorization code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cli_authorization_request1** | Option<[**CliAuthorizationRequest1**](CliAuthorizationRequest1.md)> | CLI Authorization Request |  |

### Return type

[**models::CliAuthorizationResponse1**](CliAuthorizationResponse_1.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_cli_auth_controller_exchange

> models::CliTokenExchangeResponse1 rusl_web_api_cli_auth_controller_exchange(cli_token_exchange_request1)
Exchange CLI authorization code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cli_token_exchange_request1** | Option<[**CliTokenExchangeRequest1**](CliTokenExchangeRequest1.md)> | CLI Token Exchange Request |  |

### Return type

[**models::CliTokenExchangeResponse1**](CliTokenExchangeResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_magic_link_controller_confirm

> models::LoginResponse rusl_web_api_magic_link_controller_confirm(magic_link_confirm_request)
Confirm a magic-link login

Logs the user in using the emailed token (confirms unconfirmed accounts).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**magic_link_confirm_request** | Option<[**MagicLinkConfirmRequest**](MagicLinkConfirmRequest.md)> | Magic-link confirmation |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_magic_link_controller_create

> models::AcceptedResponse rusl_web_api_magic_link_controller_create(email_request)
Request a magic-link login email

Always returns `{ok: true}` regardless of whether the email belongs to an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_request** | Option<[**EmailRequest**](EmailRequest.md)> | Email |  |

### Return type

[**models::AcceptedResponse**](AcceptedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_password_reset_controller_confirm

> models::LoginResponse rusl_web_api_password_reset_controller_confirm(forgot_password_confirm_request)
Confirm a password reset

Sets the new password using the emailed token and logs the user in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**forgot_password_confirm_request** | Option<[**ForgotPasswordConfirmRequest**](ForgotPasswordConfirmRequest.md)> | Reset confirmation |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_password_reset_controller_create

> models::AcceptedResponse rusl_web_api_password_reset_controller_create(email_request)
Request a password reset email

Always returns `{ok: true}` regardless of whether the email belongs to an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_request** | Option<[**EmailRequest**](EmailRequest.md)> | Email |  |

### Return type

[**models::AcceptedResponse**](AcceptedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_session_controller_create

> models::LoginResponse rusl_web_api_session_controller_create(login_request)
Login

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_request** | Option<[**LoginRequest**](LoginRequest.md)> | Login Request |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_session_controller_delete

> models::RuslWebApiSessionControllerDelete200Response rusl_web_api_session_controller_delete()
Logout

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RuslWebApiSessionControllerDelete200Response**](RuslWeb_Api_SessionController_delete_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_session_controller_me

> models::MeResponse rusl_web_api_session_controller_me()
Get current session info

Returns authentication status and user session data including accounts with roles and permissions. Does not require authentication.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MeResponse**](MeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_tokens_controller_exchange

> models::AccessTokenResponse1 rusl_web_api_tokens_controller_exchange()
Exchange Token

Using a refresh token, exchange it for a new access token

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccessTokenResponse1**](AccessTokenResponse_1.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_tokens_controller_refresh

> models::AccessTokenResponse1 rusl_web_api_tokens_controller_refresh()
Refresh Token

Using an access token, refresh it to get a new access token

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccessTokenResponse1**](AccessTokenResponse_1.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_webauthn_controller_authentication_options

> models::WebauthnAuthenticationOptionsResponse rusl_web_api_webauthn_controller_authentication_options()
Generate WebAuthn authentication options

Returns a challenge for navigator.credentials.get(). Uses discoverable credentials — no user identifier needed.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebauthnAuthenticationOptionsResponse**](WebauthnAuthenticationOptionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_webauthn_controller_authentication_verify

> models::LoginResponse rusl_web_api_webauthn_controller_authentication_verify(webauthn_authentication_verify_request)
Verify WebAuthn authentication

Verifies the assertion and issues JWT tokens. Response matches POST /api/login.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webauthn_authentication_verify_request** | Option<[**WebauthnAuthenticationVerifyRequest**](WebauthnAuthenticationVerifyRequest.md)> | Authentication Verify |  |

### Return type

[**models::LoginResponse**](LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_webauthn_controller_delete

> rusl_web_api_webauthn_controller_delete(id)
Remove a passkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Credential ID | [required] |

### Return type

 (empty response body)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_webauthn_controller_index

> models::WebauthnCredentialListResponse rusl_web_api_webauthn_controller_index()
List passkey credentials

Returns all registered passkeys for the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebauthnCredentialListResponse**](WebauthnCredentialListResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_webauthn_controller_registration_options

> models::WebauthnRegistrationOptionsResponse rusl_web_api_webauthn_controller_registration_options()
Generate WebAuthn registration options

Returns a challenge and options for navigator.credentials.create(). The challenge_token should be stored in the BFF session.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebauthnRegistrationOptionsResponse**](WebauthnRegistrationOptionsResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_webauthn_controller_registration_verify

> models::WebauthnRegistrationVerifyResponse rusl_web_api_webauthn_controller_registration_verify(webauthn_registration_verify_request)
Verify WebAuthn registration

Verifies the attestation and stores the new passkey credential.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webauthn_registration_verify_request** | Option<[**WebauthnRegistrationVerifyRequest**](WebauthnRegistrationVerifyRequest.md)> | Registration Verify |  |

### Return type

[**models::WebauthnRegistrationVerifyResponse**](WebauthnRegistrationVerifyResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_webauthn_controller_update

> models::WebauthnCredentialResponse rusl_web_api_webauthn_controller_update(id, webauthn_update_credential_request)
Rename a passkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Credential ID | [required] |
**webauthn_update_credential_request** | Option<[**WebauthnUpdateCredentialRequest**](WebauthnUpdateCredentialRequest.md)> | Update Credential |  |

### Return type

[**models::WebauthnCredentialResponse**](WebauthnCredentialResponse.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
