# \BundleVersionsApi

All URIs are relative to *http://localhost:4000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rusl_web_api_bundle_version_controller_create**](BundleVersionsApi.md#rusl_web_api_bundle_version_controller_create) | **POST** /api/{account_slug}/bundles/{bundle_slug}/versions | Create a draft bundle version
[**rusl_web_api_bundle_version_controller_index**](BundleVersionsApi.md#rusl_web_api_bundle_version_controller_index) | **GET** /api/{account_slug}/bundles/{bundle_slug}/versions | List bundle versions
[**rusl_web_api_bundle_version_controller_publish**](BundleVersionsApi.md#rusl_web_api_bundle_version_controller_publish) | **POST** /api/{account_slug}/bundles/{bundle_slug}/versions/{version}/publish | Publish a draft bundle version
[**rusl_web_api_bundle_version_controller_show**](BundleVersionsApi.md#rusl_web_api_bundle_version_controller_show) | **GET** /api/{account_slug}/bundles/{bundle_slug}/versions/{version} | Fetch a bundle version
[**rusl_web_api_bundle_version_controller_update**](BundleVersionsApi.md#rusl_web_api_bundle_version_controller_update) | **PATCH** /api/{account_slug}/bundles/{bundle_slug}/versions/{version} | Update draft version manifest
[**rusl_web_api_bundle_version_controller_update_status**](BundleVersionsApi.md#rusl_web_api_bundle_version_controller_update_status) | **PATCH** /api/{account_slug}/bundles/{bundle_slug}/versions/{version}/status | Update bundle version status
[**rusl_web_api_bundle_version_controller_validate**](BundleVersionsApi.md#rusl_web_api_bundle_version_controller_validate) | **POST** /api/{account_slug}/bundles/{bundle_slug}/versions/{version}/validate | Validate a draft version's manifest



## rusl_web_api_bundle_version_controller_create

> models::RuslWebApiBundleVersionControllerUpdateStatus200Response rusl_web_api_bundle_version_controller_create(account_slug, bundle_slug, rusl_web_api_bundle_version_controller_create_request)
Create a draft bundle version

Creates a new DRAFT version with manifest content. At most one draft per bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**rusl_web_api_bundle_version_controller_create_request** | Option<[**RuslWebApiBundleVersionControllerCreateRequest**](RuslWebApiBundleVersionControllerCreateRequest.md)> | Create Bundle Version |  |

### Return type

[**models::RuslWebApiBundleVersionControllerUpdateStatus200Response**](RuslWeb_Api_BundleVersionController_update_status_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_version_controller_index

> models::RuslWebApiBundleVersionControllerIndex200Response rusl_web_api_bundle_version_controller_index(account_slug, bundle_slug, stability)
List bundle versions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**stability** | Option<**String**> | Optional stability filter (comma-separated list) |  |

### Return type

[**models::RuslWebApiBundleVersionControllerIndex200Response**](RuslWeb_Api_BundleVersionController_index_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_version_controller_publish

> models::RuslWebApiBundleVersionControllerUpdateStatus200Response rusl_web_api_bundle_version_controller_publish(account_slug, bundle_slug, version)
Publish a draft bundle version

Parses the manifest, validates entries, materializes them, and transitions the version from DRAFT to ACTIVE.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**version** | **String** | SemVer version | [required] |

### Return type

[**models::RuslWebApiBundleVersionControllerUpdateStatus200Response**](RuslWeb_Api_BundleVersionController_update_status_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_version_controller_show

> models::RuslWebApiBundleVersionControllerUpdateStatus200Response rusl_web_api_bundle_version_controller_show(account_slug, bundle_slug, version)
Fetch a bundle version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**version** | **String** | SemVer version | [required] |

### Return type

[**models::RuslWebApiBundleVersionControllerUpdateStatus200Response**](RuslWeb_Api_BundleVersionController_update_status_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_version_controller_update

> models::RuslWebApiBundleVersionControllerUpdateStatus200Response rusl_web_api_bundle_version_controller_update(account_slug, bundle_slug, version, rusl_web_api_bundle_version_controller_update_request)
Update draft version manifest

Replace the manifest content on a DRAFT version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**version** | **String** | SemVer version | [required] |
**rusl_web_api_bundle_version_controller_update_request** | Option<[**RuslWebApiBundleVersionControllerUpdateRequest**](RuslWebApiBundleVersionControllerUpdateRequest.md)> | Update Manifest |  |

### Return type

[**models::RuslWebApiBundleVersionControllerUpdateStatus200Response**](RuslWeb_Api_BundleVersionController_update_status_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_version_controller_update_status

> models::RuslWebApiBundleVersionControllerUpdateStatus200Response rusl_web_api_bundle_version_controller_update_status(account_slug, bundle_slug, version, rusl_web_api_bundle_version_controller_update_status_request)
Update bundle version status

Transition a published version's status (ACTIVE → DEPRECATED, YANKED, etc.)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**version** | **String** | SemVer version | [required] |
**rusl_web_api_bundle_version_controller_update_status_request** | Option<[**RuslWebApiBundleVersionControllerUpdateStatusRequest**](RuslWebApiBundleVersionControllerUpdateStatusRequest.md)> | Status Update |  |

### Return type

[**models::RuslWebApiBundleVersionControllerUpdateStatus200Response**](RuslWeb_Api_BundleVersionController_update_status_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rusl_web_api_bundle_version_controller_validate

> models::RuslWebApiBundleVersionControllerValidate200Response rusl_web_api_bundle_version_controller_validate(account_slug, bundle_slug, version)
Validate a draft version's manifest

Dry-run: parses the manifest, checks schemas exist and visibility rules. Returns errors or success without materializing entries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | **String** | Account slug | [required] |
**bundle_slug** | **String** | Bundle slug | [required] |
**version** | **String** | SemVer version | [required] |

### Return type

[**models::RuslWebApiBundleVersionControllerValidate200Response**](RuslWeb_Api_BundleVersionController_validate_200_response.md)

### Authorization

[authorization](../README.md#authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

