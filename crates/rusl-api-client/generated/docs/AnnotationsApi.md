# \AnnotationsApi

All URIs are relative to *https://resources.rusl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_annotation_controller_create**](AnnotationsApi.md#rusl_web_api_annotation_controller_create) | **POST** /api/{account_slug}/annotations | Create an annotation
[**rusl_web_api_annotation_controller_create_0**](AnnotationsApi.md#rusl_web_api_annotation_controller_create_0) | **POST** /api/{account_slug}/annotations | Create an annotation
[**rusl_web_api_annotation_controller_endorse**](AnnotationsApi.md#rusl_web_api_annotation_controller_endorse) | **POST** /api/v1/annotations/{id}/endorse | Endorse an annotation
[**rusl_web_api_annotation_controller_endorse_0**](AnnotationsApi.md#rusl_web_api_annotation_controller_endorse_0) | **POST** /api/v1/annotations/{id}/endorse | Endorse an annotation
[**rusl_web_api_annotation_controller_show**](AnnotationsApi.md#rusl_web_api_annotation_controller_show) | **GET** /api/v1/annotations/{id} | Get a single annotation
[**rusl_web_api_annotation_controller_show_0**](AnnotationsApi.md#rusl_web_api_annotation_controller_show_0) | **GET** /api/v1/annotations/{id} | Get a single annotation



## rusl_web_api_annotation_controller_create

> models::Annotation rusl_web_api_annotation_controller_create(account_slug, rusl_web_api_annotation_controller_create_request)
Create an annotation

Create a community annotation on a visible annotatable subject. The type must be a registered annotation type identifier. Currently supported subjects: schemas, schema_versions, schema_proposals, bundles, bundle_versions, and annotations. Requires account membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**rusl_web_api_annotation_controller_create_request** | Option<[**RuslWebApiAnnotationControllerCreateRequest**](RuslWebApiAnnotationControllerCreateRequest.md)> | Create Annotation |  |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_create_0

> models::Annotation rusl_web_api_annotation_controller_create_0(account_slug, rusl_web_api_annotation_controller_create_request)
Create an annotation

Create a community annotation on a visible annotatable subject. The type must be a registered annotation type identifier. Currently supported subjects: schemas, schema_versions, schema_proposals, bundles, bundle_versions, and annotations. Requires account membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**rusl_web_api_annotation_controller_create_request** | Option<[**RuslWebApiAnnotationControllerCreateRequest**](RuslWebApiAnnotationControllerCreateRequest.md)> | Create Annotation |  |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_endorse

> models::RuslWebApiAnnotationControllerEndorse200Response rusl_web_api_annotation_controller_endorse(id)
Endorse an annotation

Add a positive endorsement interaction to the annotation identified by ID. Endorsements are user-level signal boosts backed by resource interactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Annotation ID | [required] |

### Return type

[**models::RuslWebApiAnnotationControllerEndorse200Response**](RuslWeb_Api_AnnotationController_endorse_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_endorse_0

> models::RuslWebApiAnnotationControllerEndorse200Response rusl_web_api_annotation_controller_endorse_0(id)
Endorse an annotation

Add a positive endorsement interaction to the annotation identified by ID. Endorsements are user-level signal boosts backed by resource interactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Annotation ID | [required] |

### Return type

[**models::RuslWebApiAnnotationControllerEndorse200Response**](RuslWeb_Api_AnnotationController_endorse_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_show

> models::Annotation rusl_web_api_annotation_controller_show(id)
Get a single annotation

Fetch an annotation by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Annotation ID | [required] |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_show_0

> models::Annotation rusl_web_api_annotation_controller_show_0(id)
Get a single annotation

Fetch an annotation by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Annotation ID | [required] |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
