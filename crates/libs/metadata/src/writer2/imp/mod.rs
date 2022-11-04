use super::*;

pub fn write<P: AsRef<std::path::Path>>(name: &str, defs: &[Item], refs: &[P]) -> Vec<u8> {


    todo!()
}

// Strings
// * BTreeMap of string to offset to ensure string folding
// * build stream on the fly

// Blobs
// * Heap, no need for sorting
// * Defer blob generation until TypeDef + TypeRef table is built

// TypeDefs
// * BTreeMap of namespace+name to type to ensure sorted table for reproducible builds
// * Defer table generation and store extra type information for lookup

// TypeRefs
// * BTreeSet of namespace+name for lookup but not needed for reproducible builds
// * Defer table generation
// * Defer  referenence resolution but before blobs generation

