# WebauthnAuthenticationVerifyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authenticator_data** | **String** | authenticatorData from AuthenticatorAssertionResponse, base64url-encoded (no padding). | 
**challenge_token** | **String** | Opaque server-signed token from the options response. Send verbatim. | 
**client_data_json** | **String** | The raw clientDataJSON string from AuthenticatorResponse. Send as-is (NOT base64-encoded) — the server needs the exact bytes. | 
**raw_id** | **String** | The credential rawId, base64url-encoded (no padding). | 
**signature** | **String** | signature from AuthenticatorAssertionResponse, base64url-encoded (no padding). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


