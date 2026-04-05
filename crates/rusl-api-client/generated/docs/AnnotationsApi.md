# \AnnotationsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_annotation_controller_create**](AnnotationsApi.md#rusl_web_api_annotation_controller_create) | **POST** /api/{account_slug}/annotations | Create an annotation
[**rusl_web_api_annotation_controller_deprecate**](AnnotationsApi.md#rusl_web_api_annotation_controller_deprecate) | **POST** /api/{account_slug}/annotations/{id}/deprecate | Deprecate an annotation
[**rusl_web_api_annotation_controller_filter**](AnnotationsApi.md#rusl_web_api_annotation_controller_filter) | **POST** /api/annotations/filter | Search annotations
[**rusl_web_api_annotation_controller_lookup**](AnnotationsApi.md#rusl_web_api_annotation_controller_lookup) | **GET** /api/annotations/lookup | Bulk lookup annotations
[**rusl_web_api_annotation_controller_reactivate**](AnnotationsApi.md#rusl_web_api_annotation_controller_reactivate) | **POST** /api/{account_slug}/annotations/{id}/reactivate | Reactivate an annotation
[**rusl_web_api_annotation_controller_revoke**](AnnotationsApi.md#rusl_web_api_annotation_controller_revoke) | **POST** /api/{account_slug}/annotations/{id}/revoke | Revoke an annotation
[**rusl_web_api_annotation_controller_show**](AnnotationsApi.md#rusl_web_api_annotation_controller_show) | **GET** /api/annotations/{id} | Get a single annotation
[**rusl_web_api_annotation_controller_type_typeahead**](AnnotationsApi.md#rusl_web_api_annotation_controller_type_typeahead) | **GET** /api/annotations/type_typeahead | Typeahead for annotation types
[**rusl_web_api_annotation_controller_types**](AnnotationsApi.md#rusl_web_api_annotation_controller_types) | **GET** /api/annotations/types | List distinct annotation types
[**rusl_web_api_annotation_controller_update**](AnnotationsApi.md#rusl_web_api_annotation_controller_update) | **PATCH** /api/{account_slug}/annotations/{id} | Update an annotation



## rusl_web_api_annotation_controller_create

> models::Annotation rusl_web_api_annotation_controller_create(account_slug, rusl_web_api_annotation_controller_create_request)
Create an annotation

Create a community annotation on a visible annotatable subject. Currently supported subjects: schemas, schema_versions, schema_proposals, bundles, bundle_versions, and annotations. Requires account membership.

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


## rusl_web_api_annotation_controller_deprecate

> models::Annotation rusl_web_api_annotation_controller_deprecate(account_slug, id, rusl_web_api_annotation_controller_revoke_request)
Deprecate an annotation

Marks an annotation as deprecated without creating a new one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Annotation ID | [required] |
**rusl_web_api_annotation_controller_revoke_request** | Option<[**RuslWebApiAnnotationControllerRevokeRequest**](RuslWebApiAnnotationControllerRevokeRequest.md)> | Deprecate Annotation |  |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_filter

> models::Annotation rusl_web_api_annotation_controller_filter()
Search annotations

Flop-paginated search across annotations. Filterable by subject_type, subject_guid, type, account_slug, status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_lookup

> models::RuslWebApiAnnotationControllerLookup200Response rusl_web_api_annotation_controller_lookup(ids, subject_guids, types, account_slugs, statuses)
Bulk lookup annotations

Look up annotations by IDs or subject GUIDs with subject visibility.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<**String**> | Comma-separated annotation IDs |  |
**subject_guids** | Option<**String**> | Comma-separated subject GUIDs |  |
**types** | Option<**String**> | Comma-separated annotation types |  |
**account_slugs** | Option<**String**> | Comma-separated account slugs |  |
**statuses** | Option<**String**> | Comma-separated statuses (ACTIVE, DEPRECATED, REVOKED) or 'all' |  |

### Return type

[**models::RuslWebApiAnnotationControllerLookup200Response**](RuslWeb_Api_AnnotationController_lookup_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_reactivate

> models::Annotation rusl_web_api_annotation_controller_reactivate(account_slug, id, rusl_web_api_annotation_controller_revoke_request)
Reactivate an annotation

Reactivates a previously deprecated or revoked annotation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Annotation ID | [required] |
**rusl_web_api_annotation_controller_revoke_request** | Option<[**RuslWebApiAnnotationControllerRevokeRequest**](RuslWebApiAnnotationControllerRevokeRequest.md)> | Reactivate Annotation |  |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_revoke

> models::Annotation rusl_web_api_annotation_controller_revoke(account_slug, id, rusl_web_api_annotation_controller_revoke_request)
Revoke an annotation

Revokes an annotation so it is no longer considered valid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Annotation ID | [required] |
**rusl_web_api_annotation_controller_revoke_request** | Option<[**RuslWebApiAnnotationControllerRevokeRequest**](RuslWebApiAnnotationControllerRevokeRequest.md)> | Revoke Annotation |  |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
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


## rusl_web_api_annotation_controller_type_typeahead

> models::RuslWebApiAnnotationControllerTypes200Response rusl_web_api_annotation_controller_type_typeahead(q, limit)
Typeahead for annotation types

Returns a small set of visible active annotation type suggestions for a typeahead query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Type prefix to match |  |
**limit** | Option<**i32**> | Optional result limit. Defaults to 5 and is capped at 10. |  |

### Return type

[**models::RuslWebApiAnnotationControllerTypes200Response**](RuslWeb_Api_AnnotationController_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_types

> models::RuslWebApiAnnotationControllerTypes200Response rusl_web_api_annotation_controller_types(subject_guids, statuses)
List distinct annotation types

Returns distinct annotation types with counts for given subjects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_guids** | Option<**String**> | Comma-separated subject GUIDs to check |  |
**statuses** | Option<**String**> | Comma-separated statuses or 'all' |  |

### Return type

[**models::RuslWebApiAnnotationControllerTypes200Response**](RuslWeb_Api_AnnotationController_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_update

> models::Annotation rusl_web_api_annotation_controller_update(account_slug, id, rusl_web_api_annotation_controller_revoke_request)
Update an annotation

Update an annotation owned by your account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Annotation ID | [required] |
**rusl_web_api_annotation_controller_revoke_request** | Option<[**RuslWebApiAnnotationControllerRevokeRequest**](RuslWebApiAnnotationControllerRevokeRequest.md)> | Update Annotation |  |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

