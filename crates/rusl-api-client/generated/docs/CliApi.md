# \CliApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_annotation_controller_create**](CliApi.md#rusl_web_api_annotation_controller_create) | **POST** /api/{account_slug}/annotations | Create an annotation
[**rusl_web_api_annotation_controller_endorse**](CliApi.md#rusl_web_api_annotation_controller_endorse) | **POST** /api/annotations/{id}/endorse | Endorse an annotation
[**rusl_web_api_annotation_controller_show**](CliApi.md#rusl_web_api_annotation_controller_show) | **GET** /api/annotations/{id} | Get a single annotation
[**rusl_web_api_annotation_type_controller_show**](CliApi.md#rusl_web_api_annotation_type_controller_show) | **GET** /api/{account_slug}/annotation_types/{annotation_type_slug} | Fetch a registered annotation type
[**rusl_web_api_bundle_controller_show**](CliApi.md#rusl_web_api_bundle_controller_show) | **GET** /api/{account_slug}/bundles/{bundle_slug} | Fetch a bundle
[**rusl_web_api_bundle_version_controller_show**](CliApi.md#rusl_web_api_bundle_version_controller_show) | **GET** /api/{account_slug}/bundles/{bundle_slug}/versions/{version} | Fetch a bundle version
[**rusl_web_api_cli_auth_controller_exchange**](CliApi.md#rusl_web_api_cli_auth_controller_exchange) | **POST** /api/auth/cli/token | Exchange CLI authorization code
[**rusl_web_api_proposal_controller_create**](CliApi.md#rusl_web_api_proposal_controller_create) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals | Create proposals
[**rusl_web_api_proposal_controller_show**](CliApi.md#rusl_web_api_proposal_controller_show) | **GET** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number} | Show a proposal
[**rusl_web_api_proposal_controller_update**](CliApi.md#rusl_web_api_proposal_controller_update) | **PATCH** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number} | Update a proposal
[**rusl_web_api_proposal_review_controller_create_comment**](CliApi.md#rusl_web_api_proposal_review_controller_create_comment) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/review_threads/{thread_id}/comments | Reply to review thread
[**rusl_web_api_proposal_review_controller_create_thread**](CliApi.md#rusl_web_api_proposal_review_controller_create_thread) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/review_threads | Create review thread
[**rusl_web_api_proposal_review_controller_index**](CliApi.md#rusl_web_api_proposal_review_controller_index) | **GET** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/review_threads | List review threads for proposal
[**rusl_web_api_schema_controller_create**](CliApi.md#rusl_web_api_schema_controller_create) | **POST** /api/{account_slug}/schemas | Create a new schema
[**rusl_web_api_schema_controller_show**](CliApi.md#rusl_web_api_schema_controller_show) | **GET** /api/{account_slug}/schemas/{schema_slug} | Fetch a schema
[**rusl_web_api_schema_version_controller_example_data_index**](CliApi.md#rusl_web_api_schema_version_controller_example_data_index) | **GET** /api/{account_slug}/schemas/{schema_slug}/example_data | List schema example data
[**rusl_web_api_schema_version_controller_show**](CliApi.md#rusl_web_api_schema_version_controller_show) | **GET** /api/{account_slug}/schemas/{schema_slug}/versions/{version} | Show a Schema Version
[**rusl_web_api_search_controller_annotation_types**](CliApi.md#rusl_web_api_search_controller_annotation_types) | **POST** /api/annotation-types/search | Search annotation types
[**rusl_web_api_search_controller_annotations**](CliApi.md#rusl_web_api_search_controller_annotations) | **POST** /api/annotations/search | Search annotations
[**rusl_web_api_search_controller_bundles**](CliApi.md#rusl_web_api_search_controller_bundles) | **POST** /api/bundles/search | Search bundles
[**rusl_web_api_search_controller_global**](CliApi.md#rusl_web_api_search_controller_global) | **POST** /api/search | Search schemas, bundles, annotation types, and annotations
[**rusl_web_api_search_controller_schemas**](CliApi.md#rusl_web_api_search_controller_schemas) | **POST** /api/schemas/search | Search schemas
[**rusl_web_api_session_controller_me**](CliApi.md#rusl_web_api_session_controller_me) | **GET** /api/auth/sessions/me | Get current session info
[**rusl_web_api_tokens_controller_exchange**](CliApi.md#rusl_web_api_tokens_controller_exchange) | **POST** /api/tokens/exchange | Exchange Token
[**rusl_web_raw_bundle_metadata_controller_show**](CliApi.md#rusl_web_raw_bundle_metadata_controller_show) | **GET** /resources/{account_slug}/bundles/{bundle_slug}/metadata | Bundle resolution metadata index
[**rusl_web_raw_schema_controller_show**](CliApi.md#rusl_web_raw_schema_controller_show) | **GET** /resources/{account_slug}/schemas/{schema_slug_and_version} | Serve raw JSON schema content
[**rusl_web_raw_schema_metadata_controller_show**](CliApi.md#rusl_web_raw_schema_metadata_controller_show) | **GET** /resources/{account_slug}/schemas/{schema_slug}/metadata | Schema resolution metadata index



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


## rusl_web_api_bundle_controller_show

> models::RuslWebApiBundleControllerShow200Response rusl_web_api_bundle_controller_show(account_slug, bundle_slug)
Fetch a bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |

### Return type

[**models::RuslWebApiBundleControllerShow200Response**](RuslWeb_Api_BundleController_show_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_version_controller_show

> models::RuslWebApiBundleVersionControllerShow200Response rusl_web_api_bundle_version_controller_show(account_slug, bundle_slug, version)
Fetch a bundle version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**version** | **String** | SemVer version | [required] |

### Return type

[**models::RuslWebApiBundleVersionControllerShow200Response**](RuslWeb_Api_BundleVersionController_show_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_cli_auth_controller_exchange

> models::CliTokenExchangeResponse1 rusl_web_api_cli_auth_controller_exchange(cli_token_exchange_request1)
Exchange CLI authorization code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cli_token_exchange_request1** | Option<[**CliTokenExchangeRequest1**](CliTokenExchangeRequest1.md)> | CLI Token Exchange Request |  |

### Return type

[**models::CliTokenExchangeResponse1**](CliTokenExchangeResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_create

> models::RuslWebApiProposalControllerCreate201Response rusl_web_api_proposal_controller_create(account_slug, schema_slug, open_api_schema2)
Create proposals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**open_api_schema2** | Option<[**OpenApiSchema2**](OpenApiSchema2.md)> | Create Schema Proposal Request |  |

### Return type

[**models::RuslWebApiProposalControllerCreate201Response**](RuslWeb_Api_ProposalController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_show

> models::RuslWebApiProposalControllerCreate201Response rusl_web_api_proposal_controller_show(proposal_number, account_slug, schema_slug)
Show a proposal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiProposalControllerCreate201Response**](RuslWeb_Api_ProposalController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_update

> models::RuslWebApiProposalControllerCreate201Response rusl_web_api_proposal_controller_update(proposal_number, account_slug, schema_slug, open_api_schema3)
Update a proposal

Update a proposal's content, description, and/or valid_data. Recalculates version bump type automatically based on schema changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**open_api_schema3** | Option<[**OpenApiSchema3**](OpenApiSchema3.md)> | Update Schema Proposal Request |  |

### Return type

[**models::RuslWebApiProposalControllerCreate201Response**](RuslWeb_Api_ProposalController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_review_controller_create_comment

> models::RuslWebApiProposalReviewControllerCreateComment201Response rusl_web_api_proposal_review_controller_create_comment(proposal_number, thread_id, account_slug, schema_slug, create_review_comment_request1)
Reply to review thread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**thread_id** | **String** | Review thread ID | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**create_review_comment_request1** | Option<[**CreateReviewCommentRequest1**](CreateReviewCommentRequest1.md)> | Create Review Comment Request |  |

### Return type

[**models::RuslWebApiProposalReviewControllerCreateComment201Response**](RuslWeb_Api_ProposalReviewController_create_comment_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_review_controller_create_thread

> models::RuslWebApiProposalReviewControllerCreateThread201Response rusl_web_api_proposal_review_controller_create_thread(proposal_number, account_slug, schema_slug, create_review_thread_request1)
Create review thread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**create_review_thread_request1** | Option<[**CreateReviewThreadRequest1**](CreateReviewThreadRequest1.md)> | Create Review Thread Request |  |

### Return type

[**models::RuslWebApiProposalReviewControllerCreateThread201Response**](RuslWeb_Api_ProposalReviewController_create_thread_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_review_controller_index

> models::RuslWebApiProposalReviewControllerIndex200Response rusl_web_api_proposal_review_controller_index(proposal_number, account_slug, schema_slug)
List review threads for proposal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiProposalReviewControllerIndex200Response**](RuslWeb_Api_ProposalReviewController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_create

> models::RuslWebApiSchemaControllerCreate201Response rusl_web_api_schema_controller_create(account_slug, open_api_schema1)
Create a new schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**open_api_schema1** | Option<[**OpenApiSchema1**](OpenApiSchema1.md)> | Create Schema Request |  |

### Return type

[**models::RuslWebApiSchemaControllerCreate201Response**](RuslWeb_Api_SchemaController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_controller_show

> models::RuslWebApiSchemaControllerCreate201Response rusl_web_api_schema_controller_show(account_slug, schema_slug)
Fetch a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiSchemaControllerCreate201Response**](RuslWeb_Api_SchemaController_create_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_version_controller_example_data_index

> models::RuslWebApiSchemaVersionControllerExampleDataIndex200Response rusl_web_api_schema_version_controller_example_data_index(account_slug, schema_slug, version, filters, order_by, order_directions, first, after, last, before, limit, offset, page, page_size)
List schema example data

Paginate committed example data for a schema across all committed versions.  Supports: - `version` prefix filtering against canonical schema version ordering (`1`, `1.2`, `1.2.3`) - Flop filters on `version`, `schema_version_id`, `position`, `title`, `inserted_at`, and `updated_at` - Flop ordering on `canonical_version`, `position`, `inserted_at`, and `updated_at`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**version** | Option<**String**> | Version filter - supports prefix matching (e.g., '1' matches 1.*.*, '1.2' matches 1.2.*, '1.2.3' matches exactly) |  |
**filters** | Option<[**std::collections::HashMap<String, models::RuslWebApiSchemaVersionControllerExampleDataIndexFiltersParameterValue>**](Models__RuslWebApiSchemaVersionControllerExampleDataIndexFiltersParameterValue.md)> | Flop filters for example data fields. See https://hexdocs.pm/flop/readme.html#parameter-format |  |
**order_by** | Option<[**Vec<String>**](String.md)> | Fields to order by |  |
**order_directions** | Option<[**Vec<String>**](String.md)> | Order directions |  |
**first** | Option<**i32**> | Cursor pagination: number of items to return from the start. |  |[default to 20]
**after** | Option<**String**> | Cursor pagination: return items after this cursor. |  |
**last** | Option<**i32**> | Cursor pagination: number of items to return from the end. |  |[default to 20]
**before** | Option<**String**> | Cursor pagination: return items before this cursor. |  |
**limit** | Option<**i32**> | Offset pagination: maximum number of items to return. This is the default pagination mode when no pagination params are provided. |  |[default to 20]
**offset** | Option<**i32**> | Offset pagination: zero-based starting offset. |  |[default to 0]
**page** | Option<**i32**> | Page pagination: 1-based page number. |  |[default to 1]
**page_size** | Option<**i32**> | Page pagination: number of items per page. |  |[default to 20]

### Return type

[**models::RuslWebApiSchemaVersionControllerExampleDataIndex200Response**](RuslWeb_Api_SchemaVersionController_example_data_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_schema_version_controller_show

> models::RuslWebApiSchemaVersionControllerShow200Response rusl_web_api_schema_version_controller_show(version, account_slug, schema_slug)
Show a Schema Version

Get a specific schema version by version string

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | Version string (e.g., '1.2.3') | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiSchemaVersionControllerShow200Response**](RuslWeb_Api_SchemaVersionController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## rusl_web_api_session_controller_me

> models::MeResponse rusl_web_api_session_controller_me()
Get current session info

Returns authentication status and user session data including accounts with roles and permissions. Does not require authentication.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MeResponse**](MeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_tokens_controller_exchange

> models::AccessTokenResponse1 rusl_web_api_tokens_controller_exchange()
Exchange Token

Using a refresh token, exchange it for a new access token

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccessTokenResponse1**](AccessTokenResponse_1.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_raw_bundle_metadata_controller_show

> models::RuslWebRawBundleMetadataControllerShow200Response rusl_web_raw_bundle_metadata_controller_show(account_slug, bundle_slug)
Bundle resolution metadata index

Returns every resolvable version of a bundle and its dependency constraints in a single payload, enabling the PubGrub resolver to evaluate the full dependency graph without additional network round-trips.  Canonical metadata lives under `/resources/{account_slug}/bundles/{bundle_slug}/metadata`.  Caching behavior (via `RuslWeb.RawCacheHeaders` + `Rusl.Caching`): - Public metadata: cacheable with a configurable TTL (`Rusl.Caching.metadata_max_age`).   Kept fresh via event-driven CDN purges on version publish or status changes (uses `cache-tag`). - Private metadata: always `private, no-store`. Never cached. - Version suffixes (`@vX.Y.Z`) are ignored for lookup and caching — the response reflects the current resolvable set.  Private data is never cacheable — membership is re-checked on every request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |

### Return type

[**models::RuslWebRawBundleMetadataControllerShow200Response**](RuslWeb_RawBundleMetadataController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_raw_schema_controller_show

> serde_json::Value rusl_web_raw_schema_controller_show(account_slug, schema_slug_and_version, disposition)
Serve raw JSON schema content

Serves the raw JSON schema content at the schema's canonical `/resources/{account_slug}/schemas/{schema_slug}` URL. Supports versioned access via `@v0.2.3` suffix for pinned, immutable content.  Caching behavior (via `RuslWeb.RawCacheHeaders`): - Public schemas: CDN-cacheable. Pinned versions (`@vX.Y.Z`) return `public, max-age=31536000, immutable` + ETag.   Clients and CDNs can use `If-None-Match` to receive 304 Not Modified responses. - Latest (no version suffix): shorter TTL (configurable via `Rusl.Caching.latest_max_age`). - Private schemas: always `private, no-store`. Never cached by CDNs or shared caches. - All public responses include `cache-tag` headers so the CDN can be purged on publish or status changes.  Private data is never cacheable — membership is re-checked on every request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug_and_version** | **String** | Schema slug, optionally with pinned version (e.g. `us-address` or `us-address@v1.2.3`) | [required] |
**disposition** | Option<**String**> | Optional response disposition. Omit or use `inline` to inspect the raw content; use `attachment` to force a file download. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_raw_schema_metadata_controller_show

> models::RuslWebRawSchemaMetadataControllerShow200Response rusl_web_raw_schema_metadata_controller_show(account_slug, schema_slug)
Schema resolution metadata index

Returns every resolvable version of a schema and its dependency constraints in a single payload, enabling the PubGrub resolver to evaluate the full dependency graph without additional network round-trips.  Canonical metadata lives under `/resources/{account_slug}/schemas/{schema_slug}/metadata`.  Caching behavior (via `RuslWeb.RawCacheHeaders` + `Rusl.Caching`): - Public metadata: cacheable with a configurable TTL (`Rusl.Caching.metadata_max_age`).   Kept fresh via event-driven CDN purges on version publish or status changes (uses `cache-tag`). - Private metadata: always `private, no-store`. Never cached. - Version suffixes (`@vX.Y.Z`) are ignored for lookup and caching — the response reflects the current resolvable set.  Private data is never cacheable — membership is re-checked on every request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebRawSchemaMetadataControllerShow200Response**](RuslWeb_RawSchemaMetadataController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
