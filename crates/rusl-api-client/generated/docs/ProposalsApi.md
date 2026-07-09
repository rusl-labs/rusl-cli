# \ProposalsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_proposal_controller_create**](ProposalsApi.md#rusl_web_api_proposal_controller_create) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals | Create proposals
[**rusl_web_api_proposal_controller_create_0**](ProposalsApi.md#rusl_web_api_proposal_controller_create_0) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals | Create proposals
[**rusl_web_api_proposal_controller_show**](ProposalsApi.md#rusl_web_api_proposal_controller_show) | **GET** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number} | Show a proposal
[**rusl_web_api_proposal_controller_show_0**](ProposalsApi.md#rusl_web_api_proposal_controller_show_0) | **GET** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number} | Show a proposal
[**rusl_web_api_proposal_controller_update**](ProposalsApi.md#rusl_web_api_proposal_controller_update) | **PATCH** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number} | Update a proposal
[**rusl_web_api_proposal_controller_update_0**](ProposalsApi.md#rusl_web_api_proposal_controller_update_0) | **PATCH** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number} | Update a proposal



## rusl_web_api_proposal_controller_create

> models::RuslWebApiProposalControllerCreate201Response rusl_web_api_proposal_controller_create(account_slug, schema_slug, open_api_schema2)
Create proposals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |
**open_api_schema2** | Option<[**OpenApiSchema2**](OpenApiSchema2.md)> | Create Schema Proposal Request |  |

### Return type

[**models::RuslWebApiProposalControllerCreate201Response**](RuslWeb_Api_ProposalController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_create_0

> models::RuslWebApiProposalControllerCreate201Response rusl_web_api_proposal_controller_create_0(account_slug, schema_slug, open_api_schema2)
Create proposals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |
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
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |

### Return type

[**models::RuslWebApiProposalControllerCreate201Response**](RuslWeb_Api_ProposalController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_show_0

> models::RuslWebApiProposalControllerCreate201Response rusl_web_api_proposal_controller_show_0(proposal_number, account_slug, schema_slug)
Show a proposal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |

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
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |
**open_api_schema3** | Option<[**OpenApiSchema3**](OpenApiSchema3.md)> | Update Schema Proposal Request |  |

### Return type

[**models::RuslWebApiProposalControllerCreate201Response**](RuslWeb_Api_ProposalController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_controller_update_0

> models::RuslWebApiProposalControllerCreate201Response rusl_web_api_proposal_controller_update_0(proposal_number, account_slug, schema_slug, open_api_schema3)
Update a proposal

Update a proposal's content, description, and/or valid_data. Recalculates version bump type automatically based on schema changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug — the final identifier segment. Treat as an opaque compound: for a packaged schema it carries the dotted package path plus the leaf (e.g. `payments.checkout`). Do not split it; pass it through verbatim. | [required] |
**open_api_schema3** | Option<[**OpenApiSchema3**](OpenApiSchema3.md)> | Update Schema Proposal Request |  |

### Return type

[**models::RuslWebApiProposalControllerCreate201Response**](RuslWeb_Api_ProposalController_create_201_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
