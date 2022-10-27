use std::collections::*;

pub struct TypeRef(pub BTreeMap<(String, String), Record>);

pub struct Record {
    assembly_name: String,
    assembly_version: (u16, u16),
    value_type: bool,
}