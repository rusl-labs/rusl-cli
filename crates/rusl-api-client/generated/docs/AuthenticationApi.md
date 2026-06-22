# \AuthenticationApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_cli_auth_controller_exchange**](AuthenticationApi.md#rusl_web_api_cli_auth_controller_exchange) | **POST** /api/auth/cli/token | Exchange CLI authorization code
[**rusl_web_api_cli_auth_controller_exchange_0**](AuthenticationApi.md#rusl_web_api_cli_auth_controller_exchange_0) | **POST** /api/auth/cli/token | Exchange CLI authorization code
[**rusl_web_api_session_controller_me**](AuthenticationApi.md#rusl_web_api_session_controller_me) | **GET** /api/auth/sessions/me | Get current session info
[**rusl_web_api_session_controller_me_0**](AuthenticationApi.md#rusl_web_api_session_controller_me_0) | **GET** /api/auth/sessions/me | Get current session info
[**rusl_web_api_tokens_controller_exchange**](AuthenticationApi.md#rusl_web_api_tokens_controller_exchange) | **POST** /api/tokens/exchange | Exchange Token
[**rusl_web_api_tokens_controller_exchange_0**](AuthenticationApi.md#rusl_web_api_tokens_controller_exchange_0) | **POST** /api/tokens/exchange | Exchange Token



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


## rusl_web_api_cli_auth_controller_exchange_0

> models::CliTokenExchangeResponse1 rusl_web_api_cli_auth_controller_exchange_0(cli_token_exchange_request1)
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


## rusl_web_api_session_controller_me_0

> models::MeResponse rusl_web_api_session_controller_me_0()
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


## rusl_web_api_tokens_controller_exchange_0

> models::AccessTokenResponse1 rusl_web_api_tokens_controller_exchange_0()
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
