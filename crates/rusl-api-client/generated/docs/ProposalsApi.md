# \ProposalsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_proposal_controller_accept**](ProposalsApi.md#rusl_web_api_proposal_controller_accept) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/accept | Accept a proposal
[**rusl_web_api_proposal_controller_close**](ProposalsApi.md#rusl_web_api_proposal_controller_close) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/close | Close a proposal
[**rusl_web_api_proposal_controller_create**](ProposalsApi.md#rusl_web_api_proposal_controller_create) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals | Create proposals
[**rusl_web_api_proposal_controller_index**](ProposalsApi.md#rusl_web_api_proposal_controller_index) | **GET** /api/{account_slug}/schemas/{schema_slug}/proposals | List Proposals
[**rusl_web_api_proposal_controller_lookup**](ProposalsApi.md#rusl_web_api_proposal_controller_lookup) | **GET** /api/schema_proposals/lookup | Lookup proposals by IDs
[**rusl_web_api_proposal_controller_rebase**](ProposalsApi.md#rusl_web_api_proposal_controller_rebase) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/rebase | Rebase a proposal
[**rusl_web_api_proposal_controller_reject**](ProposalsApi.md#rusl_web_api_proposal_controller_reject) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/reject | Reject a proposal
[**rusl_web_api_proposal_controller_show**](ProposalsApi.md#rusl_web_api_proposal_controller_show) | **GET** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number} | Show a proposal
[**rusl_web_api_proposal_controller_update**](ProposalsApi.md#rusl_web_api_proposal_controller_update) | **PATCH** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number} | Update a proposal



## rusl_web_api_proposal_controller_accept

> models::RuslWebApiProposalControllerShow200Response rusl_web_api_proposal_controller_accept(proposal_number, account_slug, schema_slug, rusl_web_api_proposal_controller_accept_request)
Accept a proposal

Accept a proposal, creating a new schema version and updating the schema's current version. Optionally override the proposed version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**rusl_web_api_proposal_controller_accept_request** | Option<[**RuslWebApiProposalControllerAcceptRequest**](RuslWebApiProposalControllerAcceptRequest.md)> | Accept Proposal Request |  |

### Return type

[**models::RuslWebApiProposalControllerShow200Response**](RuslWeb_Api_ProposalController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_close

> models::RuslWebApiProposalControllerShow200Response rusl_web_api_proposal_controller_close(proposal_number, account_slug, schema_slug, rusl_web_api_proposal_controller_close_request)
Close a proposal

Close a proposal without accepting it. Can only be done by the proposal creator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**rusl_web_api_proposal_controller_close_request** | Option<[**RuslWebApiProposalControllerCloseRequest**](RuslWebApiProposalControllerCloseRequest.md)> | Close Proposal Request |  |

### Return type

[**models::RuslWebApiProposalControllerShow200Response**](RuslWeb_Api_ProposalController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_create

> models::RuslWebApiProposalControllerShow200Response rusl_web_api_proposal_controller_create(account_slug, schema_slug, open_api_schema3)
Create proposals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**open_api_schema3** | Option<[**OpenApiSchema3**](OpenApiSchema3.md)> | Create Schema Proposal Request |  |

### Return type

[**models::RuslWebApiProposalControllerShow200Response**](RuslWeb_Api_ProposalController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_index

> models::RuslWebApiProposalControllerIndex200Response rusl_web_api_proposal_controller_index(account_slug, schema_slug, status, first, after, last, before, limit, offset, page, page_size)
List Proposals

Paginate proposals for a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**status** | Option<**String**> | Status |  |
**first** | Option<**i32**> | Cursor pagination: number of items to return from the start. |  |[default to 20]
**after** | Option<**String**> | Cursor pagination: return items after this cursor. |  |
**last** | Option<**i32**> | Cursor pagination: number of items to return from the end. |  |[default to 20]
**before** | Option<**String**> | Cursor pagination: return items before this cursor. |  |
**limit** | Option<**i32**> | Offset pagination: maximum number of items to return. This is the default pagination mode when no pagination params are provided. |  |[default to 20]
**offset** | Option<**i32**> | Offset pagination: zero-based starting offset. |  |[default to 0]
**page** | Option<**i32**> | Page pagination: 1-based page number. |  |[default to 1]
**page_size** | Option<**i32**> | Page pagination: number of items per page. |  |[default to 20]

### Return type

[**models::RuslWebApiProposalControllerIndex200Response**](RuslWeb_Api_ProposalController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_lookup

> models::RuslWebApiProposalControllerLookup200Response rusl_web_api_proposal_controller_lookup(ids)
Lookup proposals by IDs

Returns schema proposals matching the given comma-separated IDs (max 100).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma-separated proposal IDs | [required] |

### Return type

[**models::RuslWebApiProposalControllerLookup200Response**](RuslWeb_Api_ProposalController_lookup_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_rebase

> models::RuslWebApiProposalControllerShow200Response rusl_web_api_proposal_controller_rebase(proposal_number, account_slug, schema_slug)
Rebase a proposal

Rebase an outdated proposal to the current schema version. Recalculates version information based on the current schema state. Can be done by schema owner or proposal creator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiProposalControllerShow200Response**](RuslWeb_Api_ProposalController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_reject

> models::RuslWebApiProposalControllerShow200Response rusl_web_api_proposal_controller_reject(proposal_number, account_slug, schema_slug, rusl_web_api_proposal_controller_close_request)
Reject a proposal

Reject a proposal without accepting it. Can be done by schema collaborators (proposal.manage).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**rusl_web_api_proposal_controller_close_request** | Option<[**RuslWebApiProposalControllerCloseRequest**](RuslWebApiProposalControllerCloseRequest.md)> | Reject Proposal Request |  |

### Return type

[**models::RuslWebApiProposalControllerShow200Response**](RuslWeb_Api_ProposalController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_show

> models::RuslWebApiProposalControllerShow200Response rusl_web_api_proposal_controller_show(proposal_number, account_slug, schema_slug)
Show a proposal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiProposalControllerShow200Response**](RuslWeb_Api_ProposalController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_update

> models::RuslWebApiProposalControllerShow200Response rusl_web_api_proposal_controller_update(proposal_number, account_slug, schema_slug, open_api_schema1)
Update a proposal

Update a proposal's content, description, and/or valid_data. Recalculates version bump type automatically based on schema changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**open_api_schema1** | Option<[**OpenApiSchema1**](OpenApiSchema1.md)> | Update Schema Proposal Request |  |

### Return type

[**models::RuslWebApiProposalControllerShow200Response**](RuslWeb_Api_ProposalController_show_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
