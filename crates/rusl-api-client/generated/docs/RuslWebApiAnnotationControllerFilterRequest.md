# RuslWebApiAnnotationControllerFilterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**after** | Option<**String**> | Cursor pagination: return items after this cursor. | [optional]
**before** | Option<**String**> | Cursor pagination: return items before this cursor. | [optional]
**filters** | Option<[**Vec<models::RuslWebApiAnnotationControllerFilterRequestFiltersInner>**](RuslWebApiAnnotationControllerFilterRequestFiltersInner.md)> | List of filters representing filter operations on specific fields. Filters are applied in the order they are defined. | [optional]
**first** | Option<**i32**> | Cursor pagination: number of items to return from the start. | [optional][default to 20]
**last** | Option<**i32**> | Cursor pagination: number of items to return from the end. | [optional][default to 20]
**limit** | Option<**i32**> | Offset pagination: maximum number of items to return. Defaults to the schema default limit. | [optional][default to 20]
**offset** | Option<**i32**> | Offset pagination: zero-based starting offset. | [optional][default to 0]
**order_by** | Option<**Vec<OrderBy>**> | Fields to order by. Optional because these endpoints define a default order; include to override it. (enum: inserted_at, updated_at, type, status, account_slug, endorsement_count) | [optional]
**order_directions** | Option<**Vec<OrderDirections>**> | Order directions applied to order_by. Missing entries default to ascending. (enum: asc, desc) | [optional]
**page** | Option<**i32**> | Page pagination: 1-based page number. | [optional][default to 1]
**page_size** | Option<**i32**> | Page pagination: number of items per page. | [optional][default to 20]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
