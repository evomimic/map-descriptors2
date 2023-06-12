use hdi::prelude::*;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum BaseType {
    Holon,
    Collection,
    Composite,
    Relationship,
    Boolean,
    Integer,
    String,
    Enum,
}
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct TypeHeader {
    pub type_name: String,
    pub base_type: BaseType,
    pub description: String,
    pub is_dependent: bool,
}
pub fn validate_create_type_header(
    _action: EntryCreationAction,
    _type_header: TypeHeader,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_type_header(
    _action: Update,
    _type_header: TypeHeader,
    _original_action: EntryCreationAction,
    _original_type_header: TypeHeader,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Type Headers cannot be updated")))
}
pub fn validate_delete_type_header(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_type_header: TypeHeader,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Type Headers cannot be deleted")))
}
