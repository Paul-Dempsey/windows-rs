use super::*;

#[derive(Default)]
pub struct Streams {
    pub tables: Vec<u8>,
    pub strings: Vec<u8>,
    pub blobs: Vec<u8>,
    pub guids: Vec<u8>,
}

impl Streams {
    pub fn new(module: &str, items: &std::collections::BTreeMap<TypeName, Item>) -> Self {
        let mut tables = tables::Tables::default();
        let mut strings = Strings::new();
        let mut blobs = Blobs::new();
        tables.Module.push(tables::Module { Name: strings.insert(module), Mvid: 1, ..Default::default() });
        tables.TypeDef.push(tables::TypeDef { TypeName: strings.insert("<Module>"), ..Default::default() });
        let mscorlib = tables.AssemblyRef.push2(tables::AssemblyRef { Name: strings.insert("mscorlib"), MajorVersion: 4, ..Default::default() });
        let value_type = tables.TypeRef.push2(tables::TypeRef { TypeName: strings.insert("ValueType"), TypeNamespace: strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib) });
        //let enum_type = tables.TypeRef.push2(TypeRef { TypeName: strings.insert("Enum"), TypeNamespace: strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib) });

        // TypeDef index for self-referencing. If a type is not in the index it just means it must be a TypeRef (with a null resolution scope).
        let mut item_index = BTreeMap::<&TypeName, usize>::new();

        for (index, type_name) in items.keys().enumerate() {
            item_index.insert(type_name, index);
        }

        for (type_name, item) in items {
            match item {
                Item::Struct(item) => {
                    let mut flags = TypeAttributes::default();
                    flags.set_public();
                    if item.winrt {
                        flags.set_winrt();
                    }
                    tables.TypeDef.push(tables::TypeDef {
                        Flags: flags,
                        TypeName: strings.insert(&type_name.name),
                        TypeNamespace: strings.insert(&type_name.namespace),
                        Extends: TypeDefOrRef::TypeRef(value_type),
                        FieldList: tables.Field.len() as _,
                        MethodList: tables.MethodDef.len() as _,
                    });
                    for field in &item.fields {
                        let mut flags = FieldAttributes::default();
                        flags.set_public();
                        tables.Field.push(tables::Field { Name: strings.insert(&field.name), Flags: flags, Signature: blobs.insert_field_sig(&field.ty) })
                    }
                }
            }
        }

        Self {
            tables: tables.into_stream(),
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
