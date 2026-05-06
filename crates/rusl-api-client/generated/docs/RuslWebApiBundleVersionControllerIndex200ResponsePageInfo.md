# RuslWebApiBundleVersionControllerIndex200ResponsePageInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_offset** | Option<**i32**> | Current offset for offset or page pagination | [optional]
**current_page** | Option<**i32**> | Current page number for page or offset pagination | [optional]
**end_cursor** | Option<**String**> | Cursor for the last item in the current cursor window | [optional]
**has_next_page** | **bool** | Whether another page exists | 
**has_previous_page** | **bool** | Whether a previous page exists | 
**next_offset** | Option<**i32**> | Next offset for offset or page pagination | [optional]
**next_page** | Option<**i32**> | Next page number for page or offset pagination | [optional]
**page_size** | Option<**i32**> | Resolved page size or limit for the current result set | [optional]
**params** | Option<**serde_json::Value**> | Original params captured on validation errors | [optional]
**previous_offset** | Option<**i32**> | Previous offset for offset or page pagination | [optional]
**previous_page** | Option<**i32**> | Previous page number for page or offset pagination | [optional]
**start_cursor** | Option<**String**> | Cursor for the first item in the current cursor window | [optional]
**total_count** | Option<**i32**> | Total matching rows. Null for cursor pagination. | [optional]
**total_pages** | Option<**i32**> | Total pages for page or offset pagination | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


