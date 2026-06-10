# \FavouritesApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_reaction_controller_check**](FavouritesApi.md#rusl_web_api_reaction_controller_check) | **GET** /api/favourites/check | Check favourites for multiple entities
[**rusl_web_api_reaction_controller_favourite**](FavouritesApi.md#rusl_web_api_reaction_controller_favourite) | **POST** /api/favourites/{subject_guid} | Favourite an entity
[**rusl_web_api_reaction_controller_index**](FavouritesApi.md#rusl_web_api_reaction_controller_index) | **POST** /api/favourites/filter | List favourites
[**rusl_web_api_reaction_controller_reactions**](FavouritesApi.md#rusl_web_api_reaction_controller_reactions) | **POST** /api/reactions/filter | Search all reactions
[**rusl_web_api_reaction_controller_unfavourite**](FavouritesApi.md#rusl_web_api_reaction_controller_unfavourite) | **DELETE** /api/favourites/{subject_guid} | Unfavourite an entity



## rusl_web_api_reaction_controller_check

> models::RuslWebApiReactionControllerCheck200Response rusl_web_api_reaction_controller_check(subject_guids, reaction_types)
Check favourites for multiple entities

Check which of the given subject_guids the authenticated user has reacted to. Returns full interaction records for matches — absence means not reacted. Max 50 GUIDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_guids** | **String** | Comma-separated subject GUIDs (max 50) | [required] |
**reaction_types** | Option<**String**> | Comma-separated reaction types (default: favourite) |  |

### Return type

[**models::RuslWebApiReactionControllerCheck200Response**](RuslWeb_Api_ReactionController_check_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_reaction_controller_favourite

> models::RuslWebApiAnnotationControllerEndorse200Response rusl_web_api_reaction_controller_favourite(subject_guid)
Favourite an entity

Add a favourite reaction to the entity identified by subject_guid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_guid** | **String** | GUID of the entity to favourite | [required] |

### Return type

[**models::RuslWebApiAnnotationControllerEndorse200Response**](RuslWeb_Api_AnnotationController_endorse_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_reaction_controller_index

> models::RuslWebApiReactionControllerIndex200Response rusl_web_api_reaction_controller_index(rusl_web_api_reaction_controller_index_request)
List favourites

List the authenticated user's favourites with pagination and filtering by subject_type, subject_guid, or interaction_type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rusl_web_api_reaction_controller_index_request** | Option<[**RuslWebApiReactionControllerIndexRequest**](RuslWebApiReactionControllerIndexRequest.md)> | Filter parameters |  |

### Return type

[**models::RuslWebApiReactionControllerIndex200Response**](RuslWeb_Api_ReactionController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_reaction_controller_reactions

> models::RuslWebApiReactionControllerIndex200Response rusl_web_api_reaction_controller_reactions(rusl_web_api_reaction_controller_index_request)
Search all reactions

Search the authenticated user's reactions across all types with pagination and filtering. Filter by interaction_type, subject_guid, subject_type, or any combination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rusl_web_api_reaction_controller_index_request** | Option<[**RuslWebApiReactionControllerIndexRequest**](RuslWebApiReactionControllerIndexRequest.md)> | Filter parameters |  |

### Return type

[**models::RuslWebApiReactionControllerIndex200Response**](RuslWeb_Api_ReactionController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_reaction_controller_unfavourite

> models::RuslWebApiReactionControllerUnfavourite200Response rusl_web_api_reaction_controller_unfavourite(subject_guid)
Unfavourite an entity

Remove a favourite reaction from the entity identified by subject_guid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_guid** | **String** | GUID of the entity to unfavourite | [required] |

### Return type

[**models::RuslWebApiReactionControllerUnfavourite200Response**](RuslWeb_Api_ReactionController_unfavourite_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
