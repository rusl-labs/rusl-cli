# \AnnotationTypesApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_annotation_type_controller_archive**](AnnotationTypesApi.md#rusl_web_api_annotation_type_controller_archive) | **POST** /api/{account_slug}/annotation_types/{annotation_type_slug}/archive | Archive a registered annotation type
[**rusl_web_api_annotation_type_controller_create**](AnnotationTypesApi.md#rusl_web_api_annotation_type_controller_create) | **POST** /api/{account_slug}/annotation_types | Create a registered annotation type
[**rusl_web_api_annotation_type_controller_index**](AnnotationTypesApi.md#rusl_web_api_annotation_type_controller_index) | **GET** /api/annotation_types | Search registered annotation types across accounts
[**rusl_web_api_annotation_type_controller_lookup**](AnnotationTypesApi.md#rusl_web_api_annotation_type_controller_lookup) | **GET** /api/annotation_types/lookup | Lookup annotation types by IDs
[**rusl_web_api_annotation_type_controller_show**](AnnotationTypesApi.md#rusl_web_api_annotation_type_controller_show) | **GET** /api/{account_slug}/annotation_types/{annotation_type_slug} | Fetch a registered annotation type
[**rusl_web_api_annotation_type_controller_typeahead**](AnnotationTypesApi.md#rusl_web_api_annotation_type_controller_typeahead) | **GET** /api/annotation_types/typeahead | Typeahead for registered annotation types
[**rusl_web_api_annotation_type_controller_unarchive**](AnnotationTypesApi.md#rusl_web_api_annotation_type_controller_unarchive) | **POST** /api/{account_slug}/annotation_types/{annotation_type_slug}/unarchive | Unarchive a registered annotation type
[**rusl_web_api_annotation_type_controller_update**](AnnotationTypesApi.md#rusl_web_api_annotation_type_controller_update) | **PATCH** /api/{account_slug}/annotation_types/{annotation_type_slug} | Update a registered annotation type



## rusl_web_api_annotation_type_controller_archive

> models::RuslWebApiAnnotationTypeControllerShow200Response rusl_web_api_annotation_type_controller_archive(account_slug, annotation_type_slug)
Archive a registered annotation type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**annotation_type_slug** | **String** | Annotation type slug | [required] |

### Return type

[**models::RuslWebApiAnnotationTypeControllerShow200Response**](RuslWeb_Api_AnnotationTypeController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_type_controller_create

> models::RuslWebApiAnnotationTypeControllerShow200Response rusl_web_api_annotation_type_controller_create(account_slug, rusl_web_api_annotation_type_controller_create_request)
Create a registered annotation type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**rusl_web_api_annotation_type_controller_create_request** | Option<[**RuslWebApiAnnotationTypeControllerCreateRequest**](RuslWebApiAnnotationTypeControllerCreateRequest.md)> | Create Annotation Type |  |

### Return type

[**models::RuslWebApiAnnotationTypeControllerShow200Response**](RuslWeb_Api_AnnotationTypeController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_type_controller_index

> models::RuslWebApiAnnotationTypeControllerIndex200Response rusl_web_api_annotation_type_controller_index(filters, order_by, order_directions, first, after)
Search registered annotation types across accounts

Search registered annotation types across all accounts with Flop pagination and filtering support.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<[**serde_json::Value**](SerdeJson__Value.md)> | Flop filters. See https://hexdocs.pm/flop/readme.html#parameter-format |  |
**order_by** | Option<[**Vec<String>**](String.md)> | Fields to order by |  |
**order_directions** | Option<[**Vec<String>**](String.md)> | Order directions |  |
**first** | Option<**i32**> | Page size |  |
**after** | Option<**String**> | Cursor for pagination |  |

### Return type

[**models::RuslWebApiAnnotationTypeControllerIndex200Response**](RuslWeb_Api_AnnotationTypeController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_type_controller_lookup

> models::RuslWebApiAnnotationTypeControllerLookup200Response rusl_web_api_annotation_type_controller_lookup(ids)
Lookup annotation types by IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma-separated annotation type IDs | [required] |

### Return type

[**models::RuslWebApiAnnotationTypeControllerLookup200Response**](RuslWeb_Api_AnnotationTypeController_lookup_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_type_controller_show

> models::RuslWebApiAnnotationTypeControllerShow200Response rusl_web_api_annotation_type_controller_show(account_slug, annotation_type_slug)
Fetch a registered annotation type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**annotation_type_slug** | **String** | Annotation type slug | [required] |

### Return type

[**models::RuslWebApiAnnotationTypeControllerShow200Response**](RuslWeb_Api_AnnotationTypeController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_type_controller_typeahead

> models::RuslWebApiAnnotationTypeControllerTypeahead200Response rusl_web_api_annotation_type_controller_typeahead(q, limit)
Typeahead for registered annotation types

Returns a small set of visible active registered annotation type suggestions for a typeahead query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Annotation type identifier prefix to match |  |
**limit** | Option<**i32**> | Optional result limit. Defaults to 5 and is capped at 10. |  |

### Return type

[**models::RuslWebApiAnnotationTypeControllerTypeahead200Response**](RuslWeb_Api_AnnotationTypeController_typeahead_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_type_controller_unarchive

> models::RuslWebApiAnnotationTypeControllerShow200Response rusl_web_api_annotation_type_controller_unarchive(account_slug, annotation_type_slug)
Unarchive a registered annotation type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**annotation_type_slug** | **String** | Annotation type slug | [required] |

### Return type

[**models::RuslWebApiAnnotationTypeControllerShow200Response**](RuslWeb_Api_AnnotationTypeController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_annotation_type_controller_update

> models::RuslWebApiAnnotationTypeControllerShow200Response rusl_web_api_annotation_type_controller_update(account_slug, annotation_type_slug, rusl_web_api_annotation_type_controller_update_request)
Update a registered annotation type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**annotation_type_slug** | **String** | Annotation type slug | [required] |
**rusl_web_api_annotation_type_controller_update_request** | Option<[**RuslWebApiAnnotationTypeControllerUpdateRequest**](RuslWebApiAnnotationTypeControllerUpdateRequest.md)> | Update Annotation Type |  |

### Return type

[**models::RuslWebApiAnnotationTypeControllerShow200Response**](RuslWeb_Api_AnnotationTypeController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

