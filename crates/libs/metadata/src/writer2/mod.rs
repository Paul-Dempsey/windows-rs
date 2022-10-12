mod imp;
use super::imp::*;

pub fn write<P: AsRef<std::path::Path>>(path: P, winrt: bool, items: &std::collections::BTreeMap<TypeName, Item>) {
    imp::write(path, winrt, items)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    Enum(Enum),
}

pub struct Struct {
    pub fields: Vec<Field>,
}

pub struct Enum {
    pub constants: Vec<Constant>,
}

pub struct Field {
    pub name: String,
    pub ty: Type,
}

impl Field {
    pub fn new(name: &str, ty: Type) -> Self {
        Self { name: name.to_string(), ty }
    }
}

pub struct Constant {
    pub name: String,
    pub value: Value,
}

impl Constant {
    pub fn new(name: &str, value: Value) -> Self {
        Self { name: name.to_string(), value }
    }
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
