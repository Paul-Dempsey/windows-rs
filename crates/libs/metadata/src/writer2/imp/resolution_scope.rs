use super::*;

/// A `ResolutionScope` is an index into a certain table indicating the scope in which a TypeRef can be resolved.
#[derive(Default, Clone)]
pub enum ResolutionScope<'a> {
    #[default]
    None,
    Module(&'a str),
    ModuleRef(&'a str),
    AssemblyRef(&'a str),
    TypeRef((&'a str, &'a str)),
}
