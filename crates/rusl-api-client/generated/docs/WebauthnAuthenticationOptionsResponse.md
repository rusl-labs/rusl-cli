# WebauthnAuthenticationOptionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**challenge** | **String** | 32 random bytes, base64url-encoded (no padding). Decode to ArrayBuffer before passing to navigator.credentials.get(). |
**challenge_token** | **String** | Server-signed opaque token. Store in BFF session, send back verbatim with the verify request. Do not decode. |
**rp_id** | **String** | Relying party ID (domain) |
**timeout** | Option<**i32**> | Timeout in milliseconds | [optional]
**user_verification** | Option<**UserVerification**> |  (enum: preferred, required, discouraged) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
