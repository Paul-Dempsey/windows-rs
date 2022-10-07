mod imp;
use super::imp::*;

pub fn write<P: AsRef<std::path::Path>>(path: P, references: &crate::reader::Reader, items: &[Item]) {
    imp::write(path, references, items)
}

#[derive(Clone)]
pub struct TypeName {
    pub namespace: String,
    pub name: String,
}

impl TypeName {
    pub fn new(namespace: &str, name: &str) -> Self {
        Self { namespace: namespace.to_string(), name: name.to_string() }
    }
}

pub enum Item {
    Struct(Struct),
}

pub struct Struct {
    pub type_name: TypeName,
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

pub enum Value {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    TypeDef(TypeName),
}
