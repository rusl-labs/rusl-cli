# RuslWebApiReactionControllerIndexRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**after** | Option<**String**> | After cursor | [optional]
**before** | Option<**String**> | Before cursor | [optional]
**filters** | Option<[**Vec<models::RuslWebApiReactionControllerIndexRequestFiltersInner>**](RuslWebApiReactionControllerIndexRequestFiltersInner.md)> | List of filters represents a filter operation on a specific field. The filters are applied in the order they are defined. | [optional]
**first** | Option<**i32**> | First | [optional][default to 20]
**last** | Option<**i32**> | Last | [optional][default to 20]
**order_by** | **Vec<OrderBy>** | List of fields to order by (enum: inserted_at) | 
**order_directions** | Option<**Vec<OrderDirections>**> | List of order directions applied to the fields defined in order_by. If empty or the list is shorter than the order_by list, :asc will be used as a default for each missing order direction. (enum: asc, desc) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


