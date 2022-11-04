mod assembly_ref;
mod blobs;
mod definitions;
mod references;
mod resolution_scope;
mod strings;
mod tables;

use super::*;
use assembly_ref::*;
use blobs::*;
use std::collections::*;
use strings::*;
use tables::*;

pub use definitions::*;
pub use references::*;
pub use resolution_scope::*;

pub fn write<P: AsRef<std::path::Path>>(module: &str, items: &[Item], references: &[P]) -> Vec<u8> {
    let mut strings = Strings::default();
    let mut blobs = Blobs::default();
    let mut definitions = Definitions::default();
    let mut references = References::default();
    let mut assembly_ref = AssemblyRef::default();

    assembly_ref.insert("mscorlib", (4, 0, 0, 0));
    references.insert(("System", "ValueType"), ResolutionScope::AssemblyRef("mscorlib"));
    references.insert(("System", "Enum"), ResolutionScope::AssemblyRef("mscorlib"));

    // Collect definitions and references...
    for item in items {
        definitions.insert(item);

        match item {
            Item::Struct(ty) => ty.fields.iter().for_each(|field| field.ty.reference(&definitions, &mut references)),
            _ => {}
        }
    }

    // Index definitions and references...
    definitions.index();
    references.index();

    // Prepare tables...

    //

    todo!()
    // Write single Module table entry with module name
}

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}
