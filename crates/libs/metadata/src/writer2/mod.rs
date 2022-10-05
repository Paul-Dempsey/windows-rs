mod strings;
mod blobs;
mod tables;
mod value;
mod codes;
mod type_name;

use super::*;
use strings::*;
use blobs::*;
use value::*;
use codes::*;
pub use type_name::*;

pub fn write<P: AsRef<std::path::Path>>(path: P, _references: &super::reader::Reader, _types: &[TypeDef]) {
    let _module_name = path.as_ref().file_name().expect("Missing file name").to_str().expect("Invalid file name");

}

pub(crate) fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

pub enum TypeDef {
    Struct(Struct),
}

pub struct Struct {
    pub name: TypeName,
    pub winrt: bool,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: String,
    pub ty: Type,
}

pub enum Type {
    Void,
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    ISize,
    USize,
    String,
    GUID,
    IUnknown,
    IInspectable,
    HRESULT,
    PSTR,
    PWSTR,
    PCSTR,
    PCWSTR,
    BSTR,
    TypeName,
    GenericParam(String),
    TypeDef((TypeName, Vec<Self>)),
    MutPtr((Box<Self>, usize)),
    ConstPtr((Box<Self>, usize)),
    Win32Array((Box<Self>, usize)),
    WinrtArray(Box<Self>),
    WinrtArrayRef(Box<Self>),
    WinrtConstRef(Box<Self>),
}
