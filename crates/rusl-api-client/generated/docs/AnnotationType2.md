# AnnotationType2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**__typename** | **Typename** | Type discriminator (enum: annotation_types) |
**account_slug** | **String** | Owning account slug |
**archived_at** | Option<**String**> | Archived at | [optional]
**cardinality** | **Cardinality** | How many active annotations of this type an account may attach to one subject (enum: ONE_PER_SUBJECT_PER_ACCOUNT, MANY_PER_SUBJECT_PER_ACCOUNT) |
**content_immutable** | **bool** | Whether annotation content becomes immutable after creation |
**description** | Option<**String**> | Annotation type description | [optional]
**guid** | **String** | Global ID |
**id** | **String** | Annotation type ID |
**inserted_at** | **String** | Created at |
**pinned_schema_version_id** | Option<**String**> | Pinned schema version ID when schema_mode is PINNED | [optional]
**schema_id** | **String** | Validation schema ID |
**schema_identifier** | Option<**String**> | Validation schema identifier | [optional]
**schema_mode** | **SchemaMode** | How annotation content is validated (enum: CURRENT, PINNED) |
**slug** | **String** | Annotation type slug |
**status** | **Status** | Annotation type lifecycle status (enum: ACTIVE, ARCHIVED) |
**subject_description** | Option<[**models::Annotation1SubjectDescription**](Annotation1SubjectDescription.md)> |  | [optional]
**type_identifier** | **String** | Full identifier in account_slug/annotation-types/slug form |
**updated_at** | **String** | Updated at |
**visibility** | **Visibility** | Annotation type visibility (enum: PUBLIC, PRIVATE) |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
