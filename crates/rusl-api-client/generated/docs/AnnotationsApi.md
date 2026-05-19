# \AnnotationsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_annotation_controller_activate**](AnnotationsApi.md#rusl_web_api_annotation_controller_activate) | **POST** /api/{account_slug}/annotations/{id}/activate | Activate an annotation
[**rusl_web_api_annotation_controller_create**](AnnotationsApi.md#rusl_web_api_annotation_controller_create) | **POST** /api/{account_slug}/annotations | Create an annotation
[**rusl_web_api_annotation_controller_deprecate**](AnnotationsApi.md#rusl_web_api_annotation_controller_deprecate) | **POST** /api/{account_slug}/annotations/{id}/deprecate | Deprecate an annotation
[**rusl_web_api_annotation_controller_endorse**](AnnotationsApi.md#rusl_web_api_annotation_controller_endorse) | **POST** /api/annotations/{id}/endorse | Endorse an annotation
[**rusl_web_api_annotation_controller_filter**](AnnotationsApi.md#rusl_web_api_annotation_controller_filter) | **POST** /api/annotations/filter | Search annotations
[**rusl_web_api_annotation_controller_lookup**](AnnotationsApi.md#rusl_web_api_annotation_controller_lookup) | **GET** /api/annotations/lookup | Bulk lookup annotations
[**rusl_web_api_annotation_controller_reactivate**](AnnotationsApi.md#rusl_web_api_annotation_controller_reactivate) | **POST** /api/{account_slug}/annotations/{id}/reactivate | Reactivate an annotation
[**rusl_web_api_annotation_controller_revoke**](AnnotationsApi.md#rusl_web_api_annotation_controller_revoke) | **POST** /api/{account_slug}/annotations/{id}/revoke | Revoke an annotation
[**rusl_web_api_annotation_controller_show**](AnnotationsApi.md#rusl_web_api_annotation_controller_show) | **GET** /api/annotations/{id} | Get a single annotation
[**rusl_web_api_annotation_controller_type_typeahead**](AnnotationsApi.md#rusl_web_api_annotation_controller_type_typeahead) | **GET** /api/annotations/type_typeahead | Typeahead for annotation types
[**rusl_web_api_annotation_controller_types**](AnnotationsApi.md#rusl_web_api_annotation_controller_types) | **GET** /api/annotations/types | List distinct annotation types
[**rusl_web_api_annotation_controller_unendorse**](AnnotationsApi.md#rusl_web_api_annotation_controller_unendorse) | **DELETE** /api/annotations/{id}/endorse | Unendorse an annotation
[**rusl_web_api_annotation_controller_update**](AnnotationsApi.md#rusl_web_api_annotation_controller_update) | **PATCH** /api/{account_slug}/annotations/{id} | Update an annotation



## rusl_web_api_annotation_controller_activate

> models::Annotation rusl_web_api_annotation_controller_activate(account_slug, id)
Activate an annotation

Set an annotation's status to ACTIVE using an explicit lifecycle action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Annotation ID | [required] |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## rusl_web_api_annotation_controller_deprecate

> models::Annotation rusl_web_api_annotation_controller_deprecate(account_slug, id)
Deprecate an annotation

Set an annotation's status to DEPRECATED using an explicit lifecycle action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Annotation ID | [required] |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_endorse

> models::RuslWebApiReactionControllerFavourite201Response rusl_web_api_annotation_controller_endorse(id)
Endorse an annotation

Add a positive endorsement interaction to the annotation identified by ID. Endorsements are user-level signal boosts backed by resource interactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Annotation ID | [required] |

### Return type

[**models::RuslWebApiReactionControllerFavourite201Response**](RuslWeb_Api_ReactionController_favourite_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_filter

> models::RuslWebApiAnnotationControllerFilter200Response rusl_web_api_annotation_controller_filter(rusl_web_api_annotation_controller_filter_request)
Search annotations

Search annotations with Flop pagination and filtering support.  Supports filtering by: - q (text search across account slug, subject account slug, subject GUID, subject type, type, label, and validation schema identifier) - annotation_type_id - subject_guid - subject_type - subject_account_slug - type - type_cardinality - account_slug - status - set_by_user_id - inserted_at - updated_at

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rusl_web_api_annotation_controller_filter_request** | Option<[**RuslWebApiAnnotationControllerFilterRequest**](RuslWebApiAnnotationControllerFilterRequest.md)> | Filter parameters |  |

### Return type

[**models::RuslWebApiAnnotationControllerFilter200Response**](RuslWeb_Api_AnnotationController_filter_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_lookup

> models::RuslWebApiAnnotationControllerLookup200Response rusl_web_api_annotation_controller_lookup(ids, subject_guids, types, account_slugs)
Bulk lookup annotations

Look up annotations by IDs or subject GUIDs with subject visibility.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<**String**> | Comma-separated annotation IDs |  |
**subject_guids** | Option<**String**> | Comma-separated subject GUIDs |  |
**types** | Option<**String**> | Comma-separated annotation types |  |
**account_slugs** | Option<**String**> | Comma-separated account slugs |  |

### Return type

[**models::RuslWebApiAnnotationControllerLookup200Response**](RuslWeb_Api_AnnotationController_lookup_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_reactivate

> models::Annotation rusl_web_api_annotation_controller_reactivate(account_slug, id)
Reactivate an annotation

Set an annotation's status back to ACTIVE from another lifecycle state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Annotation ID | [required] |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_revoke

> models::Annotation rusl_web_api_annotation_controller_revoke(account_slug, id)
Revoke an annotation

Set an annotation's status to REVOKED using an explicit lifecycle action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Annotation ID | [required] |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

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

> models::RuslWebApiAnnotationControllerTypes200Response rusl_web_api_annotation_controller_types(subject_guids)
List distinct annotation types

Returns distinct annotation types with counts for given subjects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_guids** | Option<**String**> | Comma-separated subject GUIDs to check |  |

### Return type

[**models::RuslWebApiAnnotationControllerTypes200Response**](RuslWeb_Api_AnnotationController_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_unendorse

> models::RuslWebApiAnnotationControllerUnendorse200Response rusl_web_api_annotation_controller_unendorse(id)
Unendorse an annotation

Remove the authenticated user's endorsement interaction from an annotation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Annotation ID | [required] |

### Return type

[**models::RuslWebApiAnnotationControllerUnendorse200Response**](RuslWeb_Api_AnnotationController_unendorse_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_controller_update

> models::Annotation rusl_web_api_annotation_controller_update(account_slug, id, rusl_web_api_annotation_controller_update_request)
Update an annotation

Update an annotation owned by your account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**id** | **String** | Annotation ID | [required] |
**rusl_web_api_annotation_controller_update_request** | Option<[**RuslWebApiAnnotationControllerUpdateRequest**](RuslWebApiAnnotationControllerUpdateRequest.md)> | Update Annotation |  |

### Return type

[**models::Annotation**](Annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
