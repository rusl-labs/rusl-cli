# \VersionDependenciesApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_version_dependency_controller_dependents**](VersionDependenciesApi.md#rusl_web_api_version_dependency_controller_dependents) | **GET** /api/{account_slug}/schemas/{schema_slug}/dependents | List dependents of a schema
[**rusl_web_api_version_dependency_controller_index**](VersionDependenciesApi.md#rusl_web_api_version_dependency_controller_index) | **GET** /api/{account_slug}/schemas/{schema_slug}/versions/{version}/dependencies | List dependencies for a version
[**rusl_web_api_version_dependency_controller_lookup**](VersionDependenciesApi.md#rusl_web_api_version_dependency_controller_lookup) | **GET** /api/version_dependencies/lookup | Lookup version dependencies by version IDs



## rusl_web_api_version_dependency_controller_dependents

> models::RuslWebApiVersionDependencyControllerIndex200Response rusl_web_api_version_dependency_controller_dependents(account_slug, schema_slug)
List dependents of a schema

Returns all version dependencies that depend on this schema.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiVersionDependencyControllerIndex200Response**](RuslWeb_Api_VersionDependencyController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_version_dependency_controller_index

> models::RuslWebApiVersionDependencyControllerIndex200Response rusl_web_api_version_dependency_controller_index(version, account_slug, schema_slug)
List dependencies for a version

Returns all dependencies extracted from $ref URLs for a specific schema version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | Version string (e.g., '1.2.3') | [required] |
**account_slug** | **String** | Account slug | [required] |
**schema_slug** | **String** | Schema slug | [required] |

### Return type

[**models::RuslWebApiVersionDependencyControllerIndex200Response**](RuslWeb_Api_VersionDependencyController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_version_dependency_controller_lookup

> models::RuslWebApiVersionDependencyControllerIndex200Response rusl_web_api_version_dependency_controller_lookup(ids)
Lookup version dependencies by version IDs

Returns dependencies for the given schema version IDs (max 100). Visibility-scoped: dependencies for private schemas the caller cannot access are excluded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma-separated schema version IDs | [required] |

### Return type

[**models::RuslWebApiVersionDependencyControllerIndex200Response**](RuslWeb_Api_VersionDependencyController_index_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
