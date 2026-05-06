# \SearchApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_search_controller_annotation_types**](SearchApi.md#rusl_web_api_search_controller_annotation_types) | **POST** /api/annotation-types/search | Search annotation types
[**rusl_web_api_search_controller_global**](SearchApi.md#rusl_web_api_search_controller_global) | **POST** /api/search | Search schemas and annotation types
[**rusl_web_api_search_controller_schemas**](SearchApi.md#rusl_web_api_search_controller_schemas) | **POST** /api/schemas/search | Search schemas



## rusl_web_api_search_controller_annotation_types

> models::SearchResponse rusl_web_api_search_controller_annotation_types(annotation_type_search_request)
Search annotation types

Search registered annotation type projection documents with annotation-type specific filters and facets. Annotation document search is intentionally not part of this endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**annotation_type_search_request** | Option<[**AnnotationTypeSearchRequest**](AnnotationTypeSearchRequest.md)> | Annotation Type Search Request |  |

### Return type

[**models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_search_controller_global

> models::SearchResponse rusl_web_api_search_controller_global(global_search_request)
Search schemas and annotation types

Search the public server-side search surface across schemas and registered annotation types. Access filtering is injected by the server: anonymous callers see public results, and authenticated callers also see private results in accounts they can access. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**global_search_request** | Option<[**GlobalSearchRequest**](GlobalSearchRequest.md)> | Global Search Request |  |

### Return type

[**models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_search_controller_schemas

> models::SearchResponse rusl_web_api_search_controller_schemas(schema_search_request)
Search schemas

Search schema projection documents with schema-specific filters and facets. Raw Typesense parameters are not accepted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_search_request** | Option<[**SchemaSearchRequest**](SchemaSearchRequest.md)> | Schema Search Request |  |

### Return type

[**models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

