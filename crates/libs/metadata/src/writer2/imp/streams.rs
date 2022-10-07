use super::*;

#[derive(Default)]
pub struct Streams {
    pub tables: Vec<u8>,
    pub strings: Vec<u8>,
    pub blobs: Vec<u8>,
    pub guids: Vec<u8>,
}

impl Streams {
    pub fn new(module: &str, mut items: &std::collections::BTreeMap<TypeName, Item>) -> Self {
        let mut tables = Tables::default();
        let mut strings = Strings::new();
        let mut blobs = Blobs::new();
        tables.Module.push(Module { Name: strings.insert(module), Mvid: 1, ..Default::default() });
        tables.TypeDef.push(TypeDef { TypeName: strings.insert("<Module>"), ..Default::default() });
        let mscorlib = tables.AssemblyRef.push2(AssemblyRef { Name: strings.insert("mscorlib"), MajorVersion: 4, ..Default::default() });
        let value_type = tables.TypeRef.push2(TypeRef { TypeName: strings.insert("ValueType"), TypeNamespace: strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib) });
        let enum_type = tables.TypeRef.push2(TypeRef { TypeName: strings.insert("Enum"), TypeNamespace: strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib) });

        // TODO: need a list of winmd files for reference so we can properly set the ResolutionScope
        // for such TypeRef rows.

        // TODO: also needs a index of TypeDef rows being produced so that subsequent rows can reference those 
        // TypeDef row numbers.

        for (type_name, item) in items {
            match item {
                Item::Struct(item) => {
                    let mut flags = TypeAttributes::default();
                    flags.set_public();
                    if item.winrt {
                        flags.set_winrt();
                    }
                    tables.TypeDef.push(TypeDef {
                        Flags: flags,
                        TypeName: strings.insert(&type_name.name),
                        TypeNamespace: strings.insert(&type_name.namespace),
                        Extends: TypeDefOrRef::TypeRef(value_type),
                        FieldList: tables.Field.len() as _,
                        MethodList: tables.MethodDef.len() as _,
                        ..Default::default()
                    });
                    // for field in item.fields {
                    //     tables.Field.push(Field {
                    //         Name: strings.insert(&field.name),
                    //     })
                    // }
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
