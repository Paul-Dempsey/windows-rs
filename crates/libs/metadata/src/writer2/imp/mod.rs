mod assembly_ref;
mod blobs;
mod resolution_scope;
mod strings;
mod type_def;
mod type_ref;
use super::*;
pub use assembly_ref::*;
pub use blobs::*;
pub use resolution_scope::*;
use std::collections::*;
pub use strings::*;
pub use type_def::*;
pub use type_ref::*;

pub fn write<P: AsRef<std::path::Path>>(module: &str, items: &[Item], references: &[P]) -> Vec<u8> {
    let mut strings = Strings::default();
    let mut blobs = Blobs::default();
    let mut type_def = TypeDef::default();
    let mut type_ref = TypeRef::default();
    let mut assembly_ref = AssemblyRef::default();

    assembly_ref.insert("mscorlib", (4, 0, 0, 0));
    type_ref.insert(("System", "ValueType"), ResolutionScope::AssemblyRef("mscorlib"));
    type_ref.insert(("System", "Enum"), ResolutionScope::AssemblyRef("mscorlib"));

    // Collect references...
    for item in items {
        type_def.insert(item);

        match item {
            Item::Struct(ty) => ty.fields.iter().for_each(|field| field.ty.reference(&type_def, &mut type_ref)),
            _ => {}
        }
    }

    type_def.index();
    type_ref.index();

    

    todo!()
    // Write single Module table entry with module name
}

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}
