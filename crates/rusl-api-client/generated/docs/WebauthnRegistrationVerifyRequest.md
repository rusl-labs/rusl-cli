# WebauthnRegistrationVerifyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attestation_object** | **String** | The attestationObject from AuthenticatorAttestationResponse, base64url-encoded (no padding). |
**challenge_token** | **String** | Opaque server-signed token from the options response. Send verbatim. |
**client_data_json** | **String** | The raw clientDataJSON string from AuthenticatorResponse. Send as-is (NOT base64-encoded) — the server needs the exact bytes. |
**device_name** | Option<**String**> | User-provided label, e.g. 'MacBook Pro' | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
