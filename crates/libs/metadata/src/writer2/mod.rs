pub fn write(name: &str, references: &super::reader::Reader, types: &[TypeDef]) -> Vec<u8> {
    todo!();
}

pub enum TypeDef {
    Struct(Struct),
}

pub struct Struct {
    pub ident: String,
    pub winrt: bool,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub ident: String,
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
    TypeDef((String, Vec<Self>)),
    MutPtr((Box<Self>, usize)),
    ConstPtr((Box<Self>, usize)),
    Win32Array((Box<Self>, usize)),
    WinrtArray(Box<Self>),
    WinrtArrayRef(Box<Self>),
    WinrtConstRef(Box<Self>),
}
