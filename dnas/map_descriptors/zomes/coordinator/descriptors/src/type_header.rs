use hdk::prelude::*;
use descriptors_integrity::*;
use hdi::prelude::ExternResult;
pub fn new_type_header(
    type_name: String,
    base_type: BaseType,
    description: String,
    is_dependent: bool,
) -> ExternResult<TypeHeader> {
    let header = TypeHeader::new(
        type_name,
        base_type,
        description,
        SemanticVersion::default(),
        is_dependent,
    );
    Ok(header)
}
