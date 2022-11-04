mod imp;

pub fn write<P: AsRef<std::path::Path>>(name: &str, defs: &[Item], refs: &[P]) -> Vec<u8> {
    imp::write(name, defs, refs)
}

pub enum Item {
    Struct(Struct),
    Enum(Enum),
}

pub struct Struct {
    pub namespace: String,
    pub name: String,
    pub fields: Vec<Field>,
    pub winrt: bool,
}

pub struct Enum {
    pub namespace: String,
    pub name: String,
    pub constants: Vec<Constant>,
    pub winrt: bool,
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
    TypeDef((String, String)),
}

impl Type {
    pub(crate) fn reference<'a>(&'a self, type_def: &imp::TypeDef, type_ref: &mut imp::TypeRef<'a>) {
        match self {
            Type::TypeDef((namespace, name)) => {
                let name = (namespace.as_str(), name.as_str());
                if !type_def.contains(name) {
                    type_ref.insert(name, imp::ResolutionScope::None);
                }
            }
            _ => {}
        }
    }
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
}
