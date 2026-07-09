# OpenApiSchema2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | **serde_json::Value** | JSON Schema proposal |
**description** | Option<**String**> | Description | [optional]
**valid_data** | Option<[**Vec<models::ExampleData1>**](ExampleData1.md)> | Valid Data (required when the schema content is object- or array-rooted; optional for enum, scalar, or $defs-only schemas) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
