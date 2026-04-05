# \ProposalReviewsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_proposal_review_controller_create_comment**](ProposalReviewsApi.md#rusl_web_api_proposal_review_controller_create_comment) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/review_threads/{thread_id}/comments | Reply to review thread
[**rusl_web_api_proposal_review_controller_create_thread**](ProposalReviewsApi.md#rusl_web_api_proposal_review_controller_create_thread) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/review_threads | Create review thread
[**rusl_web_api_proposal_review_controller_delete_comment**](ProposalReviewsApi.md#rusl_web_api_proposal_review_controller_delete_comment) | **DELETE** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/review_threads/{thread_id}/comments/{comment_id} | Delete review comment
[**rusl_web_api_proposal_review_controller_index**](ProposalReviewsApi.md#rusl_web_api_proposal_review_controller_index) | **GET** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/review_threads | List review threads for proposal
[**rusl_web_api_proposal_review_controller_reopen_thread**](ProposalReviewsApi.md#rusl_web_api_proposal_review_controller_reopen_thread) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/review_threads/{thread_id}/reopen | Reopen review thread
[**rusl_web_api_proposal_review_controller_resolve_thread**](ProposalReviewsApi.md#rusl_web_api_proposal_review_controller_resolve_thread) | **POST** /api/{account_slug}/schemas/{schema_slug}/proposals/{proposal_number}/review_threads/{thread_id}/resolve | Resolve review thread



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

> models::RuslWebApiProposalReviewControllerReopenThread200Response rusl_web_api_proposal_review_controller_create_thread(proposal_number, account_slug, schema_slug, create_review_thread_request1)
Create review thread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |
**create_review_thread_request1** | Option<[**CreateReviewThreadRequest1**](CreateReviewThreadRequest1.md)> | Create Review Thread Request |  |

### Return type

[**models::RuslWebApiProposalReviewControllerReopenThread200Response**](RuslWeb_Api_ProposalReviewController_reopen_thread_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_review_controller_delete_comment

> models::RuslWebApiProposalReviewControllerDeleteComment200Response rusl_web_api_proposal_review_controller_delete_comment(proposal_number, thread_id, comment_id, account_slug, schema_slug)
Delete review comment

Deletes a review comment. If the comment is the root comment (depth 0), the entire thread and all its comments are deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**thread_id** | **String** | Review thread ID | [required] |
**comment_id** | **uuid::Uuid** | Review comment ID | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiProposalReviewControllerDeleteComment200Response**](RuslWeb_Api_ProposalReviewController_delete_comment_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
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


## rusl_web_api_proposal_review_controller_reopen_thread

> models::RuslWebApiProposalReviewControllerReopenThread200Response rusl_web_api_proposal_review_controller_reopen_thread(proposal_number, thread_id, account_slug, schema_slug)
Reopen review thread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**thread_id** | **String** | Review thread ID | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiProposalReviewControllerReopenThread200Response**](RuslWeb_Api_ProposalReviewController_reopen_thread_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_proposal_review_controller_resolve_thread

> models::RuslWebApiProposalReviewControllerReopenThread200Response rusl_web_api_proposal_review_controller_resolve_thread(proposal_number, thread_id, account_slug, schema_slug)
Resolve review thread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_number** | **i32** | Proposal number | [required] |
**thread_id** | **String** | Review thread ID | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiProposalReviewControllerReopenThread200Response**](RuslWeb_Api_ProposalReviewController_reopen_thread_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

