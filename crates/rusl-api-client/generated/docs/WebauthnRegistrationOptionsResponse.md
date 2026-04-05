# WebauthnRegistrationOptionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attestation** | Option<**Attestation**> |  (enum: none) | [optional]
**authenticator_selection** | Option<[**models::WebauthnRegistrationOptionsResponseAuthenticatorSelection**](WebauthnRegistrationOptionsResponseAuthenticatorSelection.md)> |  | [optional]
**challenge** | **String** | 32 random bytes, base64url-encoded (no padding). Decode to ArrayBuffer before passing to navigator.credentials.create(). | 
**challenge_token** | **String** | Server-signed opaque token. Store in BFF session, send back verbatim with the verify request. Do not decode. | 
**exclude_credentials** | Option<[**Vec<models::WebauthnRegistrationOptionsResponseExcludeCredentialsInner>**](WebauthnRegistrationOptionsResponseExcludeCredentialsInner.md)> | Credentials already registered for this user (prevent re-registration) | [optional]
**pub_key_cred_params** | [**Vec<models::WebauthnRegistrationOptionsResponsePubKeyCredParamsInner>**](WebauthnRegistrationOptionsResponsePubKeyCredParamsInner.md) |  | 
**rp** | [**models::WebauthnRegistrationOptionsResponseRp**](WebauthnRegistrationOptionsResponseRp.md) |  | 
**timeout** | Option<**i32**> | Timeout in milliseconds | [optional]
**user** | [**models::WebauthnRegistrationOptionsResponseUser**](WebauthnRegistrationOptionsResponseUser.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


