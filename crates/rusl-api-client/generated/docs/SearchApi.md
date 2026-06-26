# \SearchApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_search_controller_annotation_types**](SearchApi.md#rusl_web_api_search_controller_annotation_types) | **POST** /api/v1/annotation-types/search | Search annotation types
[**rusl_web_api_search_controller_annotation_types_0**](SearchApi.md#rusl_web_api_search_controller_annotation_types_0) | **POST** /api/v1/annotation-types/search | Search annotation types
[**rusl_web_api_search_controller_annotations**](SearchApi.md#rusl_web_api_search_controller_annotations) | **POST** /api/v1/annotations/search | Search annotations
[**rusl_web_api_search_controller_annotations_0**](SearchApi.md#rusl_web_api_search_controller_annotations_0) | **POST** /api/v1/annotations/search | Search annotations
[**rusl_web_api_search_controller_bundles**](SearchApi.md#rusl_web_api_search_controller_bundles) | **POST** /api/v1/bundles/search | Search bundles
[**rusl_web_api_search_controller_bundles_0**](SearchApi.md#rusl_web_api_search_controller_bundles_0) | **POST** /api/v1/bundles/search | Search bundles
[**rusl_web_api_search_controller_global**](SearchApi.md#rusl_web_api_search_controller_global) | **POST** /api/v1/search | Search schemas, bundles, annotation types, and annotations
[**rusl_web_api_search_controller_global_0**](SearchApi.md#rusl_web_api_search_controller_global_0) | **POST** /api/v1/search | Search schemas, bundles, annotation types, and annotations
[**rusl_web_api_search_controller_schemas**](SearchApi.md#rusl_web_api_search_controller_schemas) | **POST** /api/v1/schemas/search | Search schemas
[**rusl_web_api_search_controller_schemas_0**](SearchApi.md#rusl_web_api_search_controller_schemas_0) | **POST** /api/v1/schemas/search | Search schemas



## rusl_web_api_search_controller_annotation_types

> models::SearchResponse rusl_web_api_search_controller_annotation_types(annotation_type_search_request)
Search annotation types

Search registered annotation type projection documents with annotation-type specific filters and facets.

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


## rusl_web_api_search_controller_annotation_types_0

> models::SearchResponse rusl_web_api_search_controller_annotation_types_0(annotation_type_search_request)
Search annotation types

Search registered annotation type projection documents with annotation-type specific filters and facets.

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


## rusl_web_api_search_controller_annotations

> models::SearchResponse rusl_web_api_search_controller_annotations(annotation_search_request)
Search annotations

Search annotation projection documents with annotation-specific filters and facets. Compact view excludes annotation content; full view includes the annotation content and bounded summaries for registered type and target subject context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**annotation_search_request** | Option<[**AnnotationSearchRequest**](AnnotationSearchRequest.md)> | Annotation Search Request |  |

### Return type

[**models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_search_controller_annotations_0

> models::SearchResponse rusl_web_api_search_controller_annotations_0(annotation_search_request)
Search annotations

Search annotation projection documents with annotation-specific filters and facets. Compact view excludes annotation content; full view includes the annotation content and bounded summaries for registered type and target subject context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**annotation_search_request** | Option<[**AnnotationSearchRequest**](AnnotationSearchRequest.md)> | Annotation Search Request |  |

### Return type

[**models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_search_controller_bundles

> models::SearchResponse rusl_web_api_search_controller_bundles(bundle_search_request)
Search bundles

Search bundle projection documents with bundle-specific filters, facets, and dependency-count sorting. Raw Typesense parameters are not accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_search_request** | Option<[**BundleSearchRequest**](BundleSearchRequest.md)> | Bundle Search Request |  |

### Return type

[**models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_search_controller_bundles_0

> models::SearchResponse rusl_web_api_search_controller_bundles_0(bundle_search_request)
Search bundles

Search bundle projection documents with bundle-specific filters, facets, and dependency-count sorting. Raw Typesense parameters are not accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundle_search_request** | Option<[**BundleSearchRequest**](BundleSearchRequest.md)> | Bundle Search Request |  |

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
Search schemas, bundles, annotation types, and annotations

Search the public server-side search surface across schemas, bundles, registered annotation types, and annotation documents. Access filtering is injected by the server: anonymous callers see public results, and authenticated callers also see private results in accounts they can access.

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


## rusl_web_api_search_controller_global_0

> models::SearchResponse rusl_web_api_search_controller_global_0(global_search_request)
Search schemas, bundles, annotation types, and annotations

Search the public server-side search surface across schemas, bundles, registered annotation types, and annotation documents. Access filtering is injected by the server: anonymous callers see public results, and authenticated callers also see private results in accounts they can access.

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


## rusl_web_api_search_controller_schemas_0

> models::SearchResponse rusl_web_api_search_controller_schemas_0(schema_search_request)
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
