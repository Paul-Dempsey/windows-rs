use super::*;

#[derive(Default)]
pub struct Streams {
    pub tables: Vec<u8>,
    pub strings: Vec<u8>,
    pub blobs: Vec<u8>,
    pub guids: Vec<u8>,
}

impl Streams {
    pub fn new(module: &str, winrt: bool, items: &std::collections::BTreeMap<TypeName, Item>) -> Self {
        let mut tables = tables::Tables::default();
        let mut strings = Strings::new();
        let mut blobs = Blobs::new();
        tables.Module.push(tables::Module { Name: module });
        tables.TypeDef.push(tables::TypeDef { Flags: TypeAttributes(0), TypeName: "<Module>", TypeNamespace: "", Extends: TypeDefOrRef::None, FieldList: 0, MethodList: 0 });
        let mscorlib = tables.AssemblyRef.push2(tables::AssemblyRef { MajorVersion: 4, MinorVersion: 0, BuildNumber: 0, RevisionNumber: 0, Flags: AssemblyFlags(0), Name: "mscorlib" });
        let value_type = tables.TypeRef.push2(tables::TypeRef { TypeName: "ValueType", TypeNamespace: "System", ResolutionScope: ResolutionScope::AssemblyRef(mscorlib) });
        let enum_type = tables.TypeRef.push2(tables::TypeRef { TypeName: "Enum", TypeNamespace: "System", ResolutionScope: ResolutionScope::AssemblyRef(mscorlib) });

        // TypeDef index for self-referencing. If a type is not in the index it just means it must be a TypeRef (with a null resolution scope).
        let mut item_index = BTreeMap::<&TypeName, u32>::new();

        for (index, type_name) in items.keys().enumerate() {
            item_index.insert(type_name, index as u32 + 1);
        }

        // TODO: may need TypeRef assembly refs after all including whether the TypeRef is a value type or class type.

        let to_field_sig = |ty: &Type| {
            let mut blob = vec![0x6];
            if let Some(code) = type_to_code(ty) {
                blob.push(code as _);
            } else {
                match ty {
                    Type::TypeDef((type_name, _)) => {
                        // TODO: 0x11 is ELEMENT_TYPE_VALUETYPE - not sure it will work in all cases
                        blob.push(0x11);
                        blob.append(&mut TypeDefOrRef::TypeDef(item_index[type_name]).encode().to_le_bytes().into());
                    }
                    _ => todo!(),
                }
            }
            blob
        };

        for (type_name, item) in items {
            match item {
                Item::Struct(item) => {
                    let mut flags = TypeAttributes(0);
                    flags.set_public();
                    if winrt {
                        flags.set_winrt();
                    }
                    tables.TypeDef.push(tables::TypeDef {
                        Flags: flags,
                        TypeName: &type_name.name,
                        TypeNamespace: &type_name.namespace,
                        Extends: TypeDefOrRef::TypeRef(value_type),
                        FieldList: tables.Field.len() as _,
                        MethodList: 0,
                    });
                    // for field in &item.fields {
                    //     let mut flags = FieldAttributes(0);
                    //     flags.set_public();
                    //     tables.Field.push(tables::Field { Name: &field.name, Flags: flags, Signature: field.ty });
                    // }
                }
                Item::Enum(item) => {
                    let mut flags = TypeAttributes(0);
                    flags.set_public();
                    if winrt {
                        flags.set_winrt();
                    }
                    tables.TypeDef.push(tables::TypeDef {
                        Flags: flags,
                        TypeName: &type_name.name,
                        TypeNamespace: &type_name.namespace,
                        Extends: TypeDefOrRef::TypeRef(enum_type),
                        FieldList: tables.Field.len() as _,
                        MethodList: 0,
                    });
                    // let enum_type = Type::TypeDef((type_name.clone(), Vec::new()));
                    // for constant in &item.constants {
                    //     let mut flags = FieldAttributes(0);
                    //     flags.set_public();
                    //     flags.set_literal();
                    //     flags.set_static();
                    //     tables.Field.push(tables::Field { Name: &constant.name, Flags: flags, Signature: enum_type });
                    //     tables.Constant.push(tables::Constant { Type: value_to_type(&constant.value), Parent: HasConstant::Field(tables.Field.len() as u32), Value: value_to_bytes(&constant.value) });
                    // }
                }
            }
        }

        Self {
            tables: tables.into_stream(&mut strings, &mut blobs),
            strings: strings.into_stream(),
            blobs: blobs.into_stream(),
            guids: vec![0; 16], // zero guid
        }
    }

    pub fn len(&self) -> usize {
        self.tables.len() + self.guids.len() + self.strings.len() + self.blobs.len()
    }
}

pub trait Push2<T> {
    fn push2(&mut self, value: T) -> u32;
}

impl<T> Push2<T> for Vec<T> {
    fn push2(&mut self, value: T) -> u32 {
        self.push(value);
        (self.len() - 1) as _
    }
}

fn value_to_type(value: &Value) -> Type {
    match value {
        Value::Bool(_) => Type::Bool,
        Value::U8(_) => Type::U8,
        Value::I8(_) => Type::I8,
        Value::U16(_) => Type::U16,
        Value::I16(_) => Type::I16,
        Value::U32(_) => Type::U32,
        Value::I32(_) => Type::I32,
        Value::U64(_) => Type::U64,
        Value::I64(_) => Type::I64,
        Value::F32(_) => Type::F32,
        Value::F64(_) => Type::F64,
        Value::String(_) => Type::String,
        Value::TypeDef(value) => Type::TypeDef((value.clone(), Vec::new())),
    }
}

pub fn value_to_bytes(value: &Value) -> Vec<u8> {
    match value {
        Value::U8(value) => value.to_le_bytes().into(),
        Value::I8(value) => value.to_le_bytes().into(),
        Value::U16(value) => value.to_le_bytes().into(),
        Value::I16(value) => value.to_le_bytes().into(),
        Value::U32(value) => value.to_le_bytes().into(),
        Value::I32(value) => value.to_le_bytes().into(),
        Value::U64(value) => value.to_le_bytes().into(),
        Value::I64(value) => value.to_le_bytes().into(),
        Value::F32(value) => value.to_le_bytes().into(),
        Value::F64(value) => value.to_le_bytes().into(),
        _ => unimplemented!(),
    }
}
